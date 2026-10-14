import { ReactNode } from "react";
import Link from "next/link";
import type { LucideIcon } from "lucide-react";
import { EmptyState } from "@/components/shared/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";

export interface DashboardPanelProps {
  title: string;
  actionLabel?: string;
  actionHref?: string;
  actionIcon?: LucideIcon;
  isLoading?: boolean;
  isEmpty?: boolean;
  emptyTitle?: string;
  emptyMessage?: string;
  children: ReactNode;
  className?: string;
}

export function DashboardPanel({
  title,
  actionLabel,
  actionHref,
  actionIcon: ActionIcon,
  isLoading = false,
  isEmpty = false,
  emptyTitle = "Tidak Ada Data",
  emptyMessage = "Belum ada data untuk ditampilkan saat ini.",
  children,
  className,
}: DashboardPanelProps) {
  return (
    <div
      className={cn(
        "rounded-card border-border bg-card flex flex-col border p-5 shadow-xs",
        className
      )}
    >
      <div className="border-border mb-4 flex items-center justify-between gap-4 border-b pb-4">
        <h3 className="text-foreground text-sm font-semibold tracking-tight sm:text-base">
          {title}
        </h3>
        {actionHref && actionLabel && (
          <Link
            href={actionHref}
            className="text-muted-foreground hover:text-foreground inline-flex items-center gap-1.5 text-xs font-medium transition-colors"
          >
            <span>{actionLabel}</span>
            {ActionIcon && (
              <ActionIcon className="h-3.5 w-3.5" strokeWidth={1.5} />
            )}
          </Link>
        )}
      </div>

      <div className="flex-1">
        {isLoading ? (
          <div className="space-y-3 py-1">
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
          </div>
        ) : isEmpty ? (
          <EmptyState title={emptyTitle} description={emptyMessage} />
        ) : (
          children
        )}
      </div>
    </div>
  );
}
