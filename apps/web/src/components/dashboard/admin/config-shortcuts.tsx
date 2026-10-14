import Link from "next/link";
import {
  Settings2,
  Users,
  Building2,
  Tags,
  Sliders,
  FolderKanban,
  ChevronRight,
} from "lucide-react";
import { DashboardPanel } from "@/components/dashboard/dashboard-panel";

const shortcuts = [
  {
    title: "Pengguna & Role",
    description: "Manajemen akun, peran pengguna, dan status aktif/non-aktif.",
    href: "/admin/users",
    icon: Users,
  },
  {
    title: "Departemen",
    description:
      "Kelola divisi, unit kerja, dan pemetaan departemen perusahaan.",
    href: "/admin/departments",
    icon: Building2,
  },
  {
    title: "Kategori Ticket",
    description: "Atur kategori pengelompokan laporan kendala IT.",
    href: "/admin/categories",
    icon: Tags,
  },
  {
    title: "Prioritas & SLA",
    description: "Konfigurasi tingkat prioritas dan target durasi penanganan.",
    href: "/admin/priorities",
    icon: Sliders,
  },
  {
    title: "Kategori Basis Pengetahuan",
    description: "Struktur taksonomi topik untuk artikel knowledge base.",
    href: "/admin/knowledge-categories",
    icon: FolderKanban,
  },
];

export function ConfigShortcutsPanel() {
  return (
    <DashboardPanel
      title="Pintasan Konfigurasi"
      actionLabel="Kelola"
      actionHref="/admin/users"
      actionIcon={Settings2}
    >
      <div className="space-y-3">
        <p className="text-muted-foreground text-xs">
          Akses cepat pengelolaan master data &amp; pengaturan sistem.
        </p>
        <div className="flex flex-col gap-2.5">
          {shortcuts.map((shortcut) => {
            const Icon = shortcut.icon;
            return (
              <Link
                key={shortcut.href}
                href={shortcut.href}
                className="group rounded-card border-border bg-card hover:bg-muted/50 hover:border-border/80 focus-visible:ring-ring flex items-center gap-3.5 border p-3.5 shadow-xs transition-colors focus-visible:ring-2 focus-visible:outline-none"
              >
                <div className="bg-muted/60 text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary flex h-9 w-9 shrink-0 items-center justify-center rounded-lg transition-colors">
                  <Icon className="h-4.5 w-4.5" strokeWidth={1.5} />
                </div>
                <div className="min-w-0 flex-1">
                  <h4 className="text-foreground group-hover:text-primary text-sm font-semibold tracking-tight transition-colors">
                    {shortcut.title}
                  </h4>
                  <p className="text-muted-foreground mt-0.5 line-clamp-2 text-xs leading-relaxed">
                    {shortcut.description}
                  </p>
                </div>
                <ChevronRight
                  className="text-muted-foreground/50 group-hover:text-foreground h-4 w-4 shrink-0 transition-all group-hover:translate-x-0.5"
                  strokeWidth={1.5}
                />
              </Link>
            );
          })}
        </div>
      </div>
    </DashboardPanel>
  );
}
