"use client";

import * as React from "react";
import { format, parseISO } from "date-fns";
import { id } from "date-fns/locale";
import { Calendar as CalendarIcon, RotateCcw } from "lucide-react";
import type { DateRange } from "react-day-picker";

import { cn } from "@/lib/utils";
import { useIsMobile } from "@/hooks/use-media-query";
import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";

/**
 * Mengembalikan YYYY-MM-DD Asia/Jakarta (WIB)
 */
export function getTodayWibString(date: Date = new Date()): string {
  return new Intl.DateTimeFormat("en-CA", {
    timeZone: "Asia/Jakarta",
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  }).format(date);
}

/**
 * Menggeser tanggal (YYYY-MM-DD) sejumlah hari tertentu menggunakan kalkulasi UTC.
 */
export function shiftWibDays(ymdString: string, daysDelta: number): string {
  const [y, m, d] = ymdString.split("-").map(Number);
  const date = new Date(Date.UTC(y, m - 1, d + daysDelta));
  return date.toISOString().slice(0, 10);
}

export type DatePreset = "7d" | "30d" | "90d";

/**
 * Menghitung rentang tanggal preset dalam format YYYY-MM-DD (WIB).
 * 7d  = 6 hari sebelum hari ini hingga hari ini (inklusif 7 hari)
 * 30d = 29 hari sebelum hari ini hingga hari ini (inklusif 30 hari)
 * 90d = 89 hari sebelum hari ini hingga hari ini (inklusif 90 hari)
 */
export function presetRangeWib(
  preset: DatePreset,
  referenceDate: Date = new Date()
): { from: string; to: string } {
  const to = getTodayWibString(referenceDate);
  let daysDelta = -29;
  if (preset === "7d") daysDelta = -6;
  if (preset === "90d") daysDelta = -89;

  const from = shiftWibDays(to, daysDelta);
  return { from, to };
}

export interface DateRangePickerProps {
  from?: string; // YYYY-MM-DD (WIB)
  to?: string; // YYYY-MM-DD (WIB)
  onChange: (range: { from: string; to: string } | null) => void;
  className?: string;
}

