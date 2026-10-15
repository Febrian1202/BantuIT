"use client";

import React, { useTransition } from "react";
import { useRouter, useSearchParams, usePathname } from "next/navigation";
import Link from "next/link";
import {
  CheckCheck,
  Ticket,
  AlertTriangle,
  MessageSquare,
  ArrowRight,
  Inbox,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Skeleton } from "@/components/ui/skeleton";
import { RelativeTime } from "@/components/shared/relative-time";
import { EmptyState } from "@/components/shared/empty-state";
import { DataTablePagination } from "@/components/shared/data-table/data-table-pagination";
import { useNotifications } from "@/hooks/use-notifications";
import type { NotificationItem, NotificationType } from "@/types/notifications";
import { cn } from "@/lib/utils";

function getNotificationIcon(type: NotificationType) {
  switch (type) {
    case "TICKET_SLA_BREACHED":
      return (
        <div className="bg-destructive/10 text-destructive flex h-9 w-9 shrink-0 items-center justify-center rounded-full">
          <AlertTriangle className="h-4 w-4" />
        </div>
      );
    case "TICKET_COMMENTED":
      return (
        <div className="bg-primary/10 text-primary flex h-9 w-9 shrink-0 items-center justify-center rounded-full">
          <MessageSquare className="h-4 w-4" />
        </div>
      );
    default:
      return (
        <div className="bg-primary/10 text-primary flex h-9 w-9 shrink-0 items-center justify-center rounded-full">
          <Ticket className="h-4 w-4" />
        </div>
      );
  }
}

export function NotificationsPageClient() {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();
  const [, startTransition] = useTransition();

  const page = Number(searchParams.get("page")) || 1;
  const perPage = Number(searchParams.get("per_page")) || 10;
  const filterTab = searchParams.get("filter") || "all";

  let isReadParam: number | undefined = undefined;
  if (filterTab === "unread") isReadParam = 0;
  if (filterTab === "read") isReadParam = 1;

  const {
    data: response,
    isLoading,
    markAsRead,
    markAllAsRead,
    isMarkingAllRead,
  } = useNotifications({
    page,
    per_page: perPage,
    is_read: isReadParam,
  });

  const notifications = response?.data ?? [];
  const meta = response?.meta;

  const updateQueryParams = (
    updates: Record<string, string | number | null>
  ) => {
    const params = new URLSearchParams(searchParams.toString());
    Object.entries(updates).forEach(([key, val]) => {
      if (val === null || val === undefined || val === "") {
        params.delete(key);
      } else {
        params.set(key, String(val));
      }
    });

    startTransition(() => {
      router.push(`${pathname}?${params.toString()}`);
    });
  };

  return (
    <div className="space-y-6">
      {/* Header with Title & Action */}
      <div className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h2 className="text-foreground text-xl font-bold tracking-tight">
            Pusat Notifikasi
          </h2>
          <p className="text-muted-foreground text-xs sm:text-sm">
            Riwayat pemberitahuan penugasan tiket, perubahan status, dan
            peringatan SLA.
          </p>
        </div>

        <Button
          variant="outline"
          size="sm"
          onClick={() => markAllAsRead()}
          disabled={isMarkingAllRead || isLoading || notifications.length === 0}
          className="gap-1.5 self-start rounded-lg sm:self-auto"
        >
          <CheckCheck className="h-4 w-4" />
          <span>Tandai Semua Dibaca</span>
        </Button>
      </div>

      {/* Filter Tabs */}
      <Tabs
        value={filterTab}
        onValueChange={(val) =>
          updateQueryParams({ filter: val === "all" ? null : val, page: 1 })
        }
        className="w-full"
      >
        <TabsList className="bg-muted/50 p-1">
          <TabsTrigger value="all" className="text-xs">
            Semua
          </TabsTrigger>
          <TabsTrigger value="unread" className="text-xs">
            Belum Dibaca
          </TabsTrigger>
          <TabsTrigger value="read" className="text-xs">
            Sudah Dibaca
          </TabsTrigger>
        </TabsList>
      </Tabs>

      {/* Notifications List */}
      <div className="space-y-2.5">
        {isLoading ? (
          Array.from({ length: 4 }).map((_, idx) => (
            <Card
              key={`skeleton-notif-${idx}`}
              className="border-border bg-card p-4"
            >
              <div className="flex items-start gap-4">
                <Skeleton className="h-9 w-9 shrink-0 rounded-full" />
                <div className="flex-1 space-y-2">
                  <Skeleton className="h-4 w-3/4" />
                  <Skeleton className="h-3 w-1/4" />
                </div>
              </div>
            </Card>
          ))
        ) : notifications.length === 0 ? (
          <EmptyState
            title="Tidak Ada Notifikasi"
            description="Belum ada notifikasi pada kategori ini."
            icon={Inbox}
          />
        ) : (
          notifications.map((item: NotificationItem) => (
            <Card
              key={item.id}
              className={cn(
                "border-border bg-card hover:border-border/80 border transition-colors",
                !item.is_read && "bg-primary/2 border-primary/20"
              )}
            >
              <CardContent className="flex items-start justify-between gap-4 p-4">
                <div className="flex flex-1 items-start gap-3.5">
                  {getNotificationIcon(item.type)}
                  <div className="space-y-1">
                    <p
                      className={cn(
                        "text-foreground text-sm leading-snug",
                        !item.is_read && "font-semibold"
                      )}
                    >
                      {item.data.message}
                    </p>
                    <p className="text-muted-foreground text-xs">
                      <RelativeTime date={item.created_at} />
                    </p>
                  </div>
                </div>

                <div className="flex shrink-0 items-center gap-2">
                  {item.data.ticket_id && (
                    <Button
                      variant="ghost"
                      size="sm"
                      asChild
                      className="h-8 gap-1 text-xs"
                      onClick={() => {
                        if (!item.is_read) {
                          markAsRead(item.id);
                        }
                      }}
                    >
                      <Link href={`/tickets/${item.data.ticket_id}`}>
                        <span>Buka Tiket</span>
                        <ArrowRight className="h-3.5 w-3.5" />
                      </Link>
                    </Button>
                  )}

                  {!item.is_read && (
                    <Button
                      variant="outline"
                      size="icon"
                      className="h-8 w-8 rounded-full"
                      onClick={() => markAsRead(item.id)}
                      aria-label="Tandai telah dibaca"
                      title="Tandai telah dibaca"
                    >
                      <CheckCheck className="h-4 w-4" />
                    </Button>
                  )}
                </div>
              </CardContent>
            </Card>
          ))
        )}
      </div>

      {/* Pagination */}
      {meta && (
        <DataTablePagination
          meta={meta}
          onPageChange={(p) => updateQueryParams({ page: p })}
          onPerPageChange={(pp) => updateQueryParams({ per_page: pp, page: 1 })}
        />
      )}
    </div>
  );
}
