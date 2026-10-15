import { screen, render, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import LoginPage from "@/app/(auth)/login/page";

const pushMock = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: pushMock,
    replace: vi.fn(),
    refresh: vi.fn(),
  }),
}));

describe("LoginPage Demo Account Quick Fill Cards", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders 4 demo account cards for all system roles", () => {
    render(<LoginPage />);

    expect(
      screen.getByRole("button", { name: /administrator/i })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /manager/i })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /technician|teknisi/i })
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /employee|karyawan/i })
    ).toBeInTheDocument();

    expect(screen.getByText("admin@bantuit.test")).toBeInTheDocument();
    expect(screen.getByText("manager@bantuit.test")).toBeInTheDocument();
    expect(screen.getByText("technician@bantuit.test")).toBeInTheDocument();
    expect(screen.getByText("employee@bantuit.test")).toBeInTheDocument();
  });

  it("fills email and password fields when Administrator card is clicked", () => {
    render(<LoginPage />);

    const emailInput = screen.getByPlaceholderText(
      "nama@perusahaan.co.id"
    ) as HTMLInputElement;
    const passwordInput = screen.getByPlaceholderText(
      "••••••••"
    ) as HTMLInputElement;

    const adminButton = screen.getByRole("button", { name: /administrator/i });
    fireEvent.click(adminButton);

    expect(emailInput.value).toBe("admin@bantuit.test");
    expect(passwordInput.value).toBe("Password123!");
  });

  it("fills email and password fields when Technician card is clicked", () => {
    render(<LoginPage />);

    const emailInput = screen.getByPlaceholderText(
      "nama@perusahaan.co.id"
    ) as HTMLInputElement;
    const passwordInput = screen.getByPlaceholderText(
      "••••••••"
    ) as HTMLInputElement;

    const techButton = screen.getByRole("button", {
      name: /technician|teknisi/i,
    });
    fireEvent.click(techButton);

    expect(emailInput.value).toBe("technician@bantuit.test");
    expect(passwordInput.value).toBe("Password123!");
  });

  it("fills email and password fields when Manager card is clicked", () => {
    render(<LoginPage />);

    const emailInput = screen.getByPlaceholderText(
      "nama@perusahaan.co.id"
    ) as HTMLInputElement;
    const passwordInput = screen.getByPlaceholderText(
      "••••••••"
    ) as HTMLInputElement;

    const managerButton = screen.getByRole("button", { name: /manager/i });
    fireEvent.click(managerButton);

    expect(emailInput.value).toBe("manager@bantuit.test");
    expect(passwordInput.value).toBe("Password123!");
  });

  it("fills email and password fields when Employee card is clicked", () => {
    render(<LoginPage />);

    const emailInput = screen.getByPlaceholderText(
      "nama@perusahaan.co.id"
    ) as HTMLInputElement;
    const passwordInput = screen.getByPlaceholderText(
      "••••••••"
    ) as HTMLInputElement;

    const employeeButton = screen.getByRole("button", {
      name: /employee|karyawan/i,
    });
    fireEvent.click(employeeButton);

    expect(emailInput.value).toBe("employee@bantuit.test");
    expect(passwordInput.value).toBe("Password123!");
  });
});
