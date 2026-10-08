import { AuthProvider } from "@/components/providers/auth-provider";
import { QueryProvider } from "@/components/providers/query-provider";
import { AppSidebar } from "@/components/shell/app-sidebar";
import { AppTopbar } from "@/components/shell/app-topbar";
import { Toaster } from "@/components/ui/sonner";
import { ReactNode } from "react";

export default function AppLayout({ children }: { children: ReactNode }) {
  return (
    <QueryProvider>
      <AuthProvider>
        <div className="bg-background text-foreground flex min-h-screen">
          <AppSidebar />
          <div className="flex flex-1 flex-col md:pl-64">
            <AppTopbar />
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
