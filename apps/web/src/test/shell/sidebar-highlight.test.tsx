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
    expect(dashboardLink).toHaveClass(
      "bg-secondary text-foreground font-semibold"
    );
  });

  it("highlights Dashboard when pathname is /dashboard/admin", () => {
    currentPath = "/dashboard/admin";
    render(<AppSidebar />);
    const dashboardLink = screen.getByRole("link", { name: /^Dashboard$/i });
    expect(dashboardLink).toHaveClass(
      "bg-secondary text-foreground font-semibold"
    );
  });

  it("renders BantuIT brand text with bold and enlarged typography in AppSidebar", () => {
    render(<AppSidebar />);
    const brandLink = screen.getByRole("link", { name: /^BantuIT$/i });
    expect(brandLink).toHaveClass(
      "text-xl font-bold tracking-tight text-foreground"
    );
  });

  it("renders BantuIT brand text with bold and enlarged typography in MobileNav", () => {
    render(<MobileNav open={true} onOpenChange={vi.fn()} />);
    const brandText = screen.getByText(/^BantuIT$/i);
    expect(brandText).toHaveClass(
      "text-xl font-bold tracking-tight text-foreground"
    );
  });
});
