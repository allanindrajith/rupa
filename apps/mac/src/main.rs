//! Rupa for macOS.
//!
//! Starts the bundled Next.js standalone server with the bundled Node runtime,
//! then points the native window at it. The port is fixed because the editor
//! stores projects in origin-scoped browser storage (IndexedDB/OPFS): a new
//! port on each launch would hide previously saved projects.

use std::fs::{self, File};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use tauri::{AppHandle, Manager, RunEvent, Url};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 47321;
const START_PAGE: &str = "/projects";
const STARTUP_TIMEOUT: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// The running Node server, killed when the app exits.
struct ServerProcess(Mutex<Option<Child>>);

type BoxError = Box<dyn std::error::Error>;

fn server_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], PORT))
}

fn is_port_open() -> bool {
    TcpStream::connect_timeout(&server_addr(), POLL_INTERVAL).is_ok()
}

fn wait_for_server() -> bool {
    let deadline = Instant::now() + STARTUP_TIMEOUT;
    while Instant::now() < deadline {
        if is_port_open() {
            return true;
        }
        thread::sleep(POLL_INTERVAL);
    }
    false
}

/// Bundled sidecar binaries sit next to the main executable in Contents/MacOS.
fn node_binary() -> Result<PathBuf, BoxError> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().ok_or("executable has no parent directory")?;
    Ok(dir.join("node"))
}

fn spawn_server(server_dir: &Path, log_path: &Path) -> Result<Child, BoxError> {
    let log = File::create(log_path)?;
    let child = Command::new(node_binary()?)
        .arg("apps/web/server.js")
        .current_dir(server_dir)
        .env("NODE_ENV", "production")
        .env("HOSTNAME", HOST)
        .env("PORT", PORT.to_string())
        .env("NEXT_PUBLIC_SITE_URL", format!("http://{HOST}:{PORT}"))
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .spawn()?;
    Ok(child)
}

/// Escapes a message we control into a JS string literal.
fn js_string_literal(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

fn show_status(app: &AppHandle, message: &str) {
    if let Some(window) = app.get_webview_window("main") {
        let script = format!(
            "document.getElementById('status').textContent = {};",
            js_string_literal(message)
        );
        let _ = window.eval(&script);
    }
}

fn open_editor(app: &AppHandle) -> Result<(), BoxError> {
    let url = Url::parse(&format!("http://{HOST}:{PORT}{START_PAGE}"))?;
    let window = app
        .get_webview_window("main")
        .ok_or("main window is missing")?;
    window.navigate(url)?;
    Ok(())
}

fn start(app: &AppHandle) -> Result<(), BoxError> {
    if is_port_open() {
        return Err(format!(
            "Port {PORT} is already in use. Is another copy of Rupa running?"
        )
        .into());
    }

    let server_dir = app.path().resource_dir()?.join("server");
    let log_dir = app.path().app_log_dir()?;
    fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join("server.log");

    let child = spawn_server(&server_dir, &log_path)?;
    app.manage(ServerProcess(Mutex::new(Some(child))));

    let handle = app.clone();
    thread::spawn(move || {
        let result = if wait_for_server() {
            open_editor(&handle)
        } else {
            Err(format!("Rupa's engine did not start. See {}", log_path.display()).into())
        };
        if let Err(err) = result {
            show_status(&handle, &err.to_string());
        }
    });
    Ok(())
}

fn stop_server(app: &AppHandle) {
    let Some(state) = app.try_state::<ServerProcess>() else {
        return;
    };
    let child = match state.0.lock() {
        Ok(mut guard) => guard.take(),
        Err(_) => None,
    };
    if let Some(mut child) = child {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            if let Err(err) = start(&handle) {
                show_status(&handle, &err.to_string());
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("failed to build the Rupa app");

    app.run(|handle, event| {
        if let RunEvent::Exit = event {
            stop_server(handle);
        }
    });
}
