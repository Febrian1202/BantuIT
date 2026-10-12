"use client";

import React, { useState } from "react";
import { Check, Copy, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { apiFetch } from "@/lib/client/api";
import { useApiMutation } from "@/hooks/use-api-mutation";
import type { UserListItem } from "@/types/auth";

interface ResetPasswordDialogProps {
  open: boolean;
  user: UserListItem | null;
  onClose: () => void;
}

interface ResetPasswordResult {
  temporary_password: string;
}

export function ResetPasswordDialog({
  open,
  user,
  onClose,
}: ResetPasswordDialogProps) {
  const [copied, setCopied] = useState(false);

  const mutation = useApiMutation<void, ResetPasswordResult>({
    mutationFn: () =>
      apiFetch<ResetPasswordResult>(`/users/${user!.id}/reset-password`, {
        method: "POST",
      }),
    invalidateKeys: [],
  });

  React.useEffect(() => {
    if (
      open &&
      user &&
      !mutation.isPending &&
      !mutation.data &&
      !mutation.isError
    ) {
      mutation.mutate();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open, user?.id]);

  const handleOpenChange = (isOpen: boolean) => {
    if (!isOpen) {
      setCopied(false);
      mutation.reset();
      onClose();
    }
  };

  const temporaryPassword = mutation.data?.data?.temporary_password;

  const handleCopy = async () => {
    if (!temporaryPassword) return;
    try {
      await navigator.clipboard.writeText(temporaryPassword);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      setCopied(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="max-h-[92vh] w-[95vw] overflow-y-auto p-4 sm:max-h-[85vh] sm:max-w-md sm:p-6">
        <DialogHeader>
          <DialogTitle>Reset Password Pengguna</DialogTitle>
          <DialogDescription>
            Buat password sementara baru untuk{" "}
            <strong className="text-foreground">{user?.full_name}</strong>.
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4 py-1">
          {mutation.isPending && (
            <div className="bg-muted/40 text-muted-foreground flex items-center gap-2 rounded-lg p-4 text-sm">
              <Loader2 className="h-4 w-4 animate-spin" />
              <span>Memproses reset password…</span>
            </div>
          )}

          {mutation.isError && (
            <div className="border-destructive/30 bg-destructive/10 text-destructive rounded-lg border p-4 text-sm">
              Gagal mereset password. Silakan coba lagi.
            </div>
          )}

          {temporaryPassword && (
            <div className="space-y-3">
              <div className="border-border bg-muted/40 flex items-center justify-between gap-3 rounded-lg border p-3">
                <code className="text-foreground font-mono text-sm font-semibold select-all">
                  {temporaryPassword}
                </code>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={handleCopy}
                  className="h-11 min-h-11 shrink-0 gap-1.5 px-3 sm:h-8 sm:min-h-0"
                >
                  {copied ? (
                    <>
                      <Check className="h-3.5 w-3.5" />
                      <span>Tersalin</span>
                    </>
                  ) : (
                    <>
                      <Copy className="h-3.5 w-3.5" />
                      <span>Salin</span>
                    </>
                  )}
                </Button>
              </div>
              <p className="text-destructive text-xs font-medium">
                Password baru hanya ditampilkan satu kali. Tutup dialog ini dan
                password tidak akan dapat dilihat kembali.
              </p>
              <p className="text-muted-foreground text-xs">
                Semua sesi login aktif pengguna ini telah dicabut dan harus
                login kembali dengan password baru.
              </p>
            </div>
          )}
        </div>

        <DialogFooter className="bg-card border-border sticky bottom-0 border-t pt-2 pb-1 sm:static sm:border-0">
          <Button
            type="button"
            onClick={onClose}
            disabled={mutation.isPending}
            className="h-11 min-h-11 w-full sm:h-9 sm:min-h-0 sm:w-auto"
          >
            Tutup
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
