export interface DemoAccount {
  role: string;
  label: string;
  email: string;
}

export const DEMO_ACCOUNTS: DemoAccount[] = [
  {
    role: "Administrator",
    label: "Admin Sistem",
    email: "admin@bantuit.test",
  },
  {
    role: "Manager",
    label: "IT Manager",
    email: "manager@bantuit.test",
  },
  {
    role: "Technician",
    label: "Teknisi IT",
    email: "technician@bantuit.test",
  },
  {
    role: "Employee",
    label: "Karyawan",
    email: "employee@bantuit.test",
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

      {/* List Kartu Akun Demo (1 baris per kartu) */}
      <div className="flex flex-col gap-2">
        {DEMO_ACCOUNTS.map((acc) => (
          <button
            key={acc.email}
            type="button"
            disabled={disabled}
            onClick={() => onSelect(acc.email)}
            aria-label={`Pilih akun ${acc.role}: ${acc.email}`}
            className="border-border bg-background/50 hover:bg-secondary/70 hover:border-primary/40 focus-visible:ring-ring group flex cursor-pointer items-center justify-between rounded-lg border px-3 py-2 text-left transition-all focus-visible:ring-2 focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50"
          >
            <div className="flex min-w-0 flex-col">
              <span className="text-foreground group-hover:text-primary truncate text-xs font-semibold transition-colors">
                {acc.role}
                <span className="text-muted-foreground ml-1.5 text-[11px] font-normal">
                  ({acc.label})
                </span>
              </span>
              <span className="text-muted-foreground truncate font-mono text-[11px]">
                {acc.email}
              </span>
            </div>
            <span className="text-muted-foreground group-hover:text-primary hidden shrink-0 text-[11px] font-medium opacity-0 transition-opacity group-hover:opacity-100 sm:inline-block">
              Pilih →
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}
