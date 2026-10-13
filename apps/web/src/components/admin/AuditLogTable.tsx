"use client";

import { Eye } from "lucide-react";
import {
  DataTable,
  type ColumnDef,
} from "@/components/shared/data-table/data-table";
import { DataTableColumnHeader } from "@/components/shared/data-table/data-table-column-header";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { RelativeTime } from "@/components/shared/relative-time";
import { getAuditActionLabel, getAuditModuleLabel } from "@/lib/labels";
import type { AuditLogListItem } from "@/types/audit";
import type { PaginationMeta } from "@/types/api";

interface AuditLogTableProps {
  items: AuditLogListItem[];
  meta?: PaginationMeta;
  isLoading?: boolean;
  sortBy?: string;
  sortDir?: "asc" | "desc";
  onSort: (field: string) => void;
  onPageChange?: (page: number) => void;
  onPerPageChange?: (perPage: number) => void;
  onViewDetail: (item: AuditLogListItem) => void;
}

export function AuditLogTable({
  items,
  meta,
  isLoading = false,
  sortBy = "created_at",
  sortDir = "desc",
  onSort,
  onPageChange,
  onPerPageChange,
  onViewDetail,
}: AuditLogTableProps) {
  const columns: ColumnDef<AuditLogListItem>[] = [
    {
      id: "created_at",
      header: (
        <DataTableColumnHeader
          title="Waktu"
          sorted={sortBy === "created_at" ? sortDir : false}
          onSort={() => onSort("created_at")}
        />
      ),
      className: "w-36",
      cell: ({ row }) => (
        <div className="flex flex-col text-xs">
          <span className="text-foreground font-medium">
            <RelativeTime date={row.created_at} />
          </span>
          <span className="text-muted-foreground text-[10px]">
            {new Date(row.created_at).toLocaleDateString("id-ID", {
              day: "2-digit",
              month: "short",
              year: "numeric",
            })}
          </span>
        </div>
      ),
    },
    {
      id: "user",
      header: "Pelaku",
      className: "w-40",
      cell: ({ row }) => (
        <span className="text-foreground block truncate text-xs font-medium">
          {row.user?.full_name ?? "Sistem"}
        </span>
      ),
    },
    {
      id: "action",
      header: (
        <DataTableColumnHeader
          title="Aksi"
          sorted={sortBy === "action" ? sortDir : false}
          onSort={() => onSort("action")}
        />
      ),
      className: "w-32",
      cell: ({ row }) => (
        <Badge
          variant="outline"
          className="border-border bg-muted/40 text-foreground rounded-full px-2 py-0.5 text-[11px] font-medium"
        >
          {getAuditActionLabel(row.action)}
        </Badge>
      ),
    },
    {
      id: "module",
      header: (
        <DataTableColumnHeader
          title="Modul"
          sorted={sortBy === "module" ? sortDir : false}
          onSort={() => onSort("module")}
        />
      ),
      className: "w-32",
      cell: ({ row }) => (
        <div className="flex items-center gap-1.5 text-xs">
          <span className="text-foreground">
            {getAuditModuleLabel(row.module)}
          </span>
          {row.module_id && (
            <span className="text-muted-foreground font-mono text-[10px]">
              #{row.module_id}
            </span>
          )}
        </div>
      ),
    },
    {
      id: "description",
      header: "Keterangan",
      className: "hidden md:table-cell",
      cell: ({ row }) => (
        <span className="text-muted-foreground line-clamp-1 text-xs">
          {row.description ?? "—"}
        </span>
      ),
    },
    {
      id: "ip_address",
      header: "IP Address",
      className: "w-28 hidden md:table-cell",
      cell: ({ row }) => (
        <span className="text-muted-foreground font-mono text-xs">
          {row.ip_address ?? "—"}
        </span>
      ),
    },
    {
      id: "actions",
      header: "Rincian",
      className: "w-16 text-right",
      cell: ({ row }) => (
        <Button
          type="button"
          variant="ghost"
          size="icon"
          className="text-muted-foreground hover:text-foreground h-8 w-8"
          aria-label={`Lihat rincian log #${row.id}`}
          onClick={() => onViewDetail(row)}
        >
          <Eye className="h-4 w-4" />
        </Button>
      ),
    },
  ];

  return (
    <DataTable
      columns={columns}
      data={items}
      meta={meta}
      isLoading={isLoading}
      emptyTitle="Tidak Ada Log Audit Ditemukan"
      emptyDescription="Belum ada catatan aktivitas sistem yang cocok dengan kriteria filter Anda."
      onPageChange={onPageChange}
      onPerPageChange={onPerPageChange}
      renderCard={(log) => (
        <div
          key={log.id}
          data-testid="audit-log-card-item"
          className="border-border bg-card space-y-3 rounded-xl border p-4"
        >
          <div className="flex items-start justify-between gap-2">
            <div className="min-w-0 flex-1">
              <div className="text-foreground flex items-center gap-1.5 text-xs font-semibold">
                <span>{getAuditModuleLabel(log.module)}</span>
                {log.module_id && (
                  <span className="text-muted-foreground font-mono text-[11px]">
                    #{log.module_id}
                  </span>
                )}
              </div>
              <div className="text-muted-foreground mt-0.5 flex items-center gap-2 text-[11px]">
                <RelativeTime date={log.created_at} />
                <span>•</span>
                <span>
                  {new Date(log.created_at).toLocaleDateString("id-ID", {
                    day: "2-digit",
                    month: "short",
                    year: "numeric",
                  })}
                </span>
              </div>
            </div>
            <Badge
              variant="outline"
              className="border-border bg-muted/40 text-foreground shrink-0 rounded-full px-2 py-0.5 text-[11px] font-medium"
            >
              {getAuditActionLabel(log.action)}
            </Badge>
          </div>

          <div className="border-border/60 space-y-1 border-t pt-1 text-xs">
            <div className="flex items-center gap-1">
              <span className="text-muted-foreground text-[11px]">Pelaku:</span>
              <span className="text-foreground truncate font-medium wrap-break-word">
                {log.user?.full_name ?? "Sistem"}
              </span>
            </div>
            {log.description && (
              <p className="text-muted-foreground line-clamp-2 text-xs wrap-break-word">
                {log.description}
              </p>
            )}
          </div>

          <div className="border-border/60 flex flex-col justify-between gap-2 border-t pt-2 sm:flex-row sm:items-center">
            <div className="text-muted-foreground truncate font-mono text-[11px]">
              {log.ip_address ? `IP: ${log.ip_address}` : "IP: —"}
            </div>
            <Button
              type="button"
              variant="outline"
              size="default"
              className="text-foreground hover:bg-muted/50 flex h-11 min-h-11 w-full items-center justify-center gap-2 rounded-lg px-4 text-xs font-medium sm:w-auto"
              data-testid={`audit-log-card-detail-${log.id}`}
              aria-label={`Lihat rincian log #${log.id}`}
              onClick={() => onViewDetail(log)}
            >
              <Eye className="h-4 w-4" />
              <span>Lihat Rincian</span>
            </Button>
          </div>
        </div>
      )}
    />
  );
}
