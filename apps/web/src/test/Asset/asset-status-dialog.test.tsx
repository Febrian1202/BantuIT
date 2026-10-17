import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeAll } from "vitest";
import { renderWithProviders } from "@/test/test-utils";
import { ChangeStatusDialog } from "@/components/assets/ChangeStatusDialog";
import type { AssetDetail } from "@/types/assets";

beforeAll(() => {
  window.HTMLElement.prototype.scrollIntoView = vi.fn();
  window.HTMLElement.prototype.hasPointerCapture = vi.fn();
  window.HTMLElement.prototype.setPointerCapture = vi.fn();
  window.HTMLElement.prototype.releasePointerCapture = vi.fn();
});

const mockAsset: AssetDetail = {
  id: 1,
  asset_tag: "AST-LTP-001",
  name: "ThinkPad T14",
  category: "Laptop",
  brand: "Lenovo",
  model: "T14 Gen 3",
  serial_number: "SN123456",
  status: "available",
  purchase_date: "2025-01-10",
  notes: null,
  current_assignment: null,
  created_at: "2025-01-10T08:00:00Z",
  updated_at: "2025-01-10T08:00:00Z",
};

describe("ChangeStatusDialog", () => {
  it("renders status options and submit button when open", () => {
    renderWithProviders(
      <ChangeStatusDialog
        open={true}
        onOpenChange={vi.fn()}
        asset={mockAsset}
        isSubmitting={false}
        onSubmit={vi.fn()}
      />
    );

    expect(screen.getByText("Ubah Status Aset")).toBeInTheDocument();
    expect(
      screen.getByText(/ubah status operasional untuk aset/i)
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /simpan status/i })
    ).toBeInTheDocument();
  });

  it("submits selected status and notes when form is submitted", async () => {
    const onSubmit = vi.fn();
    renderWithProviders(
      <ChangeStatusDialog
        open={true}
        onOpenChange={vi.fn()}
        asset={mockAsset}
        isSubmitting={false}
        onSubmit={onSubmit}
      />
    );

    // Open select dropdown
    const trigger = screen.getByRole("combobox", {
      name: /status operasional/i,
    });
    fireEvent.click(trigger);

    // Click maintenance option
    const maintenanceOption = await screen.findByRole("option", {
      name: /dalam pemeliharaan/i,
    });
    fireEvent.click(maintenanceOption);

    // Fill notes
    const notesInput = screen.getByPlaceholderText(
      /alasan atau keterangan perubahan status/i
    );
    fireEvent.change(notesInput, {
      target: { value: "Perangkat perlu ganti keyboard" },
    });

    // Click submit
    const submitBtn = screen.getByRole("button", { name: /simpan status/i });
    fireEvent.click(submitBtn);

    expect(onSubmit).toHaveBeenCalledWith({
      status: "maintenance",
      notes: "Perangkat perlu ganti keyboard",
    });
  });

  it("shows warning when asset is currently assigned", () => {
    const assignedAsset: AssetDetail = {
      ...mockAsset,
      status: "assigned",
      current_assignment: {
        id: 10,
        user_id: 5,
        full_name: "Budi Santoso",
        assigned_at: "2026-01-01T00:00:00Z",
      },
    };

    renderWithProviders(
      <ChangeStatusDialog
        open={true}
        onOpenChange={vi.fn()}
        asset={assignedAsset}
        isSubmitting={false}
        onSubmit={vi.fn()}
      />
    );

    expect(screen.getByText(/sedang ditugaskan kepada/i)).toBeInTheDocument();
  });
});
