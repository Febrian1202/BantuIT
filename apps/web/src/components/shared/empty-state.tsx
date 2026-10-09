import { FileX2, type LucideIcon } from "lucide-react";
import { Button } from "@/components/ui/button";

interface EmptyStateProps {
  title?: string;
  description?: string;
  icon?: LucideIcon;
  actionText?: string;
  onAction?: () => void;
}

export function EmptyState({
  title = "Tidak ada data ditemukan",
  description = "Belum ada data untuk ditampilkan pada kategori atau filter ini.",
  icon: Icon = FileX2,
  actionText,
  onAction,
}: EmptyStateProps) {
  return (
    <div className="border-border animate-empty-in flex min-h-56 flex-col items-center justify-center rounded-lg border border-dashed p-8 text-center motion-reduce:animate-none">
      <div className="bg-muted text-muted-foreground mx-auto flex h-12 w-12 items-center justify-center rounded-full">
        <Icon className="h-6 w-6" />
      </div>
      <h3 className="text-foreground mt-3 text-sm font-semibold tracking-tight">
        {title}
      </h3>
      <p className="text-muted-foreground mt-1 max-w-sm text-xs leading-relaxed">
        {description}
      </p>
      {actionText && onAction && (
        <Button
          onClick={onAction}
          variant="outline"
          size="sm"
          className="mt-4 rounded-lg text-xs"
        >
          {actionText}
        </Button>
      )}
    </div>
  );
}
