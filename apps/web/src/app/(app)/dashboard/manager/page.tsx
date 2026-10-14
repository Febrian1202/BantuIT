import { Suspense } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { ManagerDashboardPageClient } from "./page-client";

export const metadata = {
  title: "Dashboard Manager | BantuIT",
  description:
    "Pemantauan analytics SLA, performa teknisi, tren tiket layanan, dan distribusi kerja operasional.",
};

export default function ManagerDashboardPage() {
  return (
    <Suspense
      fallback={
        <div className="space-y-6 p-4">
          <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
            <div className="space-y-2">
              <Skeleton className="h-8 w-64" />
              <Skeleton className="h-4 w-48" />
            </div>
            <Skeleton className="h-9 w-48" />
          </div>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
          </div>
        </div>
      }
    >
      <ManagerDashboardPageClient />
    </Suspense>
  );
}
