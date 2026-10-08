"use client";

import { usePathname } from "next/navigation";
import { useAuth } from "@/components/providers/auth-provider";
import { filterNavItems } from "@/lib/navigation";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Computer } from "lucide-react";
import Link from "next/link";
import { cn } from "@/lib/utils";

interface MobileNavProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function MobileNav({ open, onOpenChange }: MobileNavProps) {
  const pathname = usePathname();
  const { user } = useAuth();

  const permissions = user?.permissions ?? [];
  const role = user?.role?.name;
  const navItems = filterNavItems(permissions, role);

  const mainItems = navItems.filter((item) => item.section === "main");
  const adminItems = navItems.filter((item) => item.section === "admin");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="border-border bg-card data-[state=open]:animate-sheet-in-left data-[state=closed]:animate-sheet-out-left fixed top-0 right-auto bottom-0 left-0 z-50 flex h-full w-72 max-w-none translate-x-0 translate-y-0 flex-col gap-0 rounded-none border-r p-0 shadow-xl">
        <DialogHeader className="border-border flex h-14 shrink-0 flex-row items-center space-y-0 border-b px-4 text-left">
          <DialogTitle className="flex items-center pr-8">
            <Computer size="sm" />
          </DialogTitle>
        </DialogHeader>

        <div className="flex-1 space-y-5 overflow-y-auto px-3 py-3">
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
                    onClick={() => onOpenChange(false)}
                    className={cn(
                      "flex min-h-[44px] items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors",
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
                      onClick={() => onOpenChange(false)}
                      className={cn(
                        "flex min-h-[44px] items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors",
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

        <div className="border-border shrink-0 border-t p-3">
          <div className="bg-muted/50 rounded-lg p-2 text-center">
            <p className="text-muted-foreground text-[11px]">
              JARVIS OPS{" "}
              {(() => {
                const rawVersion =
                  process.env.NEXT_PUBLIC_APP_VERSION || "v1.0.0";
                return rawVersion.startsWith("v")
                  ? rawVersion
                  : `v${rawVersion}`;
              })()}
            </p>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
