"use client";

import { AuthSplitShell } from "@/components/auth/auth-split-shell";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { translateValidationError } from "@/lib/client/error-mapper";
import { ApiResponse, AuthUser, ApiError } from "@/types/api";
import { zodResolver } from "@hookform/resolvers/zod";
import { AlertCircle, Eye, EyeOff, Lock, Mail } from "lucide-react";
import { useRouter } from "next/navigation";
import { error } from "node:console";
import { useState } from "react";
import { useForm } from "react-hook-form";
import z from "zod";

const loginSchema = z.object({
  email: z.string().min(1, "Email wajib diisi.").email("Email tidak valid."),
  password: z.string().min(1, "Password wajib diisi."),
});

type LoginFormValues = z.infer<typeof loginSchema>;

export default function LoginPage() {
  const router = useRouter();
  const [generalError, setGeneralError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [showPassword, setShowPassword] = useState(false);

  const form = useForm<LoginFormValues>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  async function onSubmit(values: LoginFormValues) {
    setGeneralError(null);
    setLoading(true);

    try {
      const response = await fetch("/api/auth/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(values),
      });

      const payload = (await response.json()) as
        ApiResponse<{ user: AuthUser }> | ApiError;

      if (!response.ok || !payload.success) {
        const errorPayload = payload as ApiError;
        if (errorPayload.errors) {
          const firstError = Object.values(errorPayload.errors)[0]?.[0];
          setGeneralError(
            firstError
              ? translateValidationError(errorPayload.message)
              : errorPayload.message
          );
        } else {
          setGeneralError(
            errorPayload.message
              ? translateValidationError(errorPayload.message)
              : "Email atau kata sandi salah."
          );
        }
        return;
      }

      const userData = (payload as ApiResponse<{ user: AuthUser }>).data?.user;
      if (userData?.must_change_password) {
        router.push("/ganti-password");
      } else {
        router.push("/");
      }
    } catch {
      setGeneralError("Tidak dapat terhubung ke server. Silakan coba lagi.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <AuthSplitShell
      headline="Satu tempat untuk seluruh permintaan IT perusahaan."
      subhead="Lapor, tangani, dan pantau SLA tanpa kehilangan jejak."
      noticeTitle="Sistem internal"
      noticeBody="Tidak ada pendaftaran mandiri. Akun dibuat oleh Administrator."
      fieldNote="Akun tidak aktif tidak dapat masuk. Hubungi Administrator."
      cardWidthClass="max-w-[400px]"
    >
      <div className="border-border bg-card rounded-xl border p-8 shadow-xs">
        <h2 className="text-foreground text-[22px] font-semibold tracking-tight">
          Masuk
        </h2>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="mt-5 space-y-4"
          >
            <FormField
              control={form.control}
              name="email"
              render={({ field }) => (
                <FormItem className="space-y-1.5">
                  <FormLabel className="text-foreground text-xs font-medium">
                    Email
                  </FormLabel>
                  <div className="relative flex items-center">
                    <Mail
                      aria-hidden="true"
                      className="text-muted-foreground pointer-events-none absolute left-3.5 h-4 w-4"
                    />
                    <FormControl>
                      <Input
                        type="email"
                        placeholder="nama@perusahaan.co.id"
                        autoComplete="email"
                        disabled={loading}
                        className="bg-background placeholder:text-muted-foreground focus-visible:ring-ring h-10.5 rounded-md pl-10 text-sm focus-visible:ring-offset-0"
                        {...field}
                      />
                    </FormControl>
                  </div>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem className="space-y-1.5">
                  <FormLabel className="text-foreground text-xs font-medium">
                    Kata Sandi
                  </FormLabel>
                  <div className="relative flex items-center">
                    <Lock
                      aria-hidden="true"
                      className="text-muted-foreground pointer-events-none absolute left-3.5 h-4 w-4"
                    />
                    <FormControl>
                      <Input
                        type={showPassword ? "text" : "password"}
                        placeholder="••••••••"
                        autoComplete="current-password"
                        disabled={loading}
                        className="bg-background placeholder:text-muted-foreground focus-visible:ring-ring h-10.5 rounded-md pr-10 pl-10 text-sm focus-visible:ring-offset-0"
                        {...field}
                      />
                    </FormControl>
                    <button
                      type="button"
                      aria-label={
                        showPassword
                          ? "Sembunyikan kata sandi"
                          : "Tampilkan kata sandi"
                      }
                      onClick={() => setShowPassword((prev) => !prev)}
                      className="text-muted-foreground hover:text-foreground focus-visible:ring-ring absolute right-3.5 flex h-5 w-5 items-center justify-center rounded-sm transition-colors focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:outline-none"
                    >
                      {showPassword ? (
                        <EyeOff aria-hidden="true" className="h-4 w-4" />
                      ) : (
                        <Eye aria-hidden="true" className="h-4 w-4" />
                      )}
                    </button>
                  </div>
                  <FormMessage />
                </FormItem>
              )}
            />

            <Button
              type="submit"
              disabled={loading}
              className="bg-primary text-primary-foreground h-11 w-full rounded-md text-sm font-semibold shadow-[rgba(255,255,255,0.2)_0px_0.5px_0px_0px_inset,rgba(0,0,0,0.2)_0px_0px_0px_0.5px_inset] transition-opacity hover:opacity-90 active:opacity-80"
            >
              {loading ? "Memproses…" : "Masuk"}
            </Button>

            {generalError && (
              <div
                role="alert"
                className="border-destructive/30 bg-destructive/10 text-destructive flex items-start gap-2.5 rounded-md border p-3 text-xs leading-relaxed"
              >
                <AlertCircle
                  aria-hidden="true"
                  className="text-destructive mt-0.5 h-4 w-4 shrink-0"
                />
                <span>{generalError}</span>
              </div>
            )}
          </form>
        </Form>
      </div>
    </AuthSplitShell>
  );
}
