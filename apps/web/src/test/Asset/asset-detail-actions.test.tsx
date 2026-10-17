import { screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeAll } from "vitest";
import { renderWithProviders } from "@/test/test-utils";
import { AssetDetailPageClient } from "@/app/(app)/assets/[id]/page-client";
import type { AssetDetail } from "@/types/assets";

beforeAll(() => {
  window.HTMLElement.prototype.scrollIntoView = vi.fn();
  window.HTMLElement.prototype.hasPointerCapture = vi.fn();
  window.HTMLElement.prototype.setPointerCapture = vi.fn();
  window.HTMLElement.prototype.releasePointerCapture = vi.fn();
});

const mockAsset: AssetDetail = {
  id: 42,
  asset_tag: "AST-LTP-042",
  name: "MacBook Pro M2",
  category: "Laptop",
  brand: "Apple",
  model: "MacBook Pro 14",
  serial_number: "C02XYZ123",
  status: "available",
  purchase_date: "2025-01-15",
  notes: null,
  current_assignment: null,
  created_at: "2025-01-15T08:00:00Z",
  updated_at: "2025-01-15T08:00:00Z",
};

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: vi.fn(), replace: vi.fn() }),
  usePathname: () => "/assets/42",
}));

let mockCan: (permission: string) => boolean = () => true;

vi.mock("@/components/providers/auth-provider", () => ({
  useAuth: () => ({
    can: (p: string) => mockCan(p),
  }),
}));

vi.mock("@/hooks/use-assets", () => ({
  useAsset: () => ({
    data: { data: mockAsset },
    isLoading: false,
    error: null,
  }),
  useAssetHistory: () => ({
    data: { data: [] },
    isLoading: false,
  }),
}));

describe("AssetDetailPageClient Actions", () => {
  it("renders 'Ubah Aset' link button when user has asset.update permission", () => {
    mockCan = (p: string) => ["asset.viewAny", "asset.update"].includes(p);
    renderWithProviders(<AssetDetailPageClient assetId={42} />);

    const editBtn = screen.getByRole("link", { name: /ubah aset/i });
    expect(editBtn).toBeInTheDocument();
    expect(editBtn).toHaveAttribute("href", "/assets/42/edit");
  });

  it("renders 'Ubah Status' button and opens ChangeStatusDialog", () => {
    mockCan = (p: string) => ["asset.viewAny", "asset.update"].includes(p);
    renderWithProviders(<AssetDetailPageClient assetId={42} />);

    const statusBtn = screen.getByRole("button", { name: /ubah status/i });
    expect(statusBtn).toBeInTheDocument();

    fireEvent.click(statusBtn);

    expect(
      screen.getByRole("heading", { name: "Ubah Status Aset" })
    ).toBeInTheDocument();
  });
});
