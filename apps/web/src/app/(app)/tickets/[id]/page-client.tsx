"use client";

import { useRouter } from "next/navigation";
import { useQuery } from "@tanstack/react-query";
import { ArrowLeft, Clock, Building2, Cpu } from "lucide-react";
import Link from "next/link";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { TicketDetailBanner } from "@/components/tickets/TicketDetailBanner";
import { SlaTracker } from "@/components/tickets/SlaTracker";
import {
  TicketTimeline,
  TimelineHeader,
} from "@/components/tickets/TicketTimeline";
import { TicketMobileActionBar } from "@/components/tickets/TicketMobileActionBar";
import { CommentForm } from "@/components/tickets/CommentForm";
import {
  AttachmentList,
  AttachmentSectionTitle,
} from "@/components/tickets/AttachmentList";
import { ActionDialogs } from "@/components/tickets/ActionDialogs";
import { useTicketActions } from "@/hooks/use-ticket-actions";
import { mergeTimeline } from "@/lib/timeline-merge";
import { EmptyState } from "@/components/shared/empty-state";
import { apiFetch, ApiClientError } from "@/lib/client/api";
import { ticketKeys } from "@/lib/query-key";
import type {
  TicketComment,
  TicketHistoryItem,
  TicketDetail,
  TicketAction,
} from "@/types/tickets";

