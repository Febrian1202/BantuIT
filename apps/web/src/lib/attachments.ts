// Konversi download attachment url
export function attachmentDownloadUrl(url: string): string {
  if (url.startsWith("/api")) {
    return url.replace(/^\/api\//, "/api/proxy/");
  }
  return url;
}

// Format file size jadi readable
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}
