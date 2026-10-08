import { FieldValues, Path, UseFormSetError } from "react-hook-form";

const VALIDATION_TRANSLATIONS: Record<string, string> = {
  "validation.required": "Kolom ini wajib diisi.",
  "validation.email": "Format email tidak valid.",
  "validation.max.numeric": "Nilai melebihi batas maksimal.",
  "validation.min.numeric": "Nilai kurang dari batas minimal.",
  "validation.max.string": "Jumlah karakter melebihi batas maksimal.",
  "validation.min.string": "Jumlah karakter kurang dari batas minimal.",
  "validation.confirmed": "Konfirmasi tidak cocok.",
  "validation.unique": "Data ini sudah digunakan.",
  "validation.exists": "Data yang dipilih tidak valid.",
  "validation.date": "Format tanggal tidak valid.",
  "validation.mimes": "Format berkas tidak didukung.",
  "validation.max.file": "Ukuran berkas melebihi batas maksimal.",
  "These credentials do not match our records.": "Email atau kata sandi salah.",
  "This account is inactive.": "Akun ini dinonaktifkan. Hubungi Administrator.",
};

// Menerjemahkan pesan kesalahan validasi dari API ke dalam bahasa Indonesia
export function translateValidationError(message: string): string {
  if (VALIDATION_TRANSLATIONS[message]) {
    return VALIDATION_TRANSLATIONS[message];
  }
  return message;
}

// Memetakan pesan kesalahan dari API ke dalam format yang dapat digunakan oleh form
export function mapApiErrorMessages(
  errors?: Record<string, string[]>
): Record<string, string> {
  if (!errors) {
    return {};
  }

  const result: Record<string, string> = {};

  for (const [field, messages] of Object.entries(errors)) {
    if (messages && messages.length > 0) {
      result[field] = translateValidationError(messages[0]);
    }
  }

  return result;
}

// Mengatur kesalahan form berdasarkan pesan kesalahan dari API
export function setFormErrors<T extends FieldValues>(
  errors: Record<string, string[]> | undefined,
  setError: UseFormSetError<T>
): void {
  if (!errors) {
    return;
  }

  const mapped = mapApiErrorMessages(errors);

  for (const [field, message] of Object.entries(mapped)) {
    setError(field as Path<T>, {
      type: "server",
      message,
    });
  }
}
