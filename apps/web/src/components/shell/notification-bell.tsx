import { useNotificationsPoll } from "@/hooks/use-notifications.poll";
import { apiFetch } from "@/lib/client/api";
import { notificationKeys } from "@/lib/query-key";
import { NotificationItem, NotificationType } from "@/types/notifications";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  AlertTriangle,
  ArrowRight,
  Bell,
  CheckCheck,
  MessageSquare,
  Ticket,
} from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Button } from "@/components/ui/button";
import Link from "next/link";
import { cn } from "@/lib/utils";
import { RelativeTime } from "../shared/relative-time";

function getNotificationIcon(type: NotificationType) {
  switch (type) {
    case "TICKET_SLA_BREACHED":
      return <AlertTriangle className="text-destructive h-4 w-4 shrink-0" />;
    case "TICKET_COMMENTED":
      return <MessageSquare className="text-primary h-4 w-4 shrink-0" />;
    default:
      return <Ticket className="text-primary h-4 w-4 shrink-0" />;
  }
}

export function NotificationBell() {
  const queryClient = useQueryClient();
  const { data: pollData } = useNotificationsPoll();
  const unreadCount = pollData?.unread_count ?? 0;

  // Fetch top 5 notif terbaru
  const { data: listResponse, isLoading } = useQuery({
    queryKey: notificationKeys.list({ per_page: 5 }),
    queryFn: async () => {
      const res = await apiFetch<NotificationItem[]>(
        "/notifications?per_page=5"
      );
      return res.data;
    },
    staleTime: 10 * 1000,
  });

  const notifications = listResponse ?? [];

  const markAsReadMutation = useMutation({
    mutationFn: async (id: number) => {
      await apiFetch<null>(`/notifications/${id}/read`, { method: "POST" });
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: notificationKeys.all });
    },
  });

  const markAllAsReadMutation = useMutation({
    mutationFn: async () => {
      await apiFetch<null>("/notifications/read-all", { method: "POST" });
    },
    onSuccess: () => {
      void queryClient.invalidateQueries({ queryKey: notificationKeys.all });
    },
  });

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="ghost"
          size="icon"
          className="text-muted-foreground hover:text-foreground relative rounded-lg"
          aria-label={`Buka menu notifikasi (${unreadCount} belum dibaca)`}
        >
          <Bell className="h-5 w-5" />
          {unreadCount > 0 && (
            <span className="bg-destructive text-destructive-foreground absolute -top-1 -right-1 flex h-4 min-w-4 items-center justify-center rounded-full px-1 text-[10px] font-bold">
              {unreadCount > 99 ? "99+" : unreadCount}
            </span>
          )}
        </Button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        className="border-border bg-card w-80 border p-0 shadow-lg sm:w-96"
        align="end"
      >
        {/* Header */}
        <div className="border-border bg-muted/20 flex items-center justify-between border-b p-3">
          <div className="flex items-center gap-1.5">
            <span className="text-foreground text-sm font-semibold">
              Notifikasi
            </span>
            {unreadCount > 0 && (
              <span className="bg-destructive/10 text-destructive rounded-full px-1.5 py-0.5 text-[10px] font-bold">
                {unreadCount} baru
              </span>
            )}
          </div>

          {unreadCount > 0 && (
            <Button
              variant="ghost"
              size="sm"
              onClick={() => markAllAsReadMutation.mutate()}
              disabled={markAllAsReadMutation.isPending}
              className="text-muted-foreground hover:text-foreground h-7 px-2 text-[11px]"
            >
              <CheckCheck className="mr-1 h-3 w-3" />
              Tandai semua dibaca
            </Button>
          )}
        </div>

        {/* Notification List */}
        <div className="divide-border/60 max-h-80 divide-y overflow-y-auto">
          {isLoading ? (
            <div className="text-muted-foreground p-6 text-center text-xs">
              Memuat notifikasi…
            </div>
          ) : notifications.length === 0 ? (
            <div className="text-muted-foreground p-6 text-center text-xs">
              Tidak ada notifikasi untuk ditampilkan.
            </div>
          ) : (
            notifications.map((item) => (
              <DropdownMenuItem
                key={item.id}
                asChild
                className={cn(
                  "focus:bg-muted/40 flex cursor-pointer items-start gap-3 p-3 transition-colors",
                  !item.is_read && "bg-primary/3"
                )}
                onClick={() => {
                  if (!item.is_read) {
                    markAsReadMutation.mutate(item.id);
                  }
                }}
              >
                <Link
                  href={
                    item.data.ticket_id
                      ? `/tickets/${item.data.ticket_id}`
                      : "/notifications"
                  }
                  className="w-full"
                >
                  <div className="mt-0.5">{getNotificationIcon(item.type)}</div>
                  <div className="flex-1 space-y-1 overflow-hidden">
                    <p
                      className={cn(
                        "text-foreground line-clamp-2 text-xs",
                        !item.is_read && "font-medium"
                      )}
                    >
                      {item.data.message}
                    </p>
                    <p className="text-muted-foreground text-[10px]">
                      <RelativeTime date={item.created_at} />
                    </p>
                  </div>
                  {!item.is_read && (
                    <span className="bg-primary mt-1.5 h-2 w-2 shrink-0 rounded-full" />
                  )}
                </Link>
              </DropdownMenuItem>
            ))
          )}
        </div>

        {/* Footer */}
        <DropdownMenuSeparator className="m-0" />
        <div className="bg-muted/10 p-2 text-center">
          <Button
            variant="ghost"
            size="sm"
            asChild
            className="h-8 w-full text-xs font-medium"
          >
            <Link href="/notifications">
              <span>Lihat Semua Notifikasi</span>
              <ArrowRight className="ml-1 h-3.5 w-3.5" />
            </Link>
          </Button>
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
