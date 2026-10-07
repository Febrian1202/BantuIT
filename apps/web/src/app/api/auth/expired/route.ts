import { deleteToken } from "@/lib/server/session";
import { NextResponse } from "next/server";

// Redirect ke login jika token expired
export async function GET(request: Request) {
  await deleteToken();

  return NextResponse.redirect(new URL("/login", request.url));
}
