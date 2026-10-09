import { cn } from "@/lib/utils";
import { Badge } from "@/components/ui/badge";
import { getPriorityLabel } from "@/lib/labels";

interface PriorityBadgeProps {
  priority: string;
  className?: string;
}

export function PriorityBadge({ priority, className }: PriorityBadgeProps) {
  const norm = priority.toLowerCase();

  let priorityClass = "bg-[#f0ede6] text-[#5f5f5d] border-[#eceae4]";

  switch (norm) {
    case "critical":
      priorityClass =
        "bg-[#fae8e8] text-[#991b1b] border-[#f5c6c6] font-semibold";
      break;
    case "high":
      priorityClass =
        "bg-[#fbf0e4] text-[#b45309] border-[#f6ddc3] font-semibold";
      break;
    case "medium":
      priorityClass = "bg-[#e8edf5] text-[#1e40af] border-[#d4e0ee]";
      break;
    case "low":
      priorityClass = "bg-[#f0ede6] text-[#5f5f5d] border-[#eceae4]";
      break;
  }

  return (
    <Badge
      variant="outline"
      className={cn(
        "rounded-full px-2.5 py-0.5 text-xs font-medium shadow-none",
        priorityClass,
        className
      )}
    >
      {getPriorityLabel(priority)}
    </Badge>
  );
}
