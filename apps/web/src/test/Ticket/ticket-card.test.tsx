import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { TicketCard } from "@/components/tickets/TicketCard";
import type { TicketListItem } from "@/types/tickets";

const mockPush = vi.fn();
vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe("TicketCard", () => {
  const mockTicket: TicketListItem = {
    id: 1,
    ticket_number: "TKT-2023-001",
    title: "PC tidak bisa menyala",
    sla_status: "on_track",
    sla_breached: false,
    sla_deadline: "2023-12-01T10:00:00Z",
    created_at: "2023-12-01T08:00:00Z",
    status: { id: 1, name: "OPEN" },
    priority: { id: 1, name: "HIGH", sla_minutes: 60 },
    category: { id: 1, name: "Hardware" },
    reporter: { id: 1, full_name: "John Doe" },
    technician: { id: 2, full_name: "Jane Smith" },
  };

  it("merender ticket detail dengan benar", () => {
    render(<TicketCard ticket={mockTicket} />);

    // Cek nomor tiket
    expect(screen.getByText("TKT-2023-001")).toBeInTheDocument();

    // Cek judul
    expect(screen.getByText("PC tidak bisa menyala")).toBeInTheDocument();

    // Cek pelapor dan teknisi
    expect(screen.getByText(/John Doe/)).toBeInTheDocument();
    expect(screen.getByText(/Jane Smith/)).toBeInTheDocument();
  });

  it("navigasi ke ticket detail pada saat klik", () => {
    render(<TicketCard ticket={mockTicket} />);

    const card = screen.getByRole("button");
    fireEvent.click(card);

    expect(mockPush).toHaveBeenCalledWith("/tickets/1");
  });
});
