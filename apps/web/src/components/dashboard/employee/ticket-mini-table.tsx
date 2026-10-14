import React from "react";
import { useRouter } from "next/navigation";
import { StatusBadge } from "@/components/shared/status-badge";
import { PriorityBadge } from "@/components/shared/priority-badge";
import { RelativeTime } from "@/components/shared/relative-time";
import { EmptyState } from "@/components/shared/empty-state";
import { Skeleton } from "@/components/ui/skeleton";
import { formatSlaRemaining } from "@/lib/formatters";
import type { TicketListItem } from "@/types/tickets";

interface TicketMiniTableProps {
  tickets: TicketListItem[];
  isLoading?: boolean;
}

export function TicketMiniTable({
  tickets,
  isLoading = false,
}: TicketMiniTableProps) {
  const router = useRouter();
  const [now] = React.useState(() => Date.now());

  if (isLoading) {
    return (
      <div className="space-y-3 py-2">
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-10 w-full" />
        <Skeleton className="h-10 w-full" />
      </div>
    );
  }

  if (tickets.length === 0) {
    return (
      <EmptyState title="Tidak Ada Tiket" description="Belum ada tiket." />
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse text-left text-sm">
        <caption className="sr-only">
          Daftar ringkasan tiket aktif terbaru milik karyawan
        </caption>
        <thead>
          <tr className="border-border text-muted-foreground border-b text-xs">
            <th scope="col" className="pt-1 pb-3 font-medium">
              NOMOR
            </th>
            <th scope="col" className="pt-1 pb-3 font-medium">
              JUDUL
            </th>
            <th scope="col" className="pt-1 pb-3 font-medium">
              STATUS
            </th>
            <th
              scope="col"
              className="hidden pt-1 pb-3 font-medium sm:table-cell"
            >
              PRIORITAS
            </th>
            <th
              scope="col"
              className="hidden pt-1 pb-3 font-medium md:table-cell"
            >
              SISA SLA
            </th>
            <th
              scope="col"
              className="hidden pt-1 pb-3 font-medium sm:table-cell"
            >
              DIBUAT
            </th>
          </tr>
        </thead>
        <tbody className="divide-border divide-y">
          {tickets.map((ticket) => {
            const isFinished =
              ticket.status.name === "RESOLVED" ||
              ticket.status.name === "CLOSED";
            let signedRemainingMinutes: number | null = null;

            if (ticket.sla_deadline && !isFinished) {
              const deadline = new Date(ticket.sla_deadline).getTime();
              signedRemainingMinutes = Math.round(
                (deadline - now) / (1000 * 60)
              );
            }

            return (
              <tr
                key={ticket.id}
                onClick={() => router.push(`/tickets/${ticket.id}`)}
                className="group hover:bg-muted/50 cursor-pointer transition-colors"
              >
                <td className="text-foreground py-3 font-mono text-xs font-medium group-hover:underline">
                  {ticket.ticket_number}
                </td>
                <td className="text-foreground max-w-50 truncate py-3 font-medium">
                  {ticket.title}
                </td>
                <td className="py-3">
                  <StatusBadge status={ticket.status.name} />
                </td>
                <td className="hidden py-3 sm:table-cell">
                  <PriorityBadge priority={ticket.priority.name} />
                </td>
                <td className="text-muted-foreground hidden py-3 text-xs whitespace-nowrap md:table-cell">
                  {formatSlaRemaining(signedRemainingMinutes, isFinished)}
                </td>
                <td className="text-muted-foreground hidden py-3 text-xs whitespace-nowrap sm:table-cell">
                  <RelativeTime date={ticket.created_at} />
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
