import { screen, render } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { AppSidebar } from "@/components/shell/app-sidebar";
import { MobileNav } from "@/components/shell/mobile-nav";

let currentPath = "/";

vi.mock("next/navigation", () => ({
  usePathname: () => currentPath,
}));

vi.mock("@/components/providers/auth-provider", () => ({
  useAuth: () => ({
    user: {
      id: 1,
      role: { name: "administrator" },
      permissions: ["ticket.viewAny", "user.viewAny"],
    },
  }),
}));

describe("AppSidebar and MobileNav Dashboard Highlight", () => {
  beforeEach(() => {
    currentPath = "/";
  });

  it("highlights Dashboard when pathname is /", () => {
    currentPath = "/";
    render(<AppSidebar />);
    const dashboardLink = screen.getByRole("link", { name: /^Dashboard$/i });
    expect(dashboardLink).toHaveClass("bg-secondary text-foreground font-semibold");
  });

  it("highlights Dashboard when pathname is /dashboard/admin", () => {
    currentPath = "/dashboard/admin";
    render(<AppSidebar />);
    const dashboardLink = screen.getByRole("link", { name: /^Dashboard$/i });
    expect(dashboardLink).toHaveClass("bg-secondary text-foreground font-semibold");
  });

  it("highlights Dashboard in MobileNav when pathname is /dashboard/admin", () => {
    currentPath = "/dashboard/admin";
    render(<MobileNav open={true} onOpenChange={vi.fn()} />);
    const dashboardLink = screen.getByRole("link", { name: /^Dashboard$/i });
    expect(dashboardLink).toHaveClass("bg-secondary text-foreground font-semibold");
  });
});
