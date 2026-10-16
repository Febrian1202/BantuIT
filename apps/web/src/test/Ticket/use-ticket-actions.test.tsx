import { renderHook, act } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useTicketActions } from "@/hooks/use-ticket-actions";
import type { TicketDetail } from "@/types/tickets";

vi.mock("next/navigation", () => ({
  useRouter: () => ({ push: vi.fn(), replace: vi.fn() }),
}));

vi.mock("@/lib/client/api", () => ({
  apiFetch: vi.fn(),
  ApiClientError: class extends Error {},
}));

const mockTicket: TicketDetail = {
  id: 42,
  ticket_number: "TCK-0042",
  title: "Test Ticket",
  description: "Test description",
  status: { id: 1, name: "OPEN" },
  priority: { id: 2, name: "High", sla_minutes: 240 },
  category: { id: 1, name: "Network" },
  reporter: { id: 1, full_name: "User Test", department: "IT" },
  technician: null,
  department: { id: 1, name: "IT" },
  asset: null,
  sla_duration_minutes: 240,
  sla_deadline: "2026-09-03T18:00:00Z",
  sla_breached: false,
  sla_remaining_minutes: 180,
  sla_status: "on_track",
  editable_fields: [],
  available_actions: ["assign", "start"],
  comments_count: 0,
  attachments_count: 0,
  created_at: "2026-09-03T14:00:00Z",
  updated_at: "2026-09-03T14:00:00Z",
  resolved_at: null,
  closed_at: null,
};

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );
  };
}

describe("useTicketActions", () => {
  it("initializes with null activeDialog and activeAction", () => {
    const { result } = renderHook(() => useTicketActions(mockTicket), {
      wrapper: createWrapper(),
    });

    expect(result.current.activeDialog).toBeNull();
    expect(result.current.activeAction).toBeNull();
  });

  it("opens delete dialog when openDialog('delete') is called", () => {
    const { result } = renderHook(() => useTicketActions(mockTicket), {
      wrapper: createWrapper(),
    });

    act(() => {
      result.current.openDialog("delete");
    });

    expect(result.current.activeDialog).toBe("delete");
    expect(result.current.activeAction).toBe("delete");
  });

  it("opens assign dialog when openDialog('assign') is called", () => {
    const { result } = renderHook(() => useTicketActions(mockTicket), {
      wrapper: createWrapper(),
    });

    act(() => {
      result.current.openDialog("assign");
    });

    expect(result.current.activeDialog).toBe("assign");
    expect(result.current.activeAction).toBe("assign");
  });

  it("closes dialog when closeDialog is called", () => {
    const { result } = renderHook(() => useTicketActions(mockTicket), {
      wrapper: createWrapper(),
    });

    act(() => {
      result.current.openDialog("delete");
    });
    expect(result.current.activeDialog).toBe("delete");

    act(() => {
      result.current.closeDialog();
    });
    expect(result.current.activeDialog).toBeNull();
    expect(result.current.activeAction).toBeNull();
  });
});
