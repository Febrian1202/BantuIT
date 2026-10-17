"use client";

import { useState } from "react";
import { AlertCircle, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { AssetDetail, AssetStatus } from "@/types/assets";

interface ChangeStatusDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  asset: AssetDetail;
  isSubmitting: boolean;
  onSubmit: (data: { status: AssetStatus; notes?: string }) => void;
}

const statusOptions: { value: AssetStatus; label: string; desc: string }[] = [
  {
    value: "available",
    label: "Tersedia",
    desc: "Perangkat siap ditugaskan ke karyawan",
  },
  {
    value: "maintenance",
    label: "Dalam Pemeliharaan",
    desc: "Perangkat sedang rusak atau dalam perbaikan",
  },
  {
    value: "retired",
    label: "Pensiun / Tidak Aktif",
    desc: "Perangkat sudah usang atau tidak lagi digunakan",
  },
  {
    value: "lost",
    label: "Hilang",
    desc: "Perangkat hilang dan tidak ditemukan",
  },
];

export function ChangeStatusDialog({
  open,
  onOpenChange,
  asset,
  isSubmitting,
  onSubmit,
}: ChangeStatusDialogProps) {
  const defaultStatus: AssetStatus =
    asset.status === "assigned" ? "available" : asset.status;
  const [status, setStatus] = useState<AssetStatus>(defaultStatus);
  const [notes, setNotes] = useState("");

  const isAssigned = !!asset.current_assignment;

  const handleOpenChange = (nextOpen: boolean) => {
    if (nextOpen) {
      setStatus(defaultStatus);
      setNotes("");
    }
    onOpenChange(nextOpen);
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (isAssigned) return;
    onSubmit({
      status,
      notes: notes.trim() || undefined,
    });
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="sm:max-w-md">
        <form onSubmit={handleSubmit} className="space-y-4">
          <DialogHeader>
            <DialogTitle>Ubah Status Aset</DialogTitle>
            <DialogDescription>
              Ubah status operasional untuk aset{" "}
              <strong>{asset.asset_tag}</strong> ({asset.name}).
            </DialogDescription>
          </DialogHeader>

          {isAssigned && (
            <div className="border-warning/30 bg-warning/10 text-foreground flex items-start gap-2.5 rounded-lg border p-3 text-xs">
              <AlertCircle className="text-warning mt-0.5 h-4 w-4 shrink-0" />
              <div className="space-y-1">
                <p className="font-semibold">
                  Perhatian: Aset Sedang Ditugaskan
                </p>
                <p className="text-muted-foreground leading-relaxed">
                  Aset ini sedang ditugaskan kepada{" "}
                  <strong>{asset.current_assignment?.full_name}</strong>.
                  Silakan lepaskan penugasan terlebih dahulu melalui tombol{" "}
                  <strong>Lepaskan Penugasan</strong> sebelum memindahkan status
                  ke Pemeliharaan, Pensiun, atau Hilang.
                </p>
              </div>
            </div>
          )}

          <div className="space-y-3 py-1">
            <div className="space-y-1.5">
              <Label htmlFor="asset-status-select">Status Operasional</Label>
              <Select
                value={status}
                onValueChange={(val) => setStatus(val as AssetStatus)}
                disabled={isAssigned || isSubmitting}
              >
                <SelectTrigger
                  id="asset-status-select"
                  aria-label="Status Operasional"
                  className="h-10 text-sm"
                >
                  <SelectValue placeholder="Pilih status aset…" />
                </SelectTrigger>
                <SelectContent>
                  {statusOptions.map((opt) => (
                    <SelectItem key={opt.value} value={opt.value}>
                      <div className="flex flex-col text-left">
                        <span className="font-medium">{opt.label}</span>
                        <span className="text-muted-foreground text-[11px]">
                          {opt.desc}
                        </span>
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-1.5">
              <Label htmlFor="change-status-notes">
                Catatan Perubahan (opsional)
              </Label>
              <Textarea
                id="change-status-notes"
                placeholder="Alasan atau keterangan perubahan status…"
                value={notes}
                onChange={(e) => setNotes(e.target.value)}
                disabled={isAssigned || isSubmitting}
                rows={3}
              />
            </div>
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isSubmitting}
            >
              Batal
            </Button>
            <Button
              type="submit"
              disabled={isAssigned || isSubmitting || status === asset.status}
              className="gap-1.5"
            >
              {isSubmitting && <Loader2 className="h-4 w-4 animate-spin" />}
              Simpan Status
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
