"use client";

import { MessageSquare, History } from "lucide-react";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Separator } from "@/components/ui/separator";
import { RelativeTime } from "@/components/shared/relative-time";
import { cn } from "@/lib/utils";
import type { TimelineEntry } from "@/lib/timeline-merge";

interface TicketTimelineProps {
  entries: TimelineEntry[];
  isPending?: boolean;
  emptyText?: string;
}

export function TicketTimeline({
  entries,
  isPending = false,
  emptyText = "Belum ada komentar atau riwayat penanganan.",
}: TicketTimelineProps) {
  if (isPending) {
    return (
      <div className="space-y-4 py-2">
        {[1, 2, 3].map((i) => (
          <div key={i} className="flex animate-pulse gap-3">
            <div className="bg-muted h-8 w-8 shrink-0 rounded-full" />
            <div className="flex-1 space-y-2">
              <div className="bg-muted h-3 w-1/3 rounded" />
              <div className="bg-muted h-3 w-2/3 rounded" />
            </div>
          </div>
        ))}
      </div>
    );
  }

  if (entries.length === 0) {
    return <p className="text-muted-foreground py-4 text-sm">{emptyText}</p>;
  }

  return (
    <ol className="space-y-0">
      {entries.map((entry, idx) => {
        const isLast = idx === entries.length - 1;
        const initials = (entry.user?.full_name ?? "?")
          .split(" ")
          .map((w) => w[0])
          .slice(0, 2)
          .join("")
          .toUpperCase();

        return (
          <li
            key={`${entry.type}-${entry.id}`}
            className="relative flex gap-3 pb-5"
          >
            {/* Vertical line */}
            {!isLast && (
              <span
                aria-hidden
                className="bg-border absolute top-9 bottom-0 left-4 w-px"
              />
            )}

            {/* Avatar / icon */}
            {entry.type === "comment" ? (
              <Avatar className="ring-border bg-muted h-8 w-8 shrink-0 text-[11px] ring-1">
                <AvatarFallback>{initials || "?"}</AvatarFallback>
              </Avatar>
            ) : (
              <div className="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-[#e8edf2] text-[#42526e] ring-1 ring-[#d8e0ea]">
                <History className="h-3.5 w-3.5" />
              </div>
            )}

            {/* Content */}
            <div className="min-w-0 flex-1 pt-1">
              {entry.type === "comment" ? (
                <>
                  <div className="flex items-baseline justify-between gap-2">
                    <p className="text-foreground text-xs font-semibold">
                      {entry.user?.full_name ?? "Pengguna"}
                    </p>
                    <RelativeTime
                      date={entry.created_at}
                      className="text-muted-foreground shrink-0 text-[11px]"
                    />
                  </div>
                  <p className="border-border bg-muted/30 text-foreground mt-1 rounded-lg rounded-tl-none border px-3 py-2 text-sm leading-relaxed wrap-break-word whitespace-pre-wrap">
                    {entry.body}
                  </p>
                </>
              ) : (
                <div className="flex items-baseline gap-2">
                  <p className="text-foreground text-sm leading-relaxed">
                    {entry.description}
                  </p>
                  <span className="text-muted-foreground shrink-0 text-[11px]">
                    • <RelativeTime date={entry.created_at} />
                  </span>
                </div>
              )}
            </div>
          </li>
        );
      })}
    </ol>
  );
}

export function TimelineHeader() {
  return (
    <div className="flex items-center gap-2">
      <MessageSquare className="text-primary h-4 w-4" />
      <h2 className="text-foreground text-sm font-semibold">
        Riwayat Penanganan &amp; Diskusi
      </h2>
    </div>
  );
}

export function TimelineDivider() {
  return <Separator />;
}

export function getTimelineIconClass(entry: TimelineEntry) {
  return cn(
    entry.type === "comment"
      ? "bg-muted text-muted-foreground"
      : "bg-[#e8edf2] text-[#42526e]"
  );
}
