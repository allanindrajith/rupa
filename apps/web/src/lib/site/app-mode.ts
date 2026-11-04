/**
 * True when this build is bundled inside the Rupa desktop app (apps/mac).
 * The desktop app is only the editor: website pages are redirected away and
 * links back to the website are hidden. Inlined at build time.
 */
export const IS_DESKTOP_APP = process.env.NEXT_PUBLIC_RUPA_DESKTOP === "1";

/** Where the desktop app lands, and where website routes are redirected to. */
export const DESKTOP_HOME_PATH = "/projects";

/** Pages that exist in the desktop app. Everything else is website-only. */
export function isDesktopAppPath(pathname: string): boolean {
	return (
		pathname === DESKTOP_HOME_PATH ||
		pathname === "/editor" ||
		pathname.startsWith("/editor/")
	);
}
