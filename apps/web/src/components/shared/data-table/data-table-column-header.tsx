import { ChevronDown, ChevronUp, ChevronsUpDown } from "lucide-react";
import { cn } from "@/lib/utils";

interface DataTableColumnHeaderProps {
  title: string;
  sorted?: "asc" | "desc" | false;
  onSort?: () => void;
  className?: string;
}

export function DataTableColumnHeader({
  title,
  sorted = false,
  onSort,
  className,
}: DataTableColumnHeaderProps) {
  if (!onSort) {
    return (
      <span className={cn("text-xs font-semibold", className)}>{title}</span>
    );
  }

  return (
    <button
      type="button"
      onClick={onSort}
      className={cn(
        "group text-muted-foreground hover:text-foreground focus-visible:ring-ring inline-flex items-center gap-1.5 rounded-sm text-xs font-semibold focus:outline-none focus-visible:ring-2 focus-visible:ring-offset-1",
        sorted && "text-foreground font-bold",
        className
      )}
      aria-label={`Urutkan berdasarkan ${title}`}
    >
      <span>{title}</span>
      {sorted === "asc" ? (
        <ChevronUp className="text-foreground h-3.5 w-3.5" />
      ) : sorted === "desc" ? (
        <ChevronDown className="text-foreground h-3.5 w-3.5" />
      ) : (
        <ChevronsUpDown className="h-3.5 w-3.5 opacity-40 group-hover:opacity-100" />
      )}
    </button>
  );
}
