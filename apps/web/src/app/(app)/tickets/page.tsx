import { Suspense } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { TicketsPageClient } from "./page-client";

export const metadata = {
  title: "Daftar Tiket | BantuIT",
  description: "Kelola dan pantau permohonan tiket bantuan operasional IT.",
};

export default function TicketsPage() {
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
      <TicketsPageClient />
    </Suspense>
  );
}
