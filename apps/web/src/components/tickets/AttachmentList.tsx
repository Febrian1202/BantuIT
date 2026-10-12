"use client";

import { useState } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Paperclip } from "lucide-react";
import { FileUpload } from "@/components/shared/file-upload";
import { useUploadWithProgress } from "@/hooks/use-upload-with-progress";
import { useApiMutation } from "@/hooks/use-api-mutation";
import { apiFetch } from "@/lib/client/api";
import { ticketKeys } from "@/lib/query-key";
import { AttachmentCard } from "./AttachmentCard";
import { useAuth } from "@/components/providers/auth-provider";
import type { TicketAttachment } from "@/types/tickets";
import { toast } from "sonner";

interface AttachmentListProps {
  ticketId: number;
  canUpload: boolean;
}

export function AttachmentList({ ticketId, canUpload }: AttachmentListProps) {
  const { user, can } = useAuth();
  const queryClient = useQueryClient();
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const attachmentsKey = ticketKeys.attachments(ticketId);
  const detailKey = ticketKeys.detail(ticketId);

  const { data, isLoading } = useQuery({
    queryKey: attachmentsKey,
    queryFn: () =>
      apiFetch<TicketAttachment[]>(`/tickets/${ticketId}/attachments`),
  });
  const attachments = data?.data ?? [];

  const { upload, isUploading, progress } =
    useUploadWithProgress<TicketAttachment>({
      url: `/tickets/${ticketId}/attachments`,
      onSuccess: () => {
        setSelectedFile(null);
        toast.success("Lampiran berhasil diunggah.");
        void queryClient.invalidateQueries({ queryKey: attachmentsKey });
        void queryClient.invalidateQueries({ queryKey: detailKey });
      },
      onError: (msg) => {
        setSelectedFile(null);
        toast.error(msg || "Gagal mengunggah lampiran.");
      },
    });

  const deleteMutation = useApiMutation({
    mutationFn: (id: number) =>
      apiFetch<void>(`/attachments/${id}`, { method: "DELETE" }),
    onSuccessMessage: "Lampiran berhasil dihapus.",
    invalidateKeys: [attachmentsKey, detailKey],
  });

  const handleFileSelect = (file: File | null) => {
    setSelectedFile(file);
    if (!file) return;

    const formData = new FormData();
    formData.append("file", file);
    void upload(formData).catch(() => {});
  };

  const canDelete = (attachment: TicketAttachment): boolean => {
    return can("attachment.delete") || attachment.uploaded_by?.id === user?.id;
  };

  if (isLoading) {
    return (
      <div className="space-y-2">
        {[1, 2].map((i) => (
          <div key={i} className="bg-muted/60 h-12 animate-pulse rounded-lg" />
        ))}
      </div>
    );
  }

  return (
    <div className="space-y-3">
      {attachments.length === 0 && !canUpload ? (
        <p className="text-muted-foreground text-xs">Belum ada lampiran.</p>
      ) : (
        <div className="space-y-2">
          {attachments.map((a) => (
            <AttachmentCard
              key={a.id}
              attachment={a}
              canDelete={canDelete(a)}
              onDelete={(id) => deleteMutation.mutate(id)}
            />
          ))}
        </div>
      )}

      {canUpload && (
        <div className="pt-1">
          <FileUpload
            file={selectedFile}
            onFileSelect={handleFileSelect}
            isUploading={isUploading}
            uploadProgress={progress}
            disabled={isUploading}
          />
        </div>
      )}
    </div>
  );
}

export function AttachmentSectionTitle({ count }: { count: number }) {
  return (
    <div className="flex items-center gap-2">
      <Paperclip className="text-primary h-4 w-4" />
      <h2 className="text-foreground text-sm font-semibold">Lampiran</h2>
      {count > 0 && (
        <span className="bg-muted text-muted-foreground rounded-full px-1.5 py-0.5 text-[11px]">
          {count}
        </span>
      )}
    </div>
  );
}
