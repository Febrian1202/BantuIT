"use client";

import { useEffect } from "react";
import { AlertCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

export default function GlobalError({
  error,
  reset,
}: {
  error: Error & { digest?: string };
  reset: () => void;
}) {
  useEffect(() => {
    // Log error ke console untuk keperluan debugging
    console.error("Unhandled Application Error:", error?.message, error?.stack);
  }, [error]);

  return (
    <div className="flex min-h-[60vh] items-center justify-center p-4">
      <Card className="border-border bg-card max-w-md text-center shadow-sm">
        <CardHeader className="flex flex-col items-center space-y-2 p-6">
          <div className="bg-destructive/10 text-destructive flex h-12 w-12 items-center justify-center rounded-full">
            <AlertCircle className="h-6 w-6" />
          </div>
          <CardTitle className="text-xl font-bold">Terjadi Kesalahan</CardTitle>
          <CardDescription>
            Sistem mendeteksi kendala pada pemuatan halaman ini. Silakan coba
            muat ulang atau hubungi administrator.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-2 p-6 pt-0">
          <Button onClick={() => reset()} className="w-full">
            Coba Lagi
          </Button>
        </CardContent>
      </Card>
    </div>
  );
}
