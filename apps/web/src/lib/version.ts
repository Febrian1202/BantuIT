export function formatAppVersion(raw?: string | null): string {
  const version =
    raw ||
    process.env.NEXT_PUBLIC_APP_VERSION ||
    process.env.APP_VERSION ||
    "v1.0.0";
  return version.startsWith("v") ? version : `v${version}`;
}