function TicketLoaded({ ticket }: { ticket: TicketDetail }) {
  const ticketId = ticket.id;

  const { data: commentsResponse } = useQuery({
    queryKey: ticketKeys.comments(ticketId),
    queryFn: () =>
      apiFetch<TicketComment[]>(`/tickets/${ticketId}/comments?per_page=100`),
  });

  const { data: historiesResponse } = useQuery({
    queryKey: ticketKeys.histories(ticketId),
    queryFn: () =>
      apiFetch<TicketHistoryItem[]>(`/tickets/${ticketId}/histories`),
  });

  const comments = commentsResponse?.data ?? [];
  const histories = historiesResponse?.data ?? [];
  const entries = mergeTimeline(comments, histories);

  const actions = useTicketActions(ticket);
  const canComment = ticket.available_actions.includes(
    "comment" as TicketAction
  );
  const canAttach = ticket.available_actions.includes("attach" as TicketAction);

  return (
    <div className="space-y-6 pb-20 sm:pb-0">
      {/* Link kembali */}
      <div>
        <Button
          variant="ghost"
          size="sm"
          asChild
          className="text-muted-foreground gap-1"
        >
          <Link href="/tickets">
            <ArrowLeft className="h-3.5 w-3.5" />
            Kembali ke Daftar Tiket
          </Link>
        </Button>
      </div>

      {/* Banner */}
      <TicketDetailBanner
        ticket={ticket}
        onAction={(action) => actions.openDialog(action)}
      />

      {/* Layout utama: kiri timeline, kanan sidebar */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-12">
        {/* Kiri: Timeline & Komentar */}
        <div className="space-y-4 lg:col-span-8">
          <div className="border-border bg-card rounded-xl border p-5 shadow-xs">
            <div className="mb-4">
              <TimelineHeader />
            </div>
            <TicketTimeline entries={entries} />
            <div
              id="ticket-comment-section"
              className="border-border mt-4 border-t pt-4"
            >
              <CommentForm ticketId={ticket.id} enabled={canComment} />
            </div>
          </div>
        </div>

        {/* Sidebar Kanan*/}
        <div className="space-y-6 lg:col-span-4">
          {/* SLA Card */}
          <div className="border-border bg-card space-y-3 rounded-xl border p-5 shadow-xs">
            <div className="text-foreground flex items-center gap-2 text-sm font-semibold">
              <Clock className="text-primary h-4 w-4" />
              Batas Waktu (SLA)
            </div>
            <SlaTracker
              slaStatus={ticket.sla_status}
              duration={ticket.sla_duration_minutes}
              remaining={ticket.sla_remaining_minutes}
            />
          </div>

          {/* Info Terkait */}
          <div className="border-border bg-card space-y-3 rounded-xl border p-5 shadow-xs">
            <div className="text-foreground flex items-center gap-2 text-sm font-semibold">
              <Building2 className="text-primary h-4 w-4" />
              Informasi Tiket
            </div>
            <dl className="space-y-2 text-xs">
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Departemen</dt>
                <dd className="text-foreground font-medium">
                  {ticket.department?.name ?? "-"}
                </dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Kategori</dt>
                <dd className="text-foreground font-medium">
                  {ticket.category.name}
                </dd>
              </div>
              <div className="flex justify-between">
                <dt className="text-muted-foreground">Prioritas</dt>
                <dd className="text-foreground font-medium">
                  {ticket.priority.name}
                </dd>
              </div>
            </dl>

            {ticket.asset && (
              <div className="border-border bg-muted/40 flex items-center gap-2 rounded-lg border p-3 text-xs">
                <Cpu className="text-muted-foreground h-4 w-4 shrink-0" />
                <div className="min-w-0">
                  <p className="text-foreground truncate font-medium">
                    {ticket.asset.asset_tag}
                  </p>
                  <p className="text-muted-foreground truncate">
                    {ticket.asset.name}
                  </p>
                </div>
              </div>
            )}
          </div>

          {/* Attachments Card */}
          <div className="border-border bg-card space-y-3 rounded-xl border p-5 shadow-xs">
            <AttachmentSectionTitle count={ticket.attachments_count} />
            <AttachmentList ticketId={ticket.id} canUpload={canAttach} />
          </div>
        </div>
      </div>

      {/* Action Dialogs */}
      <ActionDialogs
        ticket={ticket}
        activeDialog={actions.activeDialog}
        activeAction={actions.activeAction}
        onClose={actions.closeDialog}
        onAssign={(d) => actions.assignAction.mutate(d)}
        onStatus={(d) => actions.statusAction.mutate(d)}
        onPriority={(d) => actions.priorityAction.mutate(d)}
        onEdit={(d) => actions.editAction.mutate(d)}
        onDelete={() => actions.deleteAction.mutate()}
        isDeletePending={actions.deleteAction.isPending}
        isEditPending={actions.editAction.isPending}
        isStatusPending={actions.statusAction.isPending}
        isAssignPending={actions.assignAction.isPending}
      />

      <TicketMobileActionBar
        ticket={ticket}
        onAction={(action) => actions.openDialog(action)}
        onCommentClick={() => {
          document
            .getElementById("ticket-comment-section")
            ?.scrollIntoView({ behavior: "smooth" });
          document.getElementById("comment-body")?.focus();
        }}
      />
    </div>
  );
}

export function TicketDetailPageClient({ ticketId }: { ticketId: number }) {
  const router = useRouter();
  const {
    data: detailResponse,
    isLoading,
    error,
  } = useQuery({
    queryKey: ticketKeys.detail(ticketId),
    queryFn: () => apiFetch<TicketDetail>(`/tickets/${ticketId}`),
    enabled: Number.isFinite(ticketId),
    retry: false,
  });

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-40" />
        <Skeleton className="h-28 w-full" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  if (error || !detailResponse?.data) {
    const is404 = error instanceof ApiClientError && error.status === 404;
    const is403 = error instanceof ApiClientError && error.status === 403;

    return (
      <div className="border-border bg-card rounded-xl border p-12 text-center shadow-xs">
        <EmptyState
          title={
            is404
              ? "Tiket Tidak Ditemukan"
              : is403
                ? "Akses Tiket Ditolak"
                : "Gagal Memuat Tiket"
          }
          description={
            is404
              ? "Tiket dengan nomor atau ID tersebut tidak ditemukan dalam sistem."
              : is403
                ? "Anda tidak memiliki hak akses untuk melihat tiket ini."
                : "Terjadi kendala saat memuat data tiket. Silakan coba kembali."
          }
          actionText="Kembali ke Daftar Tiket"
          onAction={() => router.push("/tickets")}
        />
      </div>
    );
  }

  return <TicketLoaded ticket={detailResponse.data} />;
}
