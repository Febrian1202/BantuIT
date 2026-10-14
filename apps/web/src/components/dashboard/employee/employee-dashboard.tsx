"use client";

import Link from "next/link";
import {
  Ticket,
  Clock,
  CheckCircle2,
  Laptop,
  Plus,
  ArrowRight,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { MetricCard } from "@/components/dashboard/metric-card";
import { DashboardPanel } from "@/components/dashboard/dashboard-panel";
import { TicketMiniTable } from "./ticket-mini-table";
import { EmployeeAssetList } from "./asset-list";
import { EmployeeArticleList } from "./article-list";
import { useEmployeeDashboard } from "@/hooks/use-dashboards";
import { useAuth } from "@/components/providers/auth-provider";
import { getGreeting } from "@/lib/formatters";

export function EmployeeDashboard() {
  const { data, isLoading } = useEmployeeDashboard();
  const { user, can } = useAuth();

  const greeting = getGreeting();
  const userName = user?.full_name ?? "Karyawan";
  const openCount = data?.my_open_tickets ?? 0;

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-foreground text-xl font-bold tracking-tight sm:text-2xl">
            {greeting}, {userName}
          </h1>
          <p className="text-muted-foreground mt-1 text-xs sm:text-sm">
            Kamu punya {openCount} ticket yang sedang berjalan
          </p>
        </div>

        {can("ticket.create") && (
          <Button
            asChild
            size="sm"
            className="gap-1.5 self-start rounded-lg sm:self-auto"
          >
            <Link href="/tickets/new">
              <Plus className="h-4 w-4" />
              <span>Buat Ticket</span>
            </Link>
          </Button>
        )}
      </div>

      <div className="grid grid-cols-2 gap-3 sm:gap-4 lg:grid-cols-4">
        <MetricCard
          label="Ticket Terbuka"
          value={data ? data.my_open_tickets : null}
          icon={Ticket}
          footer="Menunggu penanganan"
          isLoading={isLoading}
        />
        <MetricCard
          label="Sedang Dikerjakan"
          value={data ? data.my_in_progress_tickets : null}
          icon={Clock}
          footer="Sedang ditangani teknisi"
          isLoading={isLoading}
        />
        <MetricCard
          label="Selesai"
          value={data ? data.my_resolved_tickets : null}
          icon={CheckCircle2}
          footer="Total selesai"
          isLoading={isLoading}
        />
        <MetricCard
          label="Aset Dipegang"
          value={data ? data.my_assets.length : null}
          icon={Laptop}
          footer="Aset terdaftar atas nama Anda"
          isLoading={isLoading}
        />
      </div>

      <div className="grid grid-cols-1 items-start gap-6 lg:grid-cols-3">
        <div className="lg:col-span-2">
          <DashboardPanel
            title="Ticket Saya"
            actionLabel="Lihat semua"
            actionHref="/tickets"
            actionIcon={ArrowRight}
            isLoading={isLoading}
            isEmpty={
              !isLoading &&
              (!data?.recent_tickets || data.recent_tickets.length === 0)
            }
            emptyMessage="Belum ada tiket."
          >
            <TicketMiniTable
              tickets={data?.recent_tickets ?? []}
              isLoading={isLoading}
            />
          </DashboardPanel>
        </div>

        <div className="space-y-6">
          <DashboardPanel
            title="Aset Yang Kamu Pegang"
            actionLabel="Lihat semua"
            actionHref="/my-assets"
            actionIcon={ArrowRight}
            isLoading={isLoading}
            isEmpty={
              !isLoading && (!data?.my_assets || data.my_assets.length === 0)
            }
            emptyMessage="Kamu belum memegang aset apa pun."
          >
            <EmployeeAssetList
              assets={data?.my_assets ?? []}
              isLoading={isLoading}
            />
          </DashboardPanel>

          <DashboardPanel
            title="Artikel Terbaru"
            actionLabel="Lihat semua"
            actionHref="/knowledge"
            actionIcon={ArrowRight}
            isLoading={isLoading}
            isEmpty={
              !isLoading &&
              (!data?.recent_articles || data.recent_articles.length === 0)
            }
            emptyMessage="Belum ada artikel terbaru."
          >
            <EmployeeArticleList
              articles={data?.recent_articles ?? []}
              isLoading={isLoading}
            />
          </DashboardPanel>
        </div>
      </div>
    </div>
  );
}
