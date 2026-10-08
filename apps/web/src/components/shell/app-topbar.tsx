import { usePathname } from "next/navigation";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Menu } from "lucide-react";
import { NotificationBell } from "./notification-bell";
import { ThemeToggle } from "./theme-toggle";
import { NavUser } from "./nav-user";
import { MobileNav } from "./mobile-nav";

function getPageTitle(pathname: string): string {
  if (pathname.startsWith("/dashboard")) return "Dashboard";
  if (pathname.startsWith("/my-assets")) return "Aset Saya";
  if (pathname.startsWith("/assets")) return "Inventaris Aset";
  if (pathname.startsWith("/knowledge")) return "Basis Pengetahuan";
  if (pathname.startsWith("/notifications")) return "Pusat Notifikasi";
  if (pathname.startsWith("/profile")) return "Profil Pengguna";
  if (pathname.startsWith("/admin/users")) return "Kelola Pengguna";
  if (pathname.startsWith("/admin/departments")) return "Kelola Departemen";
  if (pathname.startsWith("/admin/categories")) return "Kategori Tiket";
  if (pathname.startsWith("/admin/knowledge-categories"))
    return "Kategori Pengetahuan";
  if (pathname.startsWith("/admin/priorities")) return "Prioritas & SLA";
  if (pathname.startsWith("/admin/audit-logs")) return "Log Audit";
  if (pathname.startsWith("/403")) return "Akses Ditolak";
  return "BANTU IT";
}

export function AppTopbar() {
  const [mobileNavOpen, setMobileNavOpen] = useState(false);
  const pathname = usePathname();
  const pageTitle = getPageTitle(pathname);

  return (
    <>
      <header className="border-border bg-card/80 sticky top-0 z-20 flex h-14 w-full items-center justify-between border-b px-4 backdrop-blur-sm sm:px-6">
        {/* Left: Mobile Toggle & Page Title */}
        <div className="flex items-center gap-3">
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setMobileNavOpen(true)}
            className="md:hidden"
            aria-label="Buka menu navigasi"
          >
            <Menu className="h-5 w-5" />
          </Button>

          <h1 className="text-foreground text-base font-semibold tracking-tight">
            {pageTitle}
          </h1>
        </div>

        {/* Right: Notification, Theme Toggle & User Menu */}
        <div className="flex items-center gap-2">
          <ThemeToggle />
          <NotificationBell />
          <NavUser />
        </div>
      </header>

      <MobileNav open={mobileNavOpen} onOpenChange={setMobileNavOpen} />
    </>
  );
}
