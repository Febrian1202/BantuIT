import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { AssetCard } from "@/components/assets/AssetCard";
import type { AssetListItem } from "@/types/assets";

describe("AssetCard", () => {
  const mockAsset: AssetListItem = {
    id: 1,
    asset_tag: "AST-001",
    name: "MacBook Pro 16",
    brand: "Apple",
    model: "M3 Max",
    category: "Laptop",
    status: "assigned",
    serial_number: "C02XYZ123",
    purchase_date: "2024-01-01",
    current_assignment: {
      id: 10,
      user_id: 2,
      full_name: "Budi Santoso",
      assigned_at: "2024-01-10T00:00:00Z",
    },
    created_at: "2024-01-01T00:00:00Z",
    updated_at: "2024-01-01T00:00:00Z",
  };

  it("merender tag asset, badge status , nama, brand/model, dan pemegang", () => {
    render(<AssetCard asset={mockAsset} showHolder={true} />);

    expect(screen.getByText("AST-001")).toBeInTheDocument();
    expect(screen.getByText("MacBook Pro 16")).toBeInTheDocument();
    expect(screen.getByText(/Apple/)).toBeInTheDocument();
    expect(screen.getByText(/M3 Max/)).toBeInTheDocument();
    expect(screen.getByText(/Budi Santoso/)).toBeInTheDocument();
  });

  it("memanggil onClick ketika diklik", () => {
    const handleClick = vi.fn();
    render(<AssetCard asset={mockAsset} onClick={handleClick} />);

    const card = screen.getByRole("button");
    fireEvent.click(card);

    expect(handleClick).toHaveBeenCalledWith(mockAsset);
  });
});
