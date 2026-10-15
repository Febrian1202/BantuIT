import { cn } from "@/lib/utils";
import { Briefcase, ShieldCheck, User, Wrench } from "lucide-react";

export interface DemoAccount {
  role: string;
  label: string;
  email: string;
  icon: typeof ShieldCheck;
  colorClass: string;
}

export const DEMO_ACCOUNTS: DemoAccount[] = [
  {
    role: "Administrator",
    label: "Admin Sistem",
    email: "admin@bantuit.test",
    icon: ShieldCheck,
    colorClass: "text-blue-600 dark:text-blue-400",
  },
  {
    role: "Manager",
    label: "IT Manager",
    email: "manager@bantuit.test",
    icon: Briefcase,
    colorClass: "text-purple-600 dark:text-purple-400",
  },
  {
    role: "Technician",
    label: "Teknisi IT",
    email: "technician@bantuit.test",
    icon: Wrench,
    colorClass: "text-emerald-600 dark:text-emerald-400",
  },
  {
    role: "Employee",
    label: "Karyawan",
    email: "employee@bantuit.test",
    icon: User,
    colorClass: "text-amber-600 dark:text-amber-400",
  },
];

interface DemoAccountCardsProps {
  onSelect: (email: string) => void;
  disabled?: boolean;
}

export function DemoAccountCards({
  onSelect,
  disabled,
}: DemoAccountCardsProps) {
  return (
    <div className="space-y-3">
      {/* Divider Akun Demo */}
      <div className="relative my-3 flex items-center justify-center">
        <div className="border-border absolute inset-0 flex items-center">
          <div className="w-full border-t" />
        </div>
        <div className="bg-card text-muted-foreground relative px-2.5 text-[11px] font-medium tracking-wider uppercase">
          Akun Demo (1-Click Fill)
        </div>
      </div>

      {/* Grid Kartu Akun Demo */}
      <div className="grid grid-cols-2 gap-2 sm:gap-2.5">
        {DEMO_ACCOUNTS.map((acc) => {
          const Icon = acc.icon;
          return (
            <button
              key={acc.email}
              type="button"
              disabled={disabled}
              onClick={() => onSelect(acc.email)}
              aria-label={`Pilih akun ${acc.role}: ${acc.email}`}
              className="border-border bg-background/60 hover:bg-secondary/70 hover:border-primary/40 focus-visible:ring-ring group flex cursor-pointer flex-col items-start gap-1 rounded-lg border p-2.5 text-left transition-all focus-visible:ring-2 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
            >
              <div className="flex w-full items-center justify-between">
                <span className="text-foreground group-hover:text-primary flex items-center gap-1.5 text-xs font-semibold transition-colors">
                  <Icon
                    className={cn("size-3.5 shrink-0", acc.colorClass)}
                    aria-hidden="true"
                  />
                  {acc.role}
                </span>
                <span className="text-muted-foreground text-[10px] font-medium">
                  {acc.label}
                </span>
              </div>
              <span className="text-muted-foreground group-hover:text-foreground w-full truncate font-mono text-[11px] transition-colors">
                {acc.email}
              </span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
