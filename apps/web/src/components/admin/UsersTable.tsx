"use client";

import { KeyRound, Pencil, Power, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  DataTable,
  type ColumnDef,
} from "@/components/shared/data-table/data-table";
import { DataTableColumnHeader } from "@/components/shared/data-table/data-table-column-header";
import { getRoleLabel, userStatusLabels } from "@/lib/labels";
import { cn } from "@/lib/utils";
import type { UserListItem } from "@/types/auth";
import type { PaginationMeta } from "@/types/api";

interface UsersTableProps {
  items: UserListItem[];
  meta?: PaginationMeta;
  isLoading?: boolean;
  sortBy?: string;
  sortDir?: "asc" | "desc";
  onSort: (field: string) => void;
  onPageChange?: (page: number) => void;
  onPerPageChange?: (perPage: number) => void;
  onEdit: (user: UserListItem) => void;
  onResetPassword: (user: UserListItem) => void;
  onDelete: (user: UserListItem) => void;
  onToggleStatus: (user: UserListItem) => void;
}

export function UsersTable({
  items,
  meta,
  isLoading = false,
  sortBy = "full_name",
  sortDir = "asc",
  onSort,
  onPageChange,
  onPerPageChange,
  onEdit,
  onResetPassword,
  onDelete,
  onToggleStatus,
}: UsersTableProps) {
  const columns: ColumnDef<UserListItem>[] = [
    {
      id: "full_name",
      header: (
        <DataTableColumnHeader
          title="Nama Lengkap"
          sorted={sortBy === "full_name" ? sortDir : false}
          onSort={() => onSort("full_name")}
        />
      ),
      cell: ({ row }) => (
        <div className="max-w-xs">
          <p className="text-foreground truncate text-sm font-medium">
            {row.full_name}
          </p>
          <p className="text-muted-foreground truncate text-xs">{row.email}</p>
        </div>
      ),
    },
    {
      id: "email",
      header: (
        <DataTableColumnHeader
          title="Email"
          sorted={sortBy === "email" ? sortDir : false}
          onSort={() => onSort("email")}
        />
      ),
      className: "hidden md:table-cell",
      cell: ({ row }) => (
        <span className="text-muted-foreground text-xs">{row.email}</span>
      ),
    },
    {
      id: "role",
      header: "Role",
      className: "w-32",
      cell: ({ row }) => (
        <Badge
          variant="outline"
          className="border-border bg-muted/50 text-foreground rounded-full text-xs font-medium shadow-none"
        >
          {getRoleLabel(row.role?.name ?? "")}
        </Badge>
      ),
    },
    {
      id: "department",
      header: "Departemen",
      className: "w-32 hidden md:table-cell",
      cell: ({ row }) => (
        <span className="text-muted-foreground text-xs">
          {row.department?.name ?? "—"}
        </span>
      ),
    },
    {
      id: "employee_code",
      header: "Kode Karyawan",
      className: "w-28 hidden md:table-cell",
      cell: ({ row }) => (
        <span className="text-muted-foreground font-mono text-xs">
          {row.employee_code ?? "—"}
        </span>
      ),
    },
    {
      id: "status",
      header: (
        <DataTableColumnHeader
          title="Status"
          sorted={sortBy === "status" ? sortDir : false}
          onSort={() => onSort("status")}
        />
      ),
      className: "w-24",
      cell: ({ row }) => {
        const isActive = row.status === "active";
        return (
          <Badge
            variant="outline"
            className={cn(
              "rounded-full border-transparent text-xs font-medium shadow-none",
              isActive
                ? "border-[#d9e5d4] bg-[#eaf0e6] text-[#4d663e]"
                : "bg-muted text-muted-foreground"
            )}
          >
            {userStatusLabels[row.status] ?? row.status}
          </Badge>
        );
      },
    },
    {
      id: "actions",
      header: "Aksi",
      className: "w-40 text-right",
      cell: ({ row }) => {
        const isActive = row.status === "active";
        return (
          <div className="flex items-center justify-end gap-1">
            <Button
              type="button"
              variant="ghost"
              size="icon"
              className="text-muted-foreground hover:text-foreground h-8 w-8 rounded-lg"
              aria-label={`Edit pengguna ${row.full_name}`}
              onClick={() => onEdit(row)}
            >
              <Pencil className="h-4 w-4" />
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              className="text-muted-foreground hover:text-foreground h-8 w-8 rounded-lg"
              aria-label={`Reset password pengguna ${row.full_name}`}
              onClick={() => onResetPassword(row)}
            >
              <KeyRound className="h-4 w-4" />
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              className={cn(
                "hover:text-foreground h-8 w-8 rounded-lg",
                isActive ? "text-muted-foreground" : "text-emerald-700"
              )}
              aria-label={
                isActive
                  ? `Nonaktifkan pengguna ${row.full_name}`
                  : `Aktifkan pengguna ${row.full_name}`
              }
              onClick={() => onToggleStatus(row)}
            >
              <Power className="h-4 w-4" />
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              className="text-muted-foreground hover:text-destructive h-8 w-8 rounded-lg"
              aria-label={`Hapus pengguna ${row.full_name}`}
              onClick={() => onDelete(row)}
            >
              <Trash2 className="h-4 w-4" />
            </Button>
          </div>
        );
      },
    },
  ];

  return (
    <DataTable
      columns={columns}
      data={items}
      meta={meta}
      isLoading={isLoading}
      emptyTitle="Tidak Ada Pengguna Ditemukan"
      emptyDescription="Belum ada pengguna yang cocok dengan kriteria pencarian atau filter Anda."
      onPageChange={onPageChange}
      onPerPageChange={onPerPageChange}
      renderCard={(user) => {
        const isActive = user.status === "active";
        return (
          <div
            key={user.id}
            data-testid="user-card-item"
            className="border-border bg-card space-y-3 rounded-xl border p-4"
          >
            <div className="space-y-1.5">
              <div className="flex items-start justify-between gap-2">
                <div className="min-w-0 flex-1">
                  <h4 className="text-foreground text-sm font-semibold wrap-break-word">
                    {user.full_name}
                  </h4>
                  <p className="text-muted-foreground text-xs break-all">
                    {user.email}
                  </p>
                </div>
              </div>

              <div className="flex flex-wrap items-center gap-1.5 pt-1">
                <Badge
                  variant="outline"
                  className="border-border bg-muted/50 text-foreground rounded-full text-[11px] font-medium shadow-none"
                >
                  {getRoleLabel(user.role?.name ?? "")}
                </Badge>
                <Badge
                  variant="outline"
                  className={cn(
                    "rounded-full border-transparent text-[11px] font-medium shadow-none",
                    isActive
                      ? "border-[#d9e5d4] bg-[#eaf0e6] text-[#4d663e]"
                      : "bg-muted text-muted-foreground"
                  )}
                >
                  {userStatusLabels[user.status] ?? user.status}
                </Badge>
              </div>
            </div>

            <div className="border-border/60 grid grid-cols-2 gap-2 border-t pt-2 text-xs">
              <div className="min-w-0">
                <span className="text-muted-foreground block text-[11px]">
                  Departemen
                </span>
                <span className="text-foreground block truncate font-medium">
                  {user.department?.name ?? "—"}
                </span>
              </div>
              <div className="min-w-0">
                <span className="text-muted-foreground block text-[11px]">
                  Kode Karyawan
                </span>
                <span className="text-foreground block truncate font-mono">
                  {user.employee_code ?? "—"}
                </span>
              </div>
            </div>

            <div className="border-border/60 grid grid-cols-4 gap-2 border-t pt-2">
              <Button
                type="button"
                variant="outline"
                size="icon"
                className="text-muted-foreground hover:text-foreground h-11 min-h-11 w-full min-w-11 rounded-lg"
                data-testid={`user-card-edit-${user.id}`}
                aria-label={`Edit pengguna ${user.full_name}`}
                onClick={() => onEdit(user)}
              >
                <Pencil className="h-4 w-4" />
              </Button>
              <Button
                type="button"
                variant="outline"
                size="icon"
                className="text-muted-foreground hover:text-foreground h-11 min-h-11 w-full min-w-11 rounded-lg"
                data-testid={`user-card-reset-${user.id}`}
                aria-label={`Reset password pengguna ${user.full_name}`}
                onClick={() => onResetPassword(user)}
              >
                <KeyRound className="h-4 w-4" />
              </Button>
              <Button
                type="button"
                variant="outline"
                size="icon"
                className={cn(
                  "hover:text-foreground h-11 min-h-11 w-full min-w-11 rounded-lg",
                  isActive ? "text-muted-foreground" : "text-emerald-700"
                )}
                data-testid={`user-card-toggle-${user.id}`}
                aria-label={
                  isActive
                    ? `Nonaktifkan pengguna ${user.full_name}`
                    : `Aktifkan pengguna ${user.full_name}`
                }
                onClick={() => onToggleStatus(user)}
              >
                <Power className="h-4 w-4" />
              </Button>
              <Button
                type="button"
                variant="outline"
                size="icon"
                className="text-muted-foreground hover:text-destructive h-11 min-h-11 w-full min-w-11 rounded-lg"
                data-testid={`user-card-delete-${user.id}`}
                aria-label={`Hapus pengguna ${user.full_name}`}
                onClick={() => onDelete(user)}
              >
                <Trash2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        );
      }}
    />
  );
}