export function DateRangePicker({
  from,
  to,
  onChange,
  className,
}: DateRangePickerProps) {
  const [open, setOpen] = React.useState(false);
  const isMobile = useIsMobile();

  const [selectedStart, setSelectedStart] = React.useState<Date | undefined>(
    undefined
  );

  const selectedRange: DateRange | undefined = React.useMemo(() => {
    if (selectedStart) {
      return { from: selectedStart, to: undefined };
    }
    if (!from && !to) return undefined;
    return {
      from: from ? parseISO(from) : undefined,
      to: to ? parseISO(to) : undefined,
    };
  }, [selectedStart, from, to]);

  const handlePreset = (preset: DatePreset) => {
    setSelectedStart(undefined);
    const range = presetRangeWib(preset);
    onChange(range);
    setOpen(false);
  };

  const handleReset = () => {
    setSelectedStart(undefined);
    onChange(null);
    setOpen(false);
  };

  const handleCalendarSelect = (range: DateRange | undefined) => {
    if (!range || (!range.from && !range.to)) {
      setSelectedStart(undefined);
      onChange(null);
      return;
    }

    if (!selectedStart) {
      // Klik pertama: mulai memilih rentang tanggal
      setSelectedStart(range.from);
      return;
    }

    // Klik kedua: pemilihan rentang tanggal selesai
    const finalFrom = range.from || selectedStart;
    const finalTo = range.to || selectedStart;

    setSelectedStart(undefined);
    const fromStr = format(finalFrom, "yyyy-MM-dd");
    const toStr = format(finalTo, "yyyy-MM-dd");
    onChange({ from: fromStr, to: toStr });
  };

  const labelText = React.useMemo(() => {
    if (from && to) {
      const fromDate = parseISO(from);
      const toDate = parseISO(to);
      if (fromDate.getFullYear() === toDate.getFullYear()) {
        return `${format(fromDate, "d MMM", { locale: id })} - ${format(toDate, "d MMM yyyy", { locale: id })}`;
      }
      return `${format(fromDate, "d MMM yyyy", { locale: id })} - ${format(toDate, "d MMM yyyy", { locale: id })}`;
    }
    if (from) {
      return format(parseISO(from), "d MMM yyyy", { locale: id });
    }
    return "Pilih rentang tanggal";
  }, [from, to]);

  const triggerButton = (
    <Button
      variant="outline"
      size="sm"
      className={cn(
        "focus-visible:ring-ring h-11 min-h-11 justify-start px-3.5 py-2 text-left text-xs font-normal focus-visible:ring-2 focus-visible:ring-offset-1 sm:h-9 sm:min-h-9 sm:text-sm",
        !from && !to && "text-muted-foreground"
      )}
      aria-label="Rentang Tanggal"
    >
      <CalendarIcon className="mr-2 h-4 w-4" />
      <span>{labelText}</span>
    </Button>
  );

  const presetsContent = (
    <div
      className={cn(
        "flex gap-1.5 p-3",
        isMobile
          ? "border-border flex-wrap items-center border-b"
          : "border-border flex-col border-b sm:w-36 sm:border-r sm:border-b-0"
      )}
    >
      {!isMobile && (
        <span className="text-muted-foreground px-2 py-1 text-xs font-semibold">
          Preset
        </span>
      )}
      <Button
        variant="ghost"
        size="sm"
        className={cn(
          "focus-visible:ring-ring text-xs font-normal focus-visible:ring-2 focus-visible:ring-offset-1",
          isMobile
            ? "border-border h-9 border px-3 sm:border-0"
            : "justify-start"
        )}
        onClick={() => handlePreset("7d")}
      >
        7 hari
      </Button>
      <Button
        variant="ghost"
        size="sm"
        className={cn(
          "focus-visible:ring-ring text-xs font-normal focus-visible:ring-2 focus-visible:ring-offset-1",
          isMobile
            ? "border-border h-9 border px-3 sm:border-0"
            : "justify-start"
        )}
        onClick={() => handlePreset("30d")}
      >
        30 hari
      </Button>
      <Button
        variant="ghost"
        size="sm"
        className={cn(
          "focus-visible:ring-ring text-xs font-normal focus-visible:ring-2 focus-visible:ring-offset-1",
          isMobile
            ? "border-border h-9 border px-3 sm:border-0"
            : "justify-start"
        )}
        onClick={() => handlePreset("90d")}
      >
        90 hari
      </Button>
      {(from || to) && (
        <Button
          variant="ghost"
          size="sm"
          className={cn(
            "text-destructive hover:text-destructive hover:bg-destructive/10 focus-visible:ring-ring text-xs font-normal focus-visible:ring-2 focus-visible:ring-offset-1",
            isMobile ? "ml-auto h-9 px-3" : "mt-2 justify-start"
          )}
          onClick={handleReset}
        >
          <RotateCcw className="mr-1.5 h-3 w-3" />
          Reset
        </Button>
      )}
    </div>
  );

  const calendarContent = (
    <div className={cn("p-2", isMobile && "flex justify-center")}>
      <Calendar
        mode="range"
        defaultMonth={selectedRange?.from}
        selected={selectedRange}
        onSelect={handleCalendarSelect}
        numberOfMonths={1}
      />
    </div>
  );

  if (isMobile) {
    return (
      <div className={cn("relative inline-block", className)}>
        <Dialog open={open} onOpenChange={setOpen}>
          <DialogTrigger asChild>{triggerButton}</DialogTrigger>
          <DialogContent className="max-w-[calc(100vw-2rem)] p-4 sm:max-w-md">
            <DialogHeader>
              <DialogTitle>Pilih Rentang Tanggal</DialogTitle>
            </DialogHeader>
            <div className="flex flex-col">
              {presetsContent}
              {calendarContent}
            </div>
          </DialogContent>
        </Dialog>
      </div>
    );
  }

  return (
    <div className={cn("relative inline-block", className)}>
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>{triggerButton}</PopoverTrigger>
        <PopoverContent className="w-auto p-0" align="start">
          <div className="flex flex-col sm:flex-row">
            {presetsContent}
            {calendarContent}
          </div>
        </PopoverContent>
      </Popover>
    </div>
  );
}
