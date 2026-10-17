import { render, screen, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { formatAppVersion } from "@/lib/version";
import { AppSidebar } from "@/components/shell/app-sidebar";
import { MobileNav } from "@/components/shell/mobile-nav";

vi.mock("next/navigation", () => ({
  usePathname: () => "/",
}));

vi.mock("@/components/providers/auth-provider", () => ({
  useAuth: () => ({
    user: {
      id: 1,
      role: { name: "administrator" },
      permissions: ["ticket.viewAny"],
    },
  }),
}));

describe("Application Version Handling", () => {
  const originalEnv = process.env;

  beforeEach(() => {
    vi.resetModules();
    process.env = { ...originalEnv };
  });

  afterEach(() => {
    process.env = originalEnv;
  });

  it("formats version properly with or without leading 'v'", () => {
    expect(formatAppVersion("v1.2.0")).toBe("v1.2.0");
    expect(formatAppVersion("1.2.0")).toBe("v1.2.0");
    expect(formatAppVersion("v2.0.0-rc.1")).toBe("v2.0.0-rc.1");
  });

  it("reads version from NEXT_PUBLIC_APP_VERSION env var if raw is not provided", () => {
    process.env.NEXT_PUBLIC_APP_VERSION = "v1.5.0";
    delete process.env.APP_VERSION;
    expect(formatAppVersion()).toBe("v1.5.0");
  });

  it("reads version from APP_VERSION env var if NEXT_PUBLIC_APP_VERSION is empty", () => {
    delete process.env.NEXT_PUBLIC_APP_VERSION;
    process.env.APP_VERSION = "v1.6.0";
    expect(formatAppVersion()).toBe("v1.6.0");
  });

  it("falls back to v1.0.0 if no version env var is set", () => {
    delete process.env.NEXT_PUBLIC_APP_VERSION;
    delete process.env.APP_VERSION;
    expect(formatAppVersion()).toBe("v1.0.0");
  });

  it("renders custom appVersion prop in AppSidebar", () => {
    render(<AppSidebar appVersion="v2.1.0" />);
    expect(screen.getByText("BantuIT v2.1.0")).toBeInTheDocument();
  });

  it("renders custom appVersion prop in MobileNav", () => {
    render(
      <MobileNav open={true} onOpenChange={vi.fn()} appVersion="v2.1.0" />
    );
    expect(screen.getByText("BantuIT v2.1.0")).toBeInTheDocument();
  });

  it("updates version dynamically when /api/version returns new version", async () => {
    globalThis.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({ version: "v3.0.0" }),
    } as Response);

    render(<AppSidebar appVersion="v1.0.0" />);
    expect(screen.getByText("BantuIT v1.0.0")).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText("BantuIT v3.0.0")).toBeInTheDocument();
    });
  });
});
