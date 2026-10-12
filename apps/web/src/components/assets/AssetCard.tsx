"use client";

import { AssetStatusBadge } from "./AssetStatusBadge";
import type { AssetListItem } from "@/types/assets";

export interface AssetCardProps {
  asset: AssetListItem;
  onClick?: (asset: AssetListItem) => void;
  showHolder?: boolean;
}

export function AssetCard({
  asset,
  onClick,
  showHolder = true,
}: AssetCardProps) {
  return (
    <div
      role="button"
      tabIndex={0}
      aria-label={`Aset ${asset.asset_tag}: ${asset.name}`}
      onClick={() => onClick?.(asset)}
      onKeyDown={(e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          onClick?.(asset);
        }
      }}
      className="border-border bg-card hover:bg-muted/50 flex min-h-11 cursor-pointer flex-col gap-2.5 border-b p-4 transition-colors"
    >
      <div className="flex items-center justify-between gap-2">
        <span className="text-primary font-mono text-xs font-semibold">
          {asset.asset_tag}
        </span>
        <AssetStatusBadge status={asset.status} />
      </div>

      <div className="space-y-0.5">
        <h3 className="text-foreground line-clamp-1 text-sm font-medium">
          {asset.name}
        </h3>
        <p className="text-muted-foreground text-xs">
          {[asset.brand, asset.model].filter(Boolean).join(" — ") ||
            asset.category ||
            "—"}
        </p>
      </div>

      <div className="border-border text-muted-foreground flex items-center justify-between gap-2 border-t pt-2 text-xs">
        {showHolder && (
          <div className="min-w-0 truncate">
            <span>Pemegang: </span>
            <span className="text-foreground font-medium">
              {asset.current_assignment?.full_name ?? "—"}
            </span>
          </div>
        )}
        <div className="ml-auto shrink-0 text-right">
          <span>
            {asset.purchase_date
              ? new Date(asset.purchase_date).toLocaleDateString("id-ID")
              : "—"}
          </span>
        </div>
      </div>
    </div>
  );
}
