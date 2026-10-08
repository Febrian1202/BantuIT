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
import { ApiClientError, apiFetch } from "@/lib/client/api";
import { setFormErrors } from "@/lib/client/error-mapper";
import {
  type ChangePasswordFormData,
  changePasswordSchema,
} from "@/schemas/profile";
import { zodResolver } from "@hookform/resolvers/zod";
import { AlertCircle, Eye, EyeOff, Lock } from "lucide-react";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useForm } from "react-hook-form";

export default function GantiPasswordPage() {
  const router = useRouter();
  const [generalError, setGeneralError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [showCurrentPassword, setShowCurrentPassword] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirmPassword, setShowConfirmPassword] = useState(false);

  // Inisialisasi form menggunakan react-hook-form dengan validasi menggunakan zod
  const form = useForm<ChangePasswordFormData>({
    resolver: zodResolver(changePasswordSchema),
    defaultValues: {
      current_password: "",
      password: "",
      password_confirmation: "",
    },
  });

  async function onSubmit(values: ChangePasswordFormData) {
    setGeneralError(null);
    setLoading(true);

    try {
      await apiFetch<null>("/me/password", {
        method: "PUT",
        body: JSON.stringify(values),
      });

      router.push("/");
    } catch (err) {
      if (err instanceof ApiClientError) {
        if (err.errors) {
          setFormErrors(err.errors, form.setError);
        } else {
          setGeneralError(err.message);
        }
      } else {
        setGeneralError("Gagal memperbarui kata sandi, Silakan coba lagi.");
      }
    } finally {
      setLoading(false);
    }
  }

  return (
    <AuthSplitShell
      headline="Pembaruan Kata Sandi Wajib"
      subhead="Akun Anda baru saja diatur ulang oleh Administrator. Demi keamanan, Anda wajib membuat kata sandi baru sebelum dapat mengakses sistem."
      noticeTitle="Ketentuan Kata Sandi"
      noticeBody="Minimal 8 karakter, kombinasi huruf dan angka. Jangan gunakan kata sandi lama."
      cardWidthClass="max-w-[420px]"
    >
      <div className="border-border bg-card rounded-xl border p-8 shadow-xs">
        <h2 className="text-foreground text-[20px] font-semibold tracking-tight">
          Buat Kata Sandi Baru
        </h2>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="mt-5 space-y-4"
          >
            <FormField
              control={form.control}
              name="current_password"
              render={({ field }) => (
                <FormItem className="space-y-1.5">
                  <FormLabel className="text-foreground text-xs font-medium">
                    Kata Sandi Saat Ini / Sementara
                  </FormLabel>
                  <div className="relative flex items-center">
                    <Lock
                      aria-hidden="true"
                      className="text-muted-foreground pointer-events-none absolute left-3.5 h-4 w-4"
                    />
                    <FormControl>
                      <Input
                        type={showCurrentPassword ? "text" : "password"}
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
                        showCurrentPassword
                          ? "Sembunyikan kata sandi saat ini"
                          : "Tampilkan kata sandi saat ini"
                      }
                      onClick={() => setShowCurrentPassword((prev) => !prev)}
                      className="text-muted-foreground hover:text-foreground focus-visible:ring-ring absolute right-3.5 flex h-5 w-5 items-center justify-center rounded-sm transition-colors focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:outline-none"
                    >
                      {showCurrentPassword ? (
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

            <FormField
              control={form.control}
              name="password"
              render={({ field }) => (
                <FormItem className="space-y-1.5">
                  <FormLabel className="text-foreground text-xs font-medium">
                    Kata Sandi Baru
                  </FormLabel>
                  <div className="relative flex items-center">
                    <Lock
                      aria-hidden="true"
                      className="text-muted-foreground pointer-events-none absolute left-3.5 h-4 w-4"
                    />
                    <FormControl>
                      <Input
                        type={showPassword ? "text" : "password"}
                        placeholder="Minimal 8 karakter, kombinasi huruf & angka"
                        autoComplete="new-password"
                        disabled={loading}
                        className="bg-background placeholder:text-muted-foreground focus-visible:ring-ring h-10.5 rounded-md pr-10 pl-10 text-sm focus-visible:ring-offset-0"
                        {...field}
                      />
                    </FormControl>
                    <button
                      type="button"
                      aria-label={
                        showPassword
                          ? "Sembunyikan kata sandi baru"
                          : "Tampilkan kata sandi baru"
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

            <FormField
              control={form.control}
              name="password_confirmation"
              render={({ field }) => (
                <FormItem className="space-y-1.5">
                  <FormLabel className="text-foreground text-xs font-medium">
                    Konfirmasi Kata Sandi Baru
                  </FormLabel>
                  <div className="relative flex items-center">
                    <Lock
                      aria-hidden="true"
                      className="text-muted-foreground pointer-events-none absolute left-3.5 h-4 w-4"
                    />
                    <FormControl>
                      <Input
                        type={showConfirmPassword ? "text" : "password"}
                        placeholder="Ulangi kata sandi baru"
                        autoComplete="new-password"
                        disabled={loading}
                        className="bg-background placeholder:text-muted-foreground focus-visible:ring-ring h-10.5 rounded-md pr-10 pl-10 text-sm focus-visible:ring-offset-0"
                        {...field}
                      />
                    </FormControl>
                    <button
                      type="button"
                      aria-label={
                        showConfirmPassword
                          ? "Sembunyikan konfirmasi kata sandi"
                          : "Tampilkan konfirmasi kata sandi"
                      }
                      onClick={() => setShowConfirmPassword((prev) => !prev)}
                      className="text-muted-foreground hover:text-foreground focus-visible:ring-ring absolute right-3.5 flex h-5 w-5 items-center justify-center rounded-sm transition-colors focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:outline-none"
                    >
                      {showConfirmPassword ? (
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
              {loading ? "Menyimpan…" : "Simpan & Lanjutkan ke Sistem"}
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
