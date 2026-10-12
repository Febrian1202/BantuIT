"use client";

import { MasterDataPage } from "@/components/shared/master-data-page";
import {
  departmentConfigBase,
  type DepartmentItem,
} from "@/lib/master-data-configs";
import type { ColumnDef } from "@/components/shared/data-table/data-table";

const columns: ColumnDef<DepartmentItem>[] = [
  {
    id: "name",
    header: "Nama Departemen",
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

export function DepartmentsPageClient() {
  return (
    <MasterDataPage<DepartmentItem>
      {...departmentConfigBase}
      columns={columns}
    />
  );
}
