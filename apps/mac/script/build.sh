#!/usr/bin/env bash
# Builds Rupa.app and Rupa_<version>_<arch>.dmg for this Mac's architecture.
# Output: apps/mac/target/release/bundle/{macos,dmg}/
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
MAC="$ROOT/apps/mac"
WEB="$ROOT/apps/web"
TRIPLE="$(rustc -vV | sed -n 's/^host: //p')"

echo "==> Building the web editor (Next.js standalone, desktop mode)"
# Desktop mode: website pages redirect to /projects and website links are hidden.
(cd "$WEB" && NODE_ENV=production NEXT_PUBLIC_RUPA_DESKTOP=1 bun run build)

echo "==> Assembling server bundle"
rm -rf "$MAC/server-bundle"
cp -R "$WEB/.next/standalone" "$MAC/server-bundle"
# Bun links packages through symlinks into node_modules/.bun, and the app bundler
# drops symlinks. Flatten into a classic hoisted node_modules made of real files.
node - "$MAC/server-bundle" <<'JS'
const fs = require("node:fs");
const path = require("node:path");

const root = process.argv[2];
const modules = path.join(root, "node_modules");
const store = path.join(modules, ".bun");
const flat = path.join(root, "node_modules.flat");

const packageNames = (dir) =>
	fs.readdirSync(dir).flatMap((name) =>
		name.startsWith("@")
			? fs.readdirSync(path.join(dir, name)).map((sub) => `${name}/${sub}`)
			: [name],
	);

fs.mkdirSync(flat);
for (const entry of fs.readdirSync(store)) {
	const dir = path.join(store, entry, "node_modules");
	if (!fs.existsSync(dir)) continue;
	for (const name of packageNames(dir)) {
		const target = path.join(flat, name);
		if (fs.existsSync(target)) continue;
		fs.cpSync(path.join(dir, name), target, { recursive: true, dereference: true });
	}
}
fs.rmSync(modules, { recursive: true });
fs.renameSync(flat, modules);
fs.rmSync(path.join(root, "apps/web/node_modules"), { recursive: true, force: true });
JS
cp -R "$WEB/.next/static" "$MAC/server-bundle/apps/web/.next/static"
cp -R "$WEB/public" "$MAC/server-bundle/apps/web/public"
# Never ship local secrets inside the app; every server env var has a safe default.
find "$MAC/server-bundle" -maxdepth 3 -name ".env*" -delete

echo "==> Bundling Node runtime ($TRIPLE)"
mkdir -p "$MAC/binaries"
cp "$(node -p 'process.execPath')" "$MAC/binaries/node-$TRIPLE"
chmod +x "$MAC/binaries/node-$TRIPLE"

echo "==> Compiling app icon (Icon Composer -> Assets.car)"
rm -rf "$MAC/icon-build"
mkdir -p "$MAC/icon-build"
xcrun actool "$MAC/icons/Rupa.icon" \
  --compile "$MAC/icon-build" \
  --output-partial-info-plist "$MAC/icon-build/partial.plist" \
  --app-icon Rupa --include-all-app-icons \
  --platform macosx --target-device mac --minimum-deployment-target 13.0 \
  --errors --warnings --output-format human-readable-text

echo "==> Building Rupa.app"
(cd "$MAC" && bunx @tauri-apps/cli@2 build)

BUNDLE="$MAC/target/release/bundle"
echo "==> Installing to /Applications/Rupa.app"
# Keep exactly one Rupa.app on this Mac: replace the installed copy, then drop
# the build copy so Launchpad/Spotlight don't list it twice. The DMG remains.
osascript -e 'quit app "Rupa"' >/dev/null 2>&1 || true
rm -rf /Applications/Rupa.app
ditto "$BUNDLE/macos/Rupa.app" /Applications/Rupa.app
rm -rf "$BUNDLE/macos/Rupa.app"
touch /Applications/Rupa.app

echo "==> Done: /Applications/Rupa.app and $(ls "$BUNDLE"/dmg/*.dmg)"
