"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { useQuery } from "@tanstack/react-query";
import { format, parseISO } from "date-fns";
import { id } from "date-fns/locale";
import { AlertCircle, Boxes, Calendar as CalendarIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { cn } from "@/lib/utils";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { apiFetch } from "@/lib/client/api";
import { assetKeys } from "@/lib/query-key";
import { setFormErrors } from "@/lib/client/error-mapper";
import { useApiMutation } from "@/hooks/use-api-mutation";
import {
  assetSchema,
  ASSET_FORM_STATUSES,
  type AssetFormData,
} from "@/schemas/asset";
import type { AssetDetail, AssetStatus } from "@/types/assets";

interface AssetFormProps {
  asset?: AssetDetail;
}

export function AssetForm({ asset }: AssetFormProps) {
  const router = useRouter();
  const isEdit = Boolean(asset);

  const {
    register,
    handleSubmit,
    control,
    setError,
    formState: { errors },
  } = useForm<AssetFormData>({
    resolver: zodResolver(assetSchema),
    defaultValues: {
      asset_tag: asset?.asset_tag ?? "",
      name: asset?.name ?? "",
      category: asset?.category ?? "",
      brand: asset?.brand ?? "",
      model: asset?.model ?? "",
      serial_number: asset?.serial_number ?? "",
      purchase_date: asset?.purchase_date ?? "",
      status: (asset?.status ?? "available") as AssetStatus,
      notes: asset?.notes ?? "",
    },
  });

  const [categoryMode, setCategoryMode] = useState<"select" | "free">("select");
  const [selectedCategory, setSelectedCategory] = useState<string>(
    asset?.category ?? ""
  );

  const { data: categoriesResponse } = useQuery({
    queryKey: assetKeys.categories,
    queryFn: () => apiFetch<string[]>("/assets/categories"),
    staleTime: 5 * 60 * 1000,
  });
  const categories = categoriesResponse?.data ?? [];

  const mutation = useApiMutation({
    mutationFn: (data: AssetFormData) => {
      const body = {
        ...data,
        category: categoryMode === "free" ? selectedCategory : data.category,
        notes: data.notes || null,
      };
      return apiFetch<AssetDetail>(
        isEdit ? `/assets/${asset!.id}` : "/assets",
        {
          method: isEdit ? "PUT" : "POST",
          body: JSON.stringify(body),
        }
      );
    },
    onSuccessMessage: isEdit
      ? "Aset berhasil diperbarui."
      : "Aset berhasil ditambahkan.",
    onFormError: (backendErrors) => {
      setFormErrors(backendErrors, setError);
    },
    onSuccess: (res) => {
      router.push(`/assets/${res.data?.id}`);
    },
  });

  const statusLabels: Record<string, string> = {
    available: "Tersedia",
    maintenance: "Perawatan",
    retired: "Pensiun",
    lost: "Hilang",
  };

  return (
    <form
      onSubmit={handleSubmit((data) => mutation.mutate(data))}
      className="grid grid-cols-1 gap-8 lg:grid-cols-12"
    >
      {/* Kiri: Field Form */}
      <div className="space-y-6 lg:col-span-8">
        <div className="border-border bg-card space-y-5 rounded-xl border p-6 shadow-xs">
          {/* Tag & Name */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div className="space-y-1.5">
              <Label htmlFor="asset_tag">
                Kode Aset <span className="text-destructive">*</span>
              </Label>
              <Input
                id="asset_tag"
                placeholder="Contoh: LPT-0001"
                className="h-11 text-base sm:h-9 sm:text-sm"
                {...register("asset_tag")}
                aria-invalid={!!errors.asset_tag}
              />
              {errors.asset_tag && (
                <p className="text-destructive text-xs">
                  {errors.asset_tag.message}
                </p>
              )}
            </div>
            <div className="space-y-1.5">
              <Label htmlFor="name">
                Nama Aset <span className="text-destructive">*</span>
              </Label>
              <Input
                id="name"
                placeholder="Contoh: ThinkPad T14"
                className="h-11 text-base sm:h-9 sm:text-sm"
                {...register("name")}
                aria-invalid={!!errors.name}
              />
              {errors.name && (
                <p className="text-destructive text-xs">
                  {errors.name.message}
                </p>
              )}
            </div>
          </div>

          {/* Kategori */}
          <div className="space-y-1.5">
            <Label htmlFor="category">
              Kategori <span className="text-destructive">*</span>
            </Label>
            {categoryMode === "select" ? (
              <div className="flex gap-2">
                <Controller
                  control={control}
                  name="category"
                  render={({ field }) => (
                    <Select
                      value={field.value}
                      onValueChange={(v) => {
                        field.onChange(v);
                        setSelectedCategory(v);
                      }}
                    >
                      <SelectTrigger
                        className="h-11 flex-1 text-base sm:h-9 sm:text-sm"
                        aria-invalid={!!errors.category}
                      >
                        <SelectValue placeholder="Pilih kategori…" />
                      </SelectTrigger>
                      <SelectContent>
                        {categories.map((c) => (
                          <SelectItem key={c} value={c}>
                            {c}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  )}
                />
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => setCategoryMode("free")}
                  className="shrink-0"
                >
                  Kategori lain
                </Button>
              </div>
            ) : (
              <div className="flex gap-2">
                <Input
                  value={selectedCategory}
                  onChange={(e) => setSelectedCategory(e.target.value)}
                  placeholder="Ketik kategori baru…"
                  className="h-11 flex-1 text-base sm:h-9 sm:text-sm"
                />
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  onClick={() => setCategoryMode("select")}
                  className="shrink-0"
                >
                  Pilih dari daftar
                </Button>
              </div>
            )}
            {errors.category && (
              <p className="text-destructive text-xs">
                {errors.category.message}
              </p>
            )}
          </div>

          {/* Brand dan Model */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div className="space-y-1.5">
              <Label htmlFor="brand">
                Merek <span className="text-destructive">*</span>
              </Label>
              <Input
                id="brand"
                placeholder="Contoh: Lenovo"
                className="h-11 text-base sm:h-9 sm:text-sm"
                {...register("brand")}
                aria-invalid={!!errors.brand}
              />
              {errors.brand && (
                <p className="text-destructive text-xs">
                  {errors.brand.message}
                </p>
              )}
            </div>
            <div className="space-y-1.5">
              <Label htmlFor="model">
                Model <span className="text-destructive">*</span>
              </Label>
              <Input
                id="model"
                placeholder="Contoh: T14 Gen 3"
                className="h-11 text-base sm:h-9 sm:text-sm"
                {...register("model")}
                aria-invalid={!!errors.model}
              />
              {errors.model && (
                <p className="text-destructive text-xs">
                  {errors.model.message}
                </p>
              )}
            </div>
          </div>

          {/* Serial & Purchase Date/Tanggal Pembelian */}
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <div className="space-y-1.5">
              <Label htmlFor="serial_number">
                Nomor Seri <span className="text-destructive">*</span>
              </Label>
              <Input
                id="serial_number"
                placeholder="Contoh: PF3ABCDE"
                className="h-11 text-base sm:h-9 sm:text-sm"
                {...register("serial_number")}
                aria-invalid={!!errors.serial_number}
              />
              {errors.serial_number && (
                <p className="text-destructive text-xs">
                  {errors.serial_number.message}
                </p>
              )}
            </div>
            <div className="space-y-1.5">
              <Label htmlFor="purchase_date">
                Tanggal Pembelian <span className="text-destructive">*</span>
              </Label>
              <Controller
                control={control}
                name="purchase_date"
                render={({ field }) => {
                  const selectedDate = field.value
                    ? parseISO(field.value)
                    : undefined;
                  return (
                    <Popover>
                      <PopoverTrigger asChild>
                        <Button
                          id="purchase_date"
                          type="button"
                          variant="outline"
                          aria-label={
                            field.value
                              ? `Tanggal pembelian: ${format(selectedDate!, "d MMM yyyy", { locale: id })}`
                              : "Pilih tanggal pembelian"
                          }
                          aria-invalid={!!errors.purchase_date}
                          className={cn(
                            "border-input h-11 w-full justify-start rounded-lg bg-transparent px-3 text-left text-base font-normal shadow-xs sm:h-9 sm:text-sm",
                            !field.value && "text-muted-foreground",
                            errors.purchase_date &&
                              "border-destructive ring-destructive/20"
                          )}
                        >
                          <CalendarIcon className="mr-2 h-4 w-4 shrink-0 opacity-70" />
                          <span>
                            {field.value && selectedDate
                              ? format(selectedDate, "d MMM yyyy", {
                                  locale: id,
                                })
                              : "Pilih tanggal pembelian…"}
                          </span>
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-auto p-0" align="start">
                        <Calendar
                          mode="single"
                          selected={selectedDate}
                          onSelect={(date) => {
                            if (date) {
                              field.onChange(format(date, "yyyy-MM-dd"));
                            } else {
                              field.onChange("");
                            }
                          }}
                          disabled={(date) => date > new Date()}
                          defaultMonth={selectedDate ?? new Date()}
                          captionLayout="dropdown"
                          startMonth={new Date(2000, 0)}
                          endMonth={new Date()}
                        />
                      </PopoverContent>
                    </Popover>
                  );
                }}
              />
              {errors.purchase_date && (
                <p className="text-destructive text-xs">
                  {errors.purchase_date.message}
                </p>
              )}
            </div>
          </div>

          {/* Status */}
          <div className="space-y-1.5">
            <Label htmlFor="status">
              Status <span className="text-destructive">*</span>
            </Label>
            <Controller
              control={control}
              name="status"
              render={({ field }) => (
                <Select value={field.value} onValueChange={field.onChange}>
                  <SelectTrigger
                    id="status"
                    className="h-11 text-base sm:h-9 sm:text-sm"
                    aria-invalid={!!errors.status}
                  >
                    <SelectValue placeholder="Pilih status…" />
                  </SelectTrigger>
                  <SelectContent>
                    {ASSET_FORM_STATUSES.map((s) => (
                      <SelectItem key={s} value={s}>
                        {statusLabels[s]}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              )}
            />
            {errors.status && (
              <p className="text-destructive text-xs">
                {errors.status.message}
              </p>
            )}
          </div>

          {/* Notes/Catatan */}
          <div className="space-y-1.5">
            <Label htmlFor="notes">Catatan</Label>
            <Textarea
              id="notes"
              rows={3}
              placeholder="Catatan tambahan (opsional)…"
              {...register("notes")}
            />
          </div>
        </div>

        {/* Actions/Aksi */}
        <div className="flex items-center justify-end gap-3">
          <Button type="button" variant="outline" asChild>
            <Link href="/assets">Batal</Link>
          </Button>
          <Button type="submit" disabled={mutation.isPending}>
            {mutation.isPending
              ? "Menyimpan…"
              : isEdit
                ? "Simpan Perubahan"
                : "Tambah Aset"}
          </Button>
        </div>
      </div>

      {/* Kanan: Sidebar Info */}
      <div className="space-y-6 lg:col-span-4">
        <div className="border-border bg-card space-y-3 rounded-xl border p-5 shadow-xs">
          <div className="text-foreground flex items-center gap-2 text-sm font-semibold">
            <Boxes className="text-primary h-4 w-4" />
            Informasi Status
          </div>
          <div className="bg-muted/40 text-muted-foreground flex items-start gap-2 rounded-lg p-3 text-xs">
            <AlertCircle className="text-muted-foreground mt-0.5 h-4 w-4 shrink-0" />
            <span>
              Status <strong>Ditugaskan</strong> dikelola otomatis melalui alur
              assign/release dan tidak tersedia pada formulir ini.
            </span>
          </div>
          <ul className="text-muted-foreground space-y-2 text-xs">
            <li>
              <strong className="text-foreground">Tersedia</strong> — aset siap
              ditugaskan.
            </li>
            <li>
              <strong className="text-foreground">Perawatan</strong> — sedang
              diperbaiki.
            </li>
            <li>
              <strong className="text-foreground">Pensiun</strong> — tidak lagi
              digunakan.
            </li>
            <li>
              <strong className="text-foreground">Hilang</strong> — lokasi tidak
              diketahui.
            </li>
          </ul>
        </div>
      </div>
    </form>
  );
}
