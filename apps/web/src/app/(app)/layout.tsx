import { AuthProvider } from "@/components/providers/auth-provider";
import { QueryProvider } from "@/components/providers/query-provider";
import { AppSidebar } from "@/components/shell/app-sidebar";
import { AppTopbar } from "@/components/shell/app-topbar";
import { Toaster } from "@/components/ui/sonner";
import { ReactNode } from "react";
import { formatAppVersion } from "@/lib/version";

export default function AppLayout({ children }: { children: ReactNode }) {
  const appVersion = formatAppVersion(
    process.env.NEXT_PUBLIC_APP_VERSION || process.env.APP_VERSION
  );

  return (
    <QueryProvider>
      <AuthProvider>
        <div className="bg-background text-foreground flex min-h-screen">
          <AppSidebar appVersion={appVersion} />
          <div className="flex flex-1 flex-col md:pl-64">
            <AppTopbar appVersion={appVersion} />
            <main className="mx-auto w-full max-w-7xl flex-1 p-4 sm:p-6 lg:p-8">
              {children}
            </main>
          </div>
        </div>
        <Toaster />
      </AuthProvider>
    </QueryProvider>
  );
}
