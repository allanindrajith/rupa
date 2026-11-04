import { type NextRequest, NextResponse } from "next/server";
import {
	DESKTOP_HOME_PATH,
	IS_DESKTOP_APP,
	isDesktopAppPath,
} from "@/lib/site/app-mode";

/** In the desktop app, keep users inside the editor: website pages redirect to projects. */
export function proxy(request: NextRequest) {
	if (!IS_DESKTOP_APP || isDesktopAppPath(request.nextUrl.pathname)) {
		return NextResponse.next();
	}
	return NextResponse.redirect(new URL(DESKTOP_HOME_PATH, request.url));
}

export const config = {
	// Pages only: skip API routes, Next internals, and static files (anything with an extension).
	matcher: ["/((?!api/|_next/|.*\\..*).*)"],
};
