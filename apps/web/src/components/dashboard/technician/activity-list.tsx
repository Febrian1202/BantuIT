import Link from "next/link";
import {
  Activity,
  ArrowRightLeft,
  UserCheck,
  Tag,
  SlidersHorizontal,
} from "lucide-react";
import { RelativeTime } from "@/components/shared/relative-time";
import { EmptyState } from "@/components/shared/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import { getActivityFieldLabel } from "@/lib/labels";
import type { TicketHistoryItem } from "@/types/tickets";

interface TechnicianActivityListProps {
  activities: TicketHistoryItem[];
  isLoading?: boolean;
}

function getActivityIcon(fieldChanged: string) {
  switch (fieldChanged) {
    case "status_id":
      return ArrowRightLeft;
    case "technician_id":
      return UserCheck;
    case "category_id":
      return Tag;
    case "priority_id":
      return SlidersHorizontal;
    default:
      return Activity;
  }
}

export function TechnicianActivityList({
  activities,
  isLoading = false,
}: TechnicianActivityListProps) {
  if (isLoading) {
    return (
      <div className="space-y-3 py-2">
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
        <Skeleton className="h-12 w-full" />
      </div>
    );
  }

  if (activities.length === 0) {
    return (
      <EmptyState
        title="Tidak Ada Aktivitas"
        description="Belum ada aktivitas."
      />
    );
  }

  return (
    <div className="divide-border divide-y">
      {activities.map((item) => {
        const Icon = getActivityIcon(item.field_changed);
        const actionLabel = getActivityFieldLabel(item.field_changed);

        return (
          <div
            key={item.id}
            className="hover:bg-muted/50 flex items-center justify-between rounded-sm px-1 py-3.5 transition-colors"
          >
            <div className="flex min-w-0 items-center gap-3">
              <div className="bg-muted/50 text-muted-foreground flex h-8 w-8 shrink-0 items-center justify-center rounded-lg">
                <Icon className="h-4 w-4" strokeWidth={1.5} />
              </div>
              <div className="min-w-0 space-y-0.5">
                <div className="text-foreground flex flex-wrap items-center gap-1.5 text-xs font-medium">
                  <span>{actionLabel}</span>
                  {item.ticket ? (
                    <Link
                      href={`/tickets?search=${encodeURIComponent(item.ticket.ticket_number)}`}
                      className="text-primary font-mono text-xs font-semibold hover:underline"
                    >
                      {item.ticket.ticket_number}
                    </Link>
                  ) : null}
                  {item.ticket?.title && (
                    <span className="text-muted-foreground max-w-50 truncate sm:max-w-xs">
                      — {item.ticket.title}
                    </span>
                  )}
                </div>
                {(item.old_value || item.new_value) && (
                  <div className="text-muted-foreground text-[11px]">
                    {item.old_value && (
                      <span className="line-through">{item.old_value}</span>
                    )}
                    {item.old_value && item.new_value && <span> → </span>}
                    {item.new_value && (
                      <span className="text-foreground font-medium">
                        {item.new_value}
                      </span>
                    )}
                  </div>
                )}
              </div>
            </div>
            <div className="text-muted-foreground shrink-0 pl-2 text-[11px]">
              <RelativeTime date={item.created_at} />
            </div>
          </div>
        );
      })}
    </div>
  );
}
