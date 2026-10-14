import { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";

export interface MetricCardProps {
  label: string;
  value: string | number | null;
  icon?: LucideIcon;
  footer?: ReactNode;
  tone?: "default" | "danger";
  isLoading?: boolean;
  className?: string;
}

export function MetricCard({
  label,
  value,
  icon: Icon,
  footer,
  tone = "default",
  isLoading = false,
  className,
}: MetricCardProps) {
  const isDanger = tone === "danger";

  return (
    <div
      className={cn(
        "rounded-card border-border bg-card flex flex-col justify-between border p-3 shadow-xs sm:p-4",
        isDanger &&
          "dark:border-border-red-900/40 dark:bg-card border-[#fae8e8]/80 bg-[#fdf2f2]/60 dark:border-red-900/50",
        className
      )}
    >
      <div className="flex items-center justify-between gap-2">
        <span
          className={cn(
            "truncate text-xs font-medium",
            isDanger
              ? "text-destructive dark:text-red-300"
              : "text-muted-foreground"
          )}
        >
          {label}
        </span>
        {Icon && (
          <div
            className={cn(
              "flex h-7 w-7 shrink-0 items-center justify-center rounded-full sm:h-8 sm:w-8",
              isDanger
                ? "bg-[#fae8e8] text-[#991b1b] dark:border dark:border-red-800/40 dark:bg-red-950/70 dark:text-red-400"
                : "bg-muted/60 text-muted-foreground"
            )}
          >
            <Icon className="h-3.5 w-3.5 sm:h-4 sm:w-4" strokeWidth={1.5} />
          </div>
        )}
      </div>

      <div className="mt-3">
        {isLoading ? (
          <Skeleton className="h-6 w-16 sm:h-8" />
        ) : (
          <div
            className={cn(
              "text-2xl text-lg font-semibold tracking-tight sm:text-2xl",
              isDanger ? "text-[#991b1b] dark:text-red-400" : "text-foreground"
            )}
          >
            {value === null || value === undefined ? "—" : value}
          </div>
        )}
      </div>

      {footer && !isLoading && (
        <div
          className={cn(
            "mt-1.5 text-[11px] leading-tight sm:mt-2 sm:text-xs",
            isDanger
              ? "text-destructive/90 font-medium dark:text-red-300/90"
              : "text-muted-foreground"
          )}
        >
          {footer}
        </div>
      )}
    </div>
  );
}
