import { laravelFetch } from "@/lib/server/api";
import { deleteToken, AUTH_COOKIE_NAME } from "@/lib/server/session";
import { NextResponse } from "next/server";

// Logout endpoint
export async function POST() {
  try {
    await laravelFetch("/logout", { method: "POST" });
  } catch {
    // Backend revoke gagal; token masih valid
  }

  try {
    await deleteToken();
  } catch {
    // Token delete gagal
  }

  const response = NextResponse.json({
    success: true,
    message: "Logout successful.",
    data: null,
  });

  response.cookies.set(AUTH_COOKIE_NAME, "", {
    httpOnly: true,
    secure: process.env.NODE_ENV === "production",
    sameSite: "lax",
    path: "/",
    maxAge: 0,
    expires: new Date(0),
  });

  return response;
}
