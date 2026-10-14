import { useRouter } from "next/navigation";
import { ScrollText } from "lucide-react";
import { DashboardPanel } from "@/components/dashboard/dashboard-panel";
import { RelativeTime } from "@/components/shared/relative-time";
import { EmptyState } from "@/components/shared/empty-state";
import { Badge } from "@/components/ui/badge";
import { Skeleton } from "@/components/ui/skeleton";
import {
  Table,
  TableHeader,
  TableBody,
  TableRow,
  TableHead,
  TableCell,
} from "@/components/ui/table";
import { getAuditActionLabel, getAuditModuleLabel } from "@/lib/labels";
import type { AdminDashboardData } from "@/types/dashboard";

interface AuditLogPanelProps {
  items?: AdminDashboardData["recent_system_activity"];
  isLoading?: boolean;
}

export function AuditLogPanel({
  items,
  isLoading = false,
}: AuditLogPanelProps) {
  const router = useRouter();
  const displayItems = items?.slice(0, 8) || [];

  return (
    <DashboardPanel
      title="Audit Log Terbaru"
      actionLabel="Lihat Semua"
      actionHref="/admin/audit-logs"
      actionIcon={ScrollText}
    >
      <div className="space-y-3">
        <p className="text-muted-foreground text-xs">
          Catatan riwayat aktivitas dan perubahan sistem terkini.
        </p>

        {isLoading ? (
          <div className="space-y-3 py-2">
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
            <Skeleton className="h-10 w-full" />
          </div>
        ) : !items || items.length === 0 ? (
          <EmptyState
            icon={ScrollText}
            title="Belum ada aktivitas sistem"
            description="Belum ada catatan log aktivitas yang terekam pada sistem."
          />
        ) : (
          <div className="border-border overflow-hidden rounded-md border">
            <Table>
              <caption className="sr-only">
                Daftar aktivitas log sistem terbaru
              </caption>
              <TableHeader className="bg-muted/30">
                <TableRow className="border-border text-xs hover:bg-transparent">
                  <TableHead
                    scope="col"
                    className="text-muted-foreground py-2.5 font-semibold"
                  >
                    WAKTU
                  </TableHead>
                  <TableHead
                    scope="col"
                    className="text-muted-foreground py-2.5 font-semibold"
                  >
                    PENGGUNA
                  </TableHead>
                  <TableHead
                    scope="col"
                    className="text-muted-foreground py-2.5 font-semibold"
                  >
                    AKSI &amp; MODUL
                  </TableHead>
                  <TableHead
                    scope="col"
                    className="text-muted-foreground py-2.5 font-semibold"
                  >
                    KETERANGAN
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody className="divide-border divide-y">
                {displayItems.map((item) => {
                  const userName = item.user?.full_name ?? "Sistem";
                  const actionLabel = getAuditActionLabel(item.action);
                  const moduleLabel = getAuditModuleLabel(item.module);

                  return (
                    <TableRow
                      key={item.id}
                      tabIndex={0}
                      onClick={() => router.push("/admin/audit-logs")}
                      onKeyDown={(e) => {
                        if (e.key === "Enter" || e.key === " ") {
                          e.preventDefault();
                          router.push("/admin/audit-logs");
                        }
                      }}
                      className="hover:bg-muted/40 focus-visible:bg-muted/40 border-border cursor-pointer text-xs transition-colors focus-visible:outline-none"
                    >
                      <TableCell className="text-muted-foreground py-2.5 whitespace-nowrap">
                        <RelativeTime date={item.created_at} />
                      </TableCell>
                      <TableCell className="text-foreground py-2.5 font-medium whitespace-nowrap">
                        {userName}
                      </TableCell>
                      <TableCell className="py-2.5 whitespace-nowrap">
                        <div className="flex flex-wrap items-center gap-1.5">
                          <Badge
                            variant="outline"
                            className="text-[11px] font-normal"
                          >
                            {actionLabel}
                          </Badge>
                          <Badge
                            variant="secondary"
                            className="text-[11px] font-normal"
                          >
                            {moduleLabel}
                          </Badge>
                        </div>
                      </TableCell>
                      <TableCell className="text-muted-foreground max-w-xs truncate py-2.5 sm:max-w-md">
                        {item.description || "—"}
                      </TableCell>
                    </TableRow>
                  );
                })}
              </TableBody>
            </Table>
          </div>
        )}
      </div>
    </DashboardPanel>
  );
}
