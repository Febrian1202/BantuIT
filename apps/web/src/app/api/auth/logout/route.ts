import { laravelFetch } from "@/lib/server/api";
import { deleteToken } from "@/lib/server/session";
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

  return NextResponse.json({
    success: true,
    message: "Logout successful.",
    data: null,
  });
}
