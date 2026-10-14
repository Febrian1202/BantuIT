import { Suspense } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { EmployeeDashboardPageClient } from "./page-client";

export const metadata = {
  title: "Dashboard Karyawan | BantuIT",
  description:
    "Ringkasan tiket permohonan bantuan, aset yang dipegang, dan artikel bantuan terbaru.",
};

export default function EmployeeDashboardPage() {
  return (
    <Suspense
      fallback={
        <div className="space-y-6 p-4">
          <div className="space-y-2">
            <Skeleton className="h-8 w-64" />
            <Skeleton className="h-4 w-48" />
          </div>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
            <Skeleton className="h-28 w-full" />
          </div>
          <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
            <Skeleton className="h-72 w-full lg:col-span-2" />
            <Skeleton className="h-72 w-full" />
          </div>
        </div>
      }
    >
      <EmployeeDashboardPageClient />
    </Suspense>
  );
}
