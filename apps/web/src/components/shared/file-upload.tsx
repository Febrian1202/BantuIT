"use client";

import React, { useRef, useState, useCallback } from "react";
import { UploadCloud, File, X, AlertCircle } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { cn } from "@/lib/utils";

const ALLOWED_MIME_TYPES = [
  "image/jpeg",
  "image/jpg",
  "image/png",
  "application/pdf",
];

const MAX_FILE_SIZE_BYTES = 5 * 1024 * 1024; // 5 MB

interface FileUploadProps {
  file: File | null;
  onFileSelect: (file: File | null) => void;
  className?: string;
  disabled?: boolean;
  isUploading?: boolean;
  uploadProgress?: number;
  capture?: boolean | "user" | "environment";
}

export function FileUpload({
  file,
  onFileSelect,
  className,
  disabled = false,
  isUploading = false,
  uploadProgress = 0,
  capture,
}: FileUploadProps) {
  const [error, setError] = useState<string | null>(null);
  const [isDragActive, setIsDragActive] = useState<boolean>(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const validateAndProcessFile = useCallback(
    (selected: File | null): boolean => {
      setError(null);
      if (!selected) return false;

      // Validasi mime
      if (!ALLOWED_MIME_TYPES.includes(selected.type)) {
        setError("Format berkas harus JPG, PNG, atau PDF.");
        return false;
      }

      // Validasi size
      if (selected.size > MAX_FILE_SIZE_BYTES) {
        setError("Ukuran berkas melebihi batas maksimal 5 MB.");
        return false;
      }

      onFileSelect(selected);
      return true;
    },
    [onFileSelect]
  );

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const selected = e.target.files?.[0] ?? null;
    validateAndProcessFile(selected);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (disabled || isUploading) return;
    setIsDragActive(true);
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragActive(false);
    if (disabled || isUploading) return;

    const droppedFile = e.dataTransfer.files?.[0] ?? null;
    validateAndProcessFile(droppedFile);
  };

  const handleClear = () => {
    if (isUploading) return;
    setError(null);
    onFileSelect(null);
    if (inputRef.current) {
      inputRef.current.value = "";
    }
  };

  return (
    <div className={cn("space-y-2", className)}>
      <input
        ref={inputRef}
        type="file"
        accept=".jpg,.jpeg,.png,.pdf,image/*"
        capture={
          capture
            ? typeof capture === "string"
              ? capture
              : "environment"
            : undefined
        }
        onChange={handleFileChange}
        className="hidden"
        disabled={disabled || isUploading}
        id="file-upload-input"
      />

      {!file ? (
        <label
          htmlFor="file-upload-input"
          data-testid="file-dropzone"
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
          className={cn(
            "border-border bg-muted/20 hover:bg-muted/40 flex cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed p-6 text-center transition-colors",
            isDragActive &&
              "border-primary bg-primary/5 ring-primary/20 ring-2",
            (disabled || isUploading) &&
              "pointer-events-none cursor-not-allowed opacity-50"
          )}
        >
          <UploadCloud
            className={cn(
              "text-muted-foreground mb-2 h-8 w-8 transition-colors",
              isDragActive && "text-primary"
            )}
          />
          <p className="text-foreground text-xs font-medium">
            {isDragActive
              ? "Lepaskan berkas di sini"
              : "Klik untuk memilih berkas lampiran atau tarik & lepas ke sini"}
          </p>
          <p className="text-muted-foreground mt-0.5 text-[11px]">
            Maksimal 5 MB (JPG, PNG, PDF)
          </p>
        </label>
      ) : (
        <div className="border-border bg-card space-y-2.5 rounded-lg border p-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2.5 overflow-hidden">
              <File className="text-primary h-5 w-5 shrink-0" />
              <div className="overflow-hidden">
                <p className="text-foreground max-w-xs truncate text-xs font-medium">
                  {file.name}
                </p>
                <p className="text-muted-foreground text-[11px]">
                  {(file.size / 1024 / 1024).toFixed(2)} MB
                </p>
              </div>
            </div>
            <Button
              type="button"
              variant="ghost"
              size="icon"
              onClick={handleClear}
              disabled={isUploading}
              className="text-muted-foreground hover:text-destructive h-7 w-7 rounded-full disabled:opacity-40"
              aria-label="Hapus berkas"
            >
              <X className="h-4 w-4" />
            </Button>
          </div>

          {isUploading && (
            <div className="border-border space-y-1.5 border-t pt-1">
              <div className="flex items-center justify-between text-[11px]">
                <span className="text-muted-foreground font-medium">
                  Mengunggah… {uploadProgress}%
                </span>
                <span className="text-muted-foreground font-mono">
                  {uploadProgress}%
                </span>
              </div>
              <Progress
                value={uploadProgress}
                aria-label="Progres unggah berkas"
              />
            </div>
          )}
        </div>
      )}

      {error && (
        <div className="text-destructive flex items-center gap-1.5 text-xs font-medium">
          <AlertCircle className="h-3.5 w-3.5 shrink-0" />
          <span>{error}</span>
        </div>
      )}
    </div>
  );
}
