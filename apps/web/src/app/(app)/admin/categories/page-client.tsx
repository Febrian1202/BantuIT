"use client";

import { MasterDataPage } from "@/components/shared/master-data-page";
import { ticketCategoryConfigBase } from "@/lib/master-data-configs";
import type { TicketCategory } from "@/types/tickets";
import type { ColumnDef } from "@/components/shared/data-table/data-table";

const columns: ColumnDef<TicketCategory>[] = [
  {
    id: "name",
    header: "Nama Kategori",
    cell: ({ row }) => (
      <span className="text-foreground text-sm font-medium">{row.name}</span>
    ),
  },
  {
    id: "parent",
    header: "Kategori Induk",
    className: "w-48",
    cell: ({ row }) => (
      <span className="text-muted-foreground text-xs">
        {row.parent?.name ?? "—"}
      </span>
    ),
  },
  {
    id: "description",
    header: "Deskripsi",
    cell: ({ row }) => (
      <span className="text-muted-foreground text-xs">
        {row.description ?? "—"}
      </span>
    ),
  },
];

export function TicketCategoriesPageClient() {
  return (
    <MasterDataPage<TicketCategory>
      {...ticketCategoryConfigBase}
      columns={columns}
    />
  );
}
