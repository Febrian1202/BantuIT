import { screen, fireEvent, waitFor } from "@testing-library/react";
import { describe, it, expect, vi, beforeAll } from "vitest";
import { renderWithProviders } from "@/test/test-utils";
import { UserFormDialog } from "@/components/admin/UserFormDialog";
import type {
  RoleReference,
  DepartmentReference,
  UserListItem,
} from "@/types/auth";

beforeAll(() => {
  window.HTMLElement.prototype.scrollIntoView = vi.fn();
  window.HTMLElement.prototype.hasPointerCapture = vi.fn();
  window.HTMLElement.prototype.setPointerCapture = vi.fn();
  window.HTMLElement.prototype.releasePointerCapture = vi.fn();
});

const mockRoles: RoleReference[] = [
  { id: 1, name: "administrator" },
  { id: 2, name: "manager" },
  { id: 3, name: "technician" },
  { id: 4, name: "employee" },
];

const mockDepartments: DepartmentReference[] = [
  { id: 1, name: "Information Technology" },
  { id: 2, name: "Human Resources" },
];

describe("UserFormDialog", () => {
  it("renders Popover trigger button for hire_date instead of raw HTML date input", () => {
    renderWithProviders(
      <UserFormDialog
        open={true}
        roles={mockRoles}
        departments={mockDepartments}
        onClose={vi.fn()}
      />
    );

    const dateButton = screen.getByRole("button", {
      name: /pilih tanggal bergabung/i,
    });
    expect(dateButton).toBeInTheDocument();
    expect(screen.getByText(/pilih tanggal…/i)).toBeInTheDocument();

    // Raw <input type="date"> must not exist in DOM
    expect(document.querySelector('input[type="date"]')).toBeNull();
  });

  it("displays formatted hire_date when editing a user with hire_date profile", () => {
    const mockUser: UserListItem = {
      id: 5,
      full_name: "Andi Kusuma",
      email: "andi@bantuit.test",
      role: { id: 4, name: "employee" },
      department: { id: 1, name: "Information Technology" },
      employee_code: "EMP-0005",
      status: "active",
      created_at: "2026-01-01T00:00:00Z",
    };

    renderWithProviders(
      <UserFormDialog
        open={true}
        user={mockUser}
        roles={mockRoles}
        departments={mockDepartments}
        onClose={vi.fn()}
      />
    );

    // Dialog title should be Edit Pengguna
    expect(
      screen.getByRole("heading", { name: "Edit Pengguna" })
    ).toBeInTheDocument();
    // Raw date input must still not exist
    expect(document.querySelector('input[type="date"]')).toBeNull();
  });

  it("renders role select items in Indonesian rather than raw English keys", async () => {
    renderWithProviders(
      <UserFormDialog
        open={true}
        roles={mockRoles}
        departments={mockDepartments}
        onClose={vi.fn()}
      />
    );

    // Trigger for role select
    const roleTrigger = screen.getByRole("combobox", { name: /role \*/i });
    expect(roleTrigger).toBeInTheDocument();

    fireEvent.click(roleTrigger);

    // Options should be in Indonesian
    await waitFor(() => {
      expect(
        screen.getByRole("option", { name: "Administrator" })
      ).toBeInTheDocument();
      expect(
        screen.getByRole("option", { name: "Manager" })
      ).toBeInTheDocument();
      expect(
        screen.getByRole("option", { name: "Teknisi" })
      ).toBeInTheDocument();
      expect(
        screen.getByRole("option", { name: "Karyawan" })
      ).toBeInTheDocument();
    });
  });
});
