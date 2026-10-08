import Link from "next/link";
import { FileQuestion } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function NotFound() {
  return (
    <div className="bg-background flex min-h-screen items-center justify-center p-4">
      <Card className="border-border bg-card max-w-md text-center shadow-sm">
        <CardHeader className="flex flex-col items-center space-y-2 p-6">
          <div className="bg-primary/10 text-primary flex h-12 w-12 items-center justify-center rounded-full">
            <FileQuestion className="h-6 w-6" />
          </div>
          <CardTitle className="text-xl font-bold">
            404 — Halaman Tidak Ditemukan
          </CardTitle>
          <CardDescription>
            Halaman atau data yang Anda cari tidak tersedia atau tautan telah
            berubah.
          </CardDescription>
        </CardHeader>
        <CardContent className="p-6 pt-0">
          <Button asChild className="w-full">
            <Link href="/">Kembali ke Halaman Utama</Link>
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
