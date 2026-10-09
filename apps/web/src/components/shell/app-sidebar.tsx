"use client";

import { usePathname } from "next/navigation";
import { useAuth } from "../providers/auth-provider";
import { filterNavItems } from "@/lib/navigation";
import Link from "next/link";
import { cn } from "@/lib/utils";

export function AppSidebar() {
  const pathname = usePathname();
  const { user } = useAuth();

  const permissions = user?.permissions ?? [];
  const role = user?.role?.name;
  const navItems = filterNavItems(permissions, role);

  const mainItems = navItems.filter((item) => item.section === "main");
  const adminItems = navItems.filter((item) => item.section === "admin");

  return (
    <aside className="border-border bg-card fixed inset-y-0 left-0 z-30 hidden w-64 flex-col border-r transition-transform md:flex">
      {/* Brand Header */}
      <div className="border-border flex h-14 items-center border-b px-5">
        <Link
          href="/"
          className="transition-opacity hover:opacity-90 focus-visible:outline-hidden"
        >
          {/*<BrandLogo size="sm" />*/}
          BantuIT
        </Link>
      </div>

      {/* Navigation List */}
      <div className="flex-1 space-y-6 overflow-y-auto px-3 py-4">
        <div>
          <p className="text-muted-foreground mb-2 px-3 text-xs font-medium tracking-wider uppercase">
            Menu Utama
          </p>
          <nav className="space-y-1">
            {mainItems.map((item) => {
              const isActive =
                item.href === "/"
                  ? pathname === "/"
                  : pathname.startsWith(item.href);
              const Icon = item.icon;

              return (
                <Link
                  key={item.href}
                  href={item.href}
                  className={cn(
                    "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                    isActive
                      ? "bg-secondary text-foreground font-semibold"
                      : "text-muted-foreground hover:bg-secondary/60 hover:text-foreground"
                  )}
                >
                  <Icon className="h-4 w-4 shrink-0" />
                  <span>{item.title}</span>
                </Link>
              );
            })}
          </nav>
        </div>

        {adminItems.length > 0 && (
          <div>
            <p className="text-muted-foreground mb-2 px-3 text-xs font-medium tracking-wider uppercase">
              Administrasi
            </p>
            <nav className="space-y-1">
              {adminItems.map((item) => {
                const isActive = pathname.startsWith(item.href);
                const Icon = item.icon;

                return (
                  <Link
                    key={item.href}
                    href={item.href}
                    className={cn(
                      "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                      isActive
                        ? "bg-secondary text-foreground font-semibold"
                        : "text-muted-foreground hover:bg-secondary/60 hover:text-foreground"
                    )}
                  >
                    <Icon className="h-4 w-4 shrink-0" />
                    <span>{item.title}</span>
                  </Link>
                );
              })}
            </nav>
          </div>
        )}
      </div>

      {/* Sidebar Footer */}
      <div className="border-border border-t p-3">
        <div className="bg-muted/50 rounded-lg p-2 text-center">
          <p className="text-muted-foreground text-[11px]">
            JARVIS OPS{" "}
            {(() => {
              const rawVersion =
                process.env.NEXT_PUBLIC_APP_VERSION || "v1.0.0";
              return rawVersion.startsWith("v") ? rawVersion : `v${rawVersion}`;
            })()}
          </p>
        </div>
      </div>
    </aside>
  );
}
