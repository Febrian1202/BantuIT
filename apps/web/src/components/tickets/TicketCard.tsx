"use client";

import { useRouter } from "next/navigation";
import { StatusBadge } from "@/components/shared/status-badge";
import { PriorityBadge } from "@/components/shared/priority-badge";
import { SlaIndicator } from "@/components/shared/sla-indicator";
import { RelativeTime } from "@/components/shared/relative-time";
import type { TicketListItem } from "@/types/tickets";

interface TicketCardProps {
  ticket: TicketListItem;
}

export function TicketCard({ ticket }: TicketCardProps) {
  const router = useRouter();

  return (
    <div
      role="button"
      tabIndex={0}
      aria-label={`Tiket ${ticket.ticket_number}: ${ticket.title}`}
      onClick={() => router.push(`/tickets/${ticket.id}`)}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          router.push(`/tickets/${ticket.id}`);
        }
      }}
      className="border-border bg-card hover:bg-muted/50 flex min-h-11 cursor-pointer flex-col gap-3 border-b p-4 transition-colors"
    >
      <div className="flex items-start justify-between gap-2">
        <span className="text-primary font-mono text-xs font-semibold">
          {ticket.ticket_number}
        </span>
        <div className="flex flex-wrap items-center justify-end gap-1.5">
          <StatusBadge status={ticket.status?.name} />
          <PriorityBadge priority={ticket.priority?.name} />
        </div>
      </div>

      <div>
        <h3 className="text-foreground line-clamp-2 text-sm font-medium">
          {ticket.title}
        </h3>
        <p className="text-muted-foreground mt-1 text-xs">
          {ticket.category?.name ?? "Tidak ada kategori"}
        </p>
      </div>

      <div className="mt-1 flex flex-col gap-2">
        <div className="text-muted-foreground flex items-center gap-2 text-xs">
          <span className="flex-1 truncate" title={ticket.reporter?.full_name}>
            P: {ticket.reporter?.full_name ?? "-"}
          </span>
          <span
            className="flex-1 truncate"
            title={ticket.technician?.full_name}
          >
            T: {ticket.technician?.full_name ?? "-"}
          </span>
          <div className="shrink-0 whitespace-nowrap">
            <RelativeTime date={ticket.created_at} />
          </div>
        </div>

        <SlaIndicator
          slaStatus={ticket.sla_status}
          durationMinutes={ticket.priority?.sla_minutes}
          deadline={ticket.sla_deadline}
        />
      </div>
    </div>
  );
}
