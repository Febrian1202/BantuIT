import { NextRequest, NextResponse } from "next/server";

const PUBLIC_PATHS = ["/login", "/favicon.ico"];

export function proxy(request: NextRequest) {
  const hasToken = request.cookies.has("auth_token");
  const { pathname } = request.nextUrl;

  const isPublic = PUBLIC_PATHS.some(
    (path) => pathname === path || pathname.startsWith(path)
  );

  // Jika sudah login dan mencoba mengakses /login, redirect ke /
  if (hasToken && pathname === "/login") {
    return NextResponse.redirect(new URL("/", request.url));
  }

  // Jika belum login dan mencoba mengakses rute yang dilindungi (kecuali API dan path publik)
  if (!hasToken && !isPublic && !pathname.startsWith("/api/")) {
    return NextResponse.redirect(new URL("/login", request.url));

    return NextResponse.next();
  }
}

export const config = {
  matcher: [
    "/((?!_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt).*)",
  ],
};
