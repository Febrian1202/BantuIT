"use client";

import { useState } from "react";
import { FilterField } from "./filter-bar";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";
import { Check, RotateCcw, SlidersHorizontal } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";

interface MobileFilterSheetProps {
  filters: FilterField[];
  onFilterChange?: (filterId: string, value: string) => void;
  onResetFilters?: () => void;
  hasActiveFilters?: boolean;
}

export function MobileFilterSheet({
  filters,
  onFilterChange,
  onResetFilters,
  hasActiveFilters = false,
}: MobileFilterSheetProps) {
  const [open, setOpen] = useState(false);

  const activeCount = filters.filter((f) =>
    Boolean(f.value && f.value !== "ALL")
  ).length;

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetTrigger asChild>
        <Button
          variant="outline"
          size="sm"
          className="border-border bg-card relative flex h-11 min-h-11 items-center gap-2 px-3 text-xs font-medium"
        >
          <SlidersHorizontal className="text-muted-foreground h-4 w-4" />
          <span>Filter</span>
          {activeCount > 0 && (
            <Badge
              variant="secondary"
              className="bg-primary text-primary-foreground h-5 min-w-5 rounded-full px-1.5 text-[10px] font-semibold"
            >
              {activeCount}
            </Badge>
          )}
        </Button>
      </SheetTrigger>

      <SheetContent side="bottom" className="space-y-4 pb-6">
        <SheetHeader className="border-border border-b pb-3 text-left">
          <SheetTitle className="text-base font-semibold">
            Filter Data
          </SheetTitle>
          <SheetDescription className="text-muted-foreground text-xs">
            Sesuaikan parameter untuk memfilter daftar.
          </SheetDescription>
        </SheetHeader>

        <div className="space-y-4 py-2">
          {filters.map((filter) => (
            <div key={filter.id} className="space-y-1.5">
              <label className="text-foreground text-xs font-semibold">
                {filter.label}
              </label>
              <Select
                value={filter.value || "ALL"}
                onValueChange={(val) => {
                  if (onFilterChange) {
                    onFilterChange(filter.id, val === "ALL" ? "" : val);
                  }
                }}
              >
                <SelectTrigger className="border-border bg-card h-11 min-h-11 w-full rounded-lg text-sm">
                  <SelectValue placeholder={filter.label} />
                </SelectTrigger>
                <SelectContent className="z-50">
                  <SelectItem value="ALL" className="py-2.5 text-sm">
                    Semua {filter.label}
                  </SelectItem>
                  {filter.options.map((opt) => (
                    <SelectItem
                      key={opt.value}
                      value={opt.value}
                      className="py-2.5 text-sm"
                    >
                      {opt.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          ))}
        </div>

        <SheetFooter className="flex flex-row gap-2 pt-2 sm:justify-end">
          {hasActiveFilters && onResetFilters && (
            <Button
              variant="outline"
              onClick={() => {
                onResetFilters();
              }}
              className="h-11 min-h-11 flex-1 gap-1.5 text-xs"
            >
              <RotateCcw className="h-3.5 w-3.5" />
              Reset
            </Button>
          )}
          <Button
            variant="default"
            onClick={() => setOpen(false)}
            className="h-11 min-h-11 flex-1 gap-1.5 text-xs"
          >
            <Check className="h-4 w-4" />
            Tutup
          </Button>
        </SheetFooter>
      </SheetContent>
    </Sheet>
  );
}
