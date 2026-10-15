import { Suspense } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { NotificationsPageClient } from "./page-client";

export const metadata = {
  title: "Pusat Notifikasi | BantuIT",
  description:
    "Riwayat pemberitahuan penugasan tiket, perubahan status, dan peringatan SLA.",
};

export default function NotificationsPage() {
  return (
    <Suspense
      fallback={
        <div className="space-y-4 p-4">
          <Skeleton className="h-8 w-48" />
          <Skeleton className="h-10 w-full" />
          <Skeleton className="h-64 w-full" />
        </div>
      }
    >
      <NotificationsPageClient />
    </Suspense>
  );
}
