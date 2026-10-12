import { Suspense } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { AssetsPageClient } from "./page-client";

export const metadata = {
  title: "Inventaris Aset | BantuIT",
  description: "Manajemen inventaris perangkat dan aset perusahaan.",
};

export default function AssetsPage() {
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
      <AssetsPageClient />
    </Suspense>
  );
}
