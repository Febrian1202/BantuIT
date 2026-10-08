import { cn } from "@/lib/utils";
import { Computer } from "lucide-react";
import { ReactNode } from "react";

export interface AuthSplitShellProps {
  headline: string;
  subhead: string;
  noticeTitle: string;
  noticeBody: string;
  children: ReactNode;
  fieldNote?: string;
  cardWidthClass?: string;
}

export function AuthSplitShell({
  headline,
  subhead,
  noticeTitle,
  noticeBody,
  children,
  fieldNote,
  cardWidthClass = "max-w-[400px]",
}: AuthSplitShellProps) {
  return (
    <div className="bg-background text-foreground flex min-h-screen w-full flex-col lg:flex-row">
      {/* Brand Panel */}
      <aside
        aria-label="Informasi Sistem"
        className="border-border bg-secondary/40 flex w-full flex-col justify-between border-b p-6 sm:p-8 lg:w-140 lg:shrink-0 lg:border-r lg:border-b-0 lg:p-14"
      >
        {/* Lockup */}
        <div className="inline-flex items-center">
          <Computer size="md" />
        </div>

        {/* Statement */}
        <div className="my-8 space-y-4 lg:my-0 lg:space-y-4.5">
          <h1 className="text-foreground text-2xl font-semibold tracking-tight sm:text-3xl lg:text-[30px] lg:leading-tight">
            {headline}
          </h1>
          <p className="text-muted-foreground text-sm leading-relaxed sm:text-base lg:text-[14px]">
            {subhead}
          </p>
        </div>

        {/* Notice Box */}
        <div
          role="note"
          className="border-border bg-card rounded-lg border p-4 shadow-xs"
        >
          <p className="text-foreground text-xs font-semibold">{noticeTitle}</p>
          <p className="text-muted-foreground mt-1 text-xs leading-relaxed">
            {noticeBody}
          </p>
        </div>
      </aside>

      {/* Form Column */}
      <main className="flex flex-1 flex-col items-center justify-center p-6 sm:p-8 lg:p-14">
        <div className={cn("w-full", cardWidthClass)}>
          {children}

          {fieldNote && (
            <p className="text-muted-foreground mt-4 text-center text-xs leading-normal">
              {fieldNote}
            </p>
          )}
        </div>
      </main>
    </div>
  );
}
