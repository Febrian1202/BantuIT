"use client";

import React from "react";
import { AssetStatusBadge } from "./AssetStatusBadge";
import type { AssetDetail } from "@/types/assets";

interface AssetDetailCardProps {
  asset: AssetDetail;
}

function MetaRow({ label, value }: { label: string; value: React.ReactNode }) {
  return (
    <div className="flex justify-between gap-4 py-1.5">
      <dt className="text-muted-foreground">{label}</dt>
      <dd className="text-foreground text-right font-medium">{value}</dd>
    </div>
  );
}

export function AssetDetailCard({ asset }: AssetDetailCardProps) {
  return (
    <div className="border-border bg-card space-y-4 rounded-xl border p-5 shadow-xs">
      {/* Banner */}
      <div className="flex flex-wrap items-center gap-2">
        <span className="text-primary font-mono text-sm font-semibold">
          {asset.asset_tag}
        </span>
        <span className="text-muted-foreground text-sm">—</span>
        <h1 className="text-foreground text-xl font-bold tracking-tight">
          {asset.name}
        </h1>
        <AssetStatusBadge status={asset.status} className="ml-auto" />
      </div>

      {/* Metadata */}
      <dl className="divide-border border-border divide-y border-t text-sm">
        <MetaRow label="Nama Aset" value={asset.name} />
        <MetaRow label="Kode Aset" value={asset.asset_tag} />
        <MetaRow label="Kategori" value={asset.category || "—"} />
        <MetaRow label="Merek" value={asset.brand || "—"} />
        <MetaRow label="Model" value={asset.model || "—"} />
        <MetaRow label="Nomor Seri" value={asset.serial_number || "—"} />
        <MetaRow
          label="Tanggal Pembelian"
          value={
            asset.purchase_date
              ? new Date(asset.purchase_date).toLocaleDateString("id-ID")
              : "—"
          }
        />
        <MetaRow
          label="Pemegang Saat Ini"
          value={asset.current_assignment?.full_name ?? "—"}
        />
      </dl>

      {/* Catatan */}
      {asset.notes && (
        <div className="bg-muted/40 text-muted-foreground rounded-lg p-3 text-xs leading-relaxed">
          <p className="text-foreground mb-1 font-semibold">Catatan</p>
          <p className="whitespace-pre-wrap">{asset.notes}</p>
        </div>
      )}
    </div>
  );
}
