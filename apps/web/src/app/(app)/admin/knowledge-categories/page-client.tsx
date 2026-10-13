"use client";

import { MasterDataPage } from "@/components/shared/master-data-page";
import { knowledgeCategoryConfigBase } from "@/lib/master-data-configs";
import type { KnowledgeCategory } from "@/types/articles";
import type { ColumnDef } from "@/components/shared/data-table/data-table";

const columns: ColumnDef<KnowledgeCategory>[] = [
  {
    id: "name",
    header: "Nama Kategori",
    cell: ({ row }) => (
      <span className="text-foreground text-sm font-medium">{row.name}</span>
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

export function KnowledgeCategoriesPageClient() {
  return (
    <MasterDataPage<KnowledgeCategory>
      {...knowledgeCategoryConfigBase}
      columns={columns}
    />
  );
}
