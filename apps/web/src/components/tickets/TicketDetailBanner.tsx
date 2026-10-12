"use client";

import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { StatusBadge } from "@/components/shared/status-badge";
import { PriorityBadge } from "@/components/shared/priority-badge";
import { RelativeTime } from "@/components/shared/relative-time";
import { useAuth } from "@/components/providers/auth-provider";
import { getTicketActionLabel } from "@/lib/labels";
import type { TicketDetail, TicketAction } from "@/types/tickets";

export type TicketBannerAction = TicketAction | "delete";

const BANNER_ACTIONS: TicketAction[] = [
  "start",
  "assign",
  "resolve",
  "close",
  "reopen",
  "unassign",
  "cancel",
  "change_priority",
  "edit",
];

interface TicketDetailBannerProps {
  ticket: TicketDetail;
  onAction: (action: TicketBannerAction) => void;
}

export function TicketDetailBanner({
  ticket,
  onAction,
}: TicketDetailBannerProps) {
  const { can } = useAuth();

  const actions = ticket.available_actions.filter((a) =>
    BANNER_ACTIONS.includes(a)
  );

  return (
    <div className="border-border bg-card space-y-4 rounded-xl border p-5 shadow-xs">
      {/* Top row: nomor tiket + badge */}
      <div className="flex flex-wrap items-center gap-2">
        <span className="text-primary font-mono text-sm font-semibold">
          {ticket.ticket_number}
        </span>
        <StatusBadge status={ticket.status.name} />
        <PriorityBadge priority={ticket.priority.name} />
        <Badge
          variant="outline"
          className="bg-muted/50 text-muted-foreground text-xs font-normal"
        >
          {ticket.category.name}
        </Badge>
      </div>

      {/* Judul */}
      <h1 className="text-foreground text-xl leading-snug font-bold tracking-tight">
        {ticket.title}
      </h1>

      {/* Pelapor & metadata row */}
      <div className="text-muted-foreground flex flex-wrap items-center gap-x-4 gap-y-1 text-xs">
        <span>
          Pelapor:{" "}
          <span className="text-foreground font-medium">
            {ticket.reporter.full_name}
          </span>
        </span>
        {ticket.reporter.department && (
          <span>
            Departemen:{" "}
            <span className="text-foreground font-medium">
              {ticket.reporter.department}
            </span>
          </span>
        )}
        {ticket.technician && (
          <span>
            Teknisi:{" "}
            <span className="text-foreground font-medium">
              {ticket.technician.full_name}
            </span>
          </span>
        )}
        <span>
          Dibuat: <RelativeTime date={ticket.created_at} />
        </span>
        <span>
          Komentar:{" "}
          <span className="text-foreground font-medium">
            {ticket.comments_count}
          </span>
        </span>
        <span>
          Lampiran:{" "}
          <span className="text-foreground font-medium">
            {ticket.attachments_count}
          </span>
        </span>
      </div>

      {/* Tombol Aksi */}
      <div className="border-border flex flex-wrap items-center gap-2 border-t pt-1">
        {actions.map((action) => (
          <Button
            key={action}
            size="sm"
            variant="outline"
            className="h-8"
            onClick={() => onAction(action)}
          >
            {getTicketActionLabel(action)}
          </Button>
        ))}

        {/* Tombol Delete — Admin only */}
        {can("ticket.delete") && (
          <Button
            variant="destructive"
            size="sm"
            className="ml-auto h-8"
            onClick={() => onAction("delete")}
          >
            Hapus Tiket
          </Button>
        )}
      </div>
    </div>
  );
}
