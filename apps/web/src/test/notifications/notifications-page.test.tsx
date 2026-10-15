import { screen, render, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { NotificationsPageClient } from "@/app/(app)/notifications/page-client";
import { useNotifications } from "@/hooks/use-notifications";
import type { NotificationItem } from "@/types/notifications";

const mockPush = vi.fn();

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  usePathname: () => "/notifications",
  useSearchParams: () => new URLSearchParams(),
}));

vi.mock("@/hooks/use-notifications", () => ({
  useNotifications: vi.fn(),
}));

const mockNotifications: NotificationItem[] = [
  {
    id: 101,
    type: "TICKET_ASSIGNED",
    data: {
      ticket_id: 42,
      ticket_number: "TCK-20261015-0001",
      message: "Tiket baru #TCK-20261015-0001 ditugaskan kepada Anda.",
    },
    is_read: false,
    read_at: null,
    created_at: new Date().toISOString(),
  },
  {
    id: 102,
    type: "TICKET_SLA_BREACHED",
    data: {
      ticket_id: 43,
      ticket_number: "TCK-20261015-0002",
      message: "Peringatan: SLA tiket #TCK-20261015-0002 telah terlampaui!",
    },
    is_read: true,
    read_at: new Date().toISOString(),
    created_at: new Date().toISOString(),
  },
];

describe("NotificationsPageClient Component", () => {
  const markAsReadMock = vi.fn();
  const markAllAsReadMock = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders header, tabs, and empty state when there are no notifications", () => {
    vi.mocked(useNotifications).mockReturnValue({
      data: {
        success: true,
        message: "OK",
        data: [],
        meta: {
          current_page: 1,
          from: null,
          last_page: 1,
          per_page: 10,
          to: null,
          total: 0,
        },
      },
      isLoading: false,
      markAsRead: markAsReadMock,
      markAllAsRead: markAllAsReadMock,
      isMarkingAllRead: false,
      isMarkingRead: false,
    } as unknown as ReturnType<typeof useNotifications>);

    render(<NotificationsPageClient />);

    expect(
      screen.getByRole("heading", { name: "Pusat Notifikasi" })
    ).toBeInTheDocument();
    expect(
      screen.getByText(/Riwayat pemberitahuan penugasan tiket/i)
    ).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /Tandai Semua Dibaca/i })
    ).toBeDisabled();
    expect(screen.getByText("Tidak Ada Notifikasi")).toBeInTheDocument();
  });

  it("renders notification items with actions and handles mark as read", () => {
    vi.mocked(useNotifications).mockReturnValue({
      data: {
        success: true,
        message: "OK",
        data: mockNotifications,
        meta: {
          current_page: 1,
          from: 1,
          last_page: 1,
          per_page: 10,
          to: 2,
          total: 2,
        },
      },
      isLoading: false,
      markAsRead: markAsReadMock,
      markAllAsRead: markAllAsReadMock,
      isMarkingAllRead: false,
      isMarkingRead: false,
    } as unknown as ReturnType<typeof useNotifications>);

    render(<NotificationsPageClient />);

    // Check notification messages
    expect(
      screen.getByText("Tiket baru #TCK-20261015-0001 ditugaskan kepada Anda.")
    ).toBeInTheDocument();
    expect(
      screen.getByText(
        "Peringatan: SLA tiket #TCK-20261015-0002 telah terlampaui!"
      )
    ).toBeInTheDocument();

    // Mark all as read button should be enabled
    const markAllBtn = screen.getByRole("button", {
      name: /Tandai Semua Dibaca/i,
    });
    expect(markAllBtn).not.toBeDisabled();
    fireEvent.click(markAllBtn);
    expect(markAllAsReadMock).toHaveBeenCalledTimes(1);

    // Unread item has single check button
    const singleMarkBtn = screen.getByRole("button", {
      name: "Tandai telah dibaca",
    });
    fireEvent.click(singleMarkBtn);
    expect(markAsReadMock).toHaveBeenCalledWith(101);

    // Open ticket links
    const ticketLinks = screen.getAllByRole("link", { name: /Buka Tiket/i });
    expect(ticketLinks).toHaveLength(2);
    expect(ticketLinks[0]).toHaveAttribute("href", "/tickets/42");
    expect(ticketLinks[1]).toHaveAttribute("href", "/tickets/43");

    // Clicking unread ticket link calls markAsRead
    fireEvent.click(ticketLinks[0]);
    expect(markAsReadMock).toHaveBeenCalledWith(101);
  });
});
