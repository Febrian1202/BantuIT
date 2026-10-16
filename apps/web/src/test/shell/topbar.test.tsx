import { screen, render } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { AppTopbar, getPageTitle } from "@/components/shell/app-topbar";

let currentPath = "/tickets";

vi.mock("next/navigation", () => ({
  usePathname: () => currentPath,
}));

vi.mock("@/components/shell/notification-bell", () => ({
  NotificationBell: () => <div data-testid="notification-bell" />,
}));

vi.mock("@/components/shell/theme-toggle", () => ({
  ThemeToggle: () => <div data-testid="theme-toggle" />,
}));

vi.mock("@/components/shell/nav-user", () => ({
  NavUser: () => <div data-testid="nav-user" />,
}));

vi.mock("@/components/shell/mobile-nav", () => ({
  MobileNav: () => <div data-testid="mobile-nav" />,
}));

describe("AppTopbar and getPageTitle", () => {
  beforeEach(() => {
    currentPath = "/tickets";
  });

  describe("getPageTitle unit tests", () => {
    it("returns 'Daftar Tiket' for /tickets", () => {
      expect(getPageTitle("/tickets")).toBe("Daftar Tiket");
    });

    it("returns 'Daftar Tiket' for /tickets subroutes like /tickets/new and /tickets/123", () => {
      expect(getPageTitle("/tickets/new")).toBe("Daftar Tiket");
      expect(getPageTitle("/tickets/123")).toBe("Daftar Tiket");
    });

    it("returns correct titles for other pages", () => {
      expect(getPageTitle("/dashboard")).toBe("Dashboard");
      expect(getPageTitle("/assets")).toBe("Inventaris Aset");
      expect(getPageTitle("/knowledge")).toBe("Basis Pengetahuan");
      expect(getPageTitle("/notifications")).toBe("Pusat Notifikasi");
    });
  });

  describe("AppTopbar rendering", () => {
    it("renders 'Daftar Tiket' in the topbar heading when on /tickets", () => {
      currentPath = "/tickets";
      render(<AppTopbar />);
      const heading = screen.getByRole("heading", { level: 1 });
      expect(heading).toHaveTextContent("Daftar Tiket");
    });
  });
});
