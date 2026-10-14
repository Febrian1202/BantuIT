import React from "react";
import { DashboardPanel } from "../dashboard-panel";
import { useReferenceData } from "@/hooks/use-reference-data";
import { formatDuration } from "@/lib/formatters";
import type { PriorityDistribution } from "@/types/dashboard";
import type { TicketPriorityReference } from "@/types/tickets";

/**
 * Helper untuk menggabungkan API bucket data dengan reference list
 */
export function mergePriorityDistribution(
  apiData: PriorityDistribution[],
  reference: TicketPriorityReference[]
): { label: string; count: number; percentage: number }[] {
  const total = apiData.reduce((sum, item) => sum + item.count, 0);

  const result: { label: string; count: number; percentage: number }[] = [];
  const processedNames = new Set<string>();

  // 1. Proses reference priorities
  for (const ref of reference) {
    const bucket = apiData.find((b) => b.priority === ref.name);
    const count = bucket ? bucket.count : 0;
    const percentage = total > 0 ? (count / total) * 100 : 0;

    result.push({
      label: `${ref.name} (${formatDuration(ref.sla_minutes)})`,
      count,
      percentage,
    });

    processedNames.add(ref.name);
  }

  // 2. Proses soft-delete / prioritas yang tidak dikenali di apiData
  for (const bucket of apiData) {
    if (!processedNames.has(bucket.priority)) {
      const percentage = total > 0 ? (bucket.count / total) * 100 : 0;
      result.push({
        label: bucket.priority,
        count: bucket.count,
        percentage,
      });
    }
  }

  return result;
}

export interface PriorityDistributionPanelProps {
  data: PriorityDistribution[];
  isLoading?: boolean;
}

export function PriorityDistributionPanel({
  data,
  isLoading,
}: PriorityDistributionPanelProps) {
  const { priorities, isLoading: isRefLoading } = useReferenceData();

  const loading = isLoading || isRefLoading;
  const merged = React.useMemo(
    () => mergePriorityDistribution(data, priorities),
    [data, priorities]
  );
  const hasData = data.length > 0;

  const maxCount = merged.reduce(
    (max, item) => (item.count > max ? item.count : max),
    0
  );

  return (
    <DashboardPanel
      title="Distribusi Berdasarkan Prioritas"
      isLoading={loading}
      isEmpty={!hasData}
      emptyMessage="Belum ada data distribusi."
    >
      <div
        className="space-y-3 sm:space-y-4"
        role="list"
        aria-label="Distribusi prioritas"
      >
        {merged.map((item, i) => (
          <div key={i} className="space-y-1" role="listitem">
            <div className="flex items-end justify-between text-xs sm:text-sm">
              <span className="text-foreground font-medium">{item.label}</span>
              <span className="text-muted-foreground ml-2 shrink-0 text-[11px] sm:text-xs">
                {item.count} tiket ({item.percentage.toFixed(0)}%)
              </span>
            </div>
            <div className="bg-muted h-1.5 overflow-hidden rounded-full">
              <div
                className="bg-foreground h-full rounded-full transition-all duration-500 ease-in-out"
                style={{
                  width: `${maxCount > 0 ? (item.count / maxCount) * 100 : 0}%`,
                }}
              />
            </div>
          </div>
        ))}
      </div>
    </DashboardPanel>
  );
}
