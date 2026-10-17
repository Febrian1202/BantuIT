import { NextResponse } from "next/server";
import { formatAppVersion } from "@/lib/version";

export const dynamic = "force-dynamic";

export async function GET() {
  const version = formatAppVersion(
    process.env.NEXT_PUBLIC_APP_VERSION || process.env.APP_VERSION
  );

  return NextResponse.json(
    { version },
    {
      headers: {
        "Cache-Control": "no-store, no-cache, must-revalidate",
      },
    }
  );
}
