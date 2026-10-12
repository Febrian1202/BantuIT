"use client";

import { FileText, Download, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { RelativeTime } from "@/components/shared/relative-time";
import { attachmentDownloadUrl, formatFileSize } from "@/lib/attachments";
import type { TicketAttachment } from "@/types/tickets";

interface AttachmentCardProps {
  attachment: TicketAttachment;
  canDelete: boolean;
  onDelete: (id: number) => void;
}

export function AttachmentCard({
  attachment,
  canDelete,
  onDelete,
}: AttachmentCardProps) {
  const downloadUrl = attachmentDownloadUrl(attachment.download_url);

  return (
    <div className="border-border bg-card flex items-center gap-3 rounded-lg border p-3">
      <FileText className="text-primary h-5 w-5 shrink-0" />

      <div className="min-w-0 flex-1">
        <p className="text-foreground max-w-50 truncate text-xs font-medium">
          {attachment.original_filename}
        </p>
        <p className="text-muted-foreground text-[11px]">
          {formatFileSize(attachment.file_size)} —{" "}
          {attachment.uploaded_by?.full_name ?? "—"}
        </p>
        <RelativeTime
          date={attachment.created_at}
          className="text-muted-foreground text-[11px]"
        />
      </div>

      <div className="flex shrink-0 items-center gap-1">
        <a
          href={downloadUrl}
          download
          className="text-muted-foreground hover:text-foreground hover:bg-muted inline-flex h-7 w-7 items-center justify-center rounded-md transition-colors"
          aria-label="Unduh lampiran"
        >
          <Download className="h-3.5 w-3.5" />
        </a>

        {canDelete && (
          <Button
            variant="ghost"
            size="icon"
            className="text-muted-foreground hover:text-destructive hover:bg-destructive/10 h-7 w-7"
            onClick={() => onDelete(attachment.id)}
            aria-label="Hapus lampiran"
          >
            <Trash2 className="h-3.5 w-3.5" />
          </Button>
        )}
      </div>
    </div>
  );
}
