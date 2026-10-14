"use client";

import { DateRangePicker } from "@/components/dashboard/date-range-picker";
import { ManagerDashboardView } from "@/components/dashboard/manager/manager-dashboard-view";
import { AdminMetrics } from "./admin-metrics";
import { AuditLogPanel } from "./audit-log-panel";
import { ConfigShortcutsPanel } from "./config-shortcuts";
import type { AdminDashboardData } from "@/types/dashboard";
import type { User } from "@/types/auth";
import { getGreeting } from "@/lib/formatters";

export interface AdminDashboardViewProps {
  data?: AdminDashboardData;
  isLoading?: boolean;
  range?: { from?: string; to?: string };
  onRangeChange?: (range: { from: string; to: string } | null) => void;
  user?: User | null;
}

export function AdminDashboardView({
  data,
  isLoading,
  range,
  onRangeChange,
  user,
}: AdminDashboardViewProps) {
  const greeting = getGreeting();

  return (
    <div className="space-y-6">
      {/* Top Header & DateRangePicker */}
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-foreground text-xl font-bold tracking-tight sm:text-2xl">
            {greeting}, {user?.full_name || "Administrator"}
          </h1>
          <p className="text-muted-foreground mt-1 text-xs sm:text-sm">
            Ringkasan metrik sistem, status operasional, log audit, dan performa
            layanan.
          </p>
        </div>

        {onRangeChange && (
          <div className="self-start sm:self-auto">
            <DateRangePicker
              from={range?.from}
              to={range?.to}
              onChange={onRangeChange}
            />
          </div>
        )}
      </div>

      {/* 4 Admin Metric Card */}
      <AdminMetrics data={data} isLoading={isLoading} />

      {/* Section Manager */}
      <ManagerDashboardView
        data={data}
        isLoading={isLoading}
        range={range}
        hideHeader={true}
        user={user}
      />

      {/* Bottom Section: Audit Log & Config Shortcuts */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-12">
        <div className="lg:col-span-8">
          <AuditLogPanel
            items={data?.recent_system_activity}
            isLoading={isLoading}
          />
        </div>
        <div className="lg:col-span-4">
          <ConfigShortcutsPanel />
        </div>
      </div>
    </div>
  );
}
