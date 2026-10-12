/**
 * Memilih status/state dari editor teks.
 */
export interface EditorState {
  /** Isi teks keseluruhan di dalam editor */
  value: string;
  /** Indeks posisi awal kursor atau seleksi teks */
  selectionStart: number;
  /** Indeks posisi akhir kursor atau seleksi teks */
  selectionEnd: number;
}

/**
 * Menambahkan atau melepas simbol format inline (seperti `**` untuk cetak tebal, `*` untuk miring).
 *
 * - Jika tidak ada teks yang dipilih, akan menyisipkan marker beserta teks placeholder,
 *   lalu memblok teks placeholder tersebut.
 * - Jika teks yang dipilih sudah dibungkus marker, marker akan dihapus (toggle off).
 * - Jika belum dibungkus, marker akan ditambahkan di luar teks yang dipilih (toggle on).
 *
 * @param state Status editor saat ini.
 * @param marker Simbol pembungkus (contoh: `**`, `*`, `` ` ``).
 * @param placeholder Teks default jika tidak ada teks yang sedang dipilih.
 * @returns Status editor baru setelah penyesuaian teks dan kursor.
 */
export function toggleInlineWrap(
  state: EditorState,
  marker: string,
  placeholder = "teks"
): EditorState {
  const { value, selectionStart, selectionEnd } = state;

  // Jika tidak ada teks yang di-highlight/diseleksi
  if (selectionStart === selectionEnd) {
    const before = value.slice(0, selectionStart);
    const after = value.slice(selectionStart);
    const innerStart = selectionStart + marker.length;
    return {
      value: `${before}${marker}${placeholder}${marker}${after}`,
      selectionStart: innerStart,
      selectionEnd: innerStart + placeholder.length,
    };
  }

  const selected = value.slice(selectionStart, selectionEnd);
  const before = value.slice(0, selectionStart);
  const after = value.slice(selectionEnd);

  // Periksa apakah teks yang dipilih sudah terbungkus marker
  const isWrapped =
    selected.length >= marker.length * 2 &&
    selected.startsWith(marker) &&
    selected.endsWith(marker);

  if (isWrapped) {
    // Lepaskan pembungkus (un-wrap)
    const inner = selected.slice(
      marker.length,
      selected.length - marker.length
    );
    return {
      value: `${before}${inner}${after}`,
      selectionStart,
      selectionEnd: selectionStart + inner.length,
    };
  }

  // Bungkus teks (wrap)
  return {
    value: `${before}${marker}${selected}${marker}${after}`,
    selectionStart: selectionStart + marker.length,
    selectionEnd: selectionEnd + marker.length,
  };
}

/**
 * Fungsi pembantu internal untuk melakukan operasi pada baris-baris teks yang sedang dipilih.
 *
 * @param state Status editor saat ini.
 * @param transform Fungsi transformasi yang menerima array baris dan mengembalikan array baris baru.
 * @returns Status editor baru.
 */
function lineOp(
  state: EditorState,
  transform: (lines: string[]) => string[]
): EditorState {
  const { value, selectionStart, selectionEnd } = state;

  // Tentukan batas awal baris pertama dan akhir dari baris terakhir yang terjangkau oleh seleksi
  const blockStart = value.lastIndexOf("\n", selectionStart - 1) + 1;
  const nextNewline = value.indexOf("\n", selectionEnd);
  const blockEnd = nextNewline === -1 ? value.length : nextNewline;

  const lines = value.slice(blockStart, blockEnd).split("\n");
  const newBlock = transform(lines).join("\n");

  return {
    value: value.slice(0, blockStart) + newBlock + value.slice(blockEnd),
    selectionStart: blockStart,
    selectionEnd: blockStart + newBlock.length,
  };
}

/**
 * Mengaplikasikan atau mengubah tingkat Heading Markdown (`#`, `##`, `###`) pada baris yang dipilih.
 *
 * @param state Status editor saat ini.
 * @param level Tingkat heading (1, 2, atau 3).
 * @returns Status editor baru dengan format heading yang telah disesuaikan.
 */
export function applyHeading(
  state: EditorState,
  level: 1 | 2 | 3
): EditorState {
  const hashes = "#".repeat(level);
  return lineOp(state, (lines) =>
    lines.map((line) => {
      // Hapus simbol heading yang sudah ada sebelumnya (1 hingga 6 buah #)
      const stripped = line.replace(/^#{1,6}\s+/, "");
      // Jika baris sudah memiliki level heading yang persis sama, hapus heading tersebut
      return line.startsWith(`${hashes} `) ? stripped : `${hashes} ${stripped}`;
    })
  );
}

/**
 * Menambahkan atau menghapus awalan (prefix) tertentu di setiap awal baris yang dipilih
 * (contoh: awalan `"- "` untuk daftar poin atau `"> "` untuk blok kutipan).
 *
 * @param state Status editor saat ini.
 * @param prefix Teks awalan yang ingin ditambahkan atau dihapus.
 * @returns Status editor baru.
 */
export function toggleLinePrefix(
  state: EditorState,
  prefix: string
): EditorState {
  return lineOp(state, (lines) => {
    const nonEmpty = lines.filter((line) => line.length > 0);
    // Periksa apakah semua baris yang tidak kosong sudah diawali oleh prefix tersebut
    const allHave =
      nonEmpty.length > 0 && nonEmpty.every((line) => line.startsWith(prefix));

    return lines.map((line) => {
      if (line.length === 0) return line; // Biarkan baris kosong tetap kosong
      if (allHave)
        return line.startsWith(prefix) ? line.slice(prefix.length) : line;
      return line.startsWith(prefix) ? line : `${prefix}${line}`;
    });
  });
}

/**
 * Menambahkan atau menghapus daftar angka berurutan (*ordered list*) pada baris-baris yang dipilih.
 *
 * @param state Status editor saat ini.
 * @returns Status editor baru.
 */
export function toggleOrderedList(state: EditorState): EditorState {
  const ordered = /^\d+\.\s/; // Ekspresi reguler untuk mencocokkan format "1. ", "2. ", dst.
  return lineOp(state, (lines) => {
    const nonEmpty = lines.filter((line) => line.length > 0);
    // Periksa apakah semua baris tidak kosong sudah berformat daftar angka
    const allOrdered =
      nonEmpty.length > 0 && nonEmpty.every((line) => ordered.test(line));

    let counter = 0;
    return lines.map((line) => {
      if (line.length === 0) return line;
      if (allOrdered) return line.replace(ordered, "");
      counter += 1;
      return `${counter}. ${line}`;
    });
  });
}

/**
 * Menyisipkan tautan berformat Markdown `[teks](url)`.
 *
 * - Jika ada teks yang diseleksi, teks tersebut digunakan sebagai label tautan.
 * - Kursor/seleksi otomatis akan mengarahkan dan memblok bagian `"url"`, sehingga
 *   pengguna bisa langsung mengetik alamat URL yang diinginkan.
 *
 * @param state Status editor saat ini.
 * @param placeholder Teks default label tautan jika tidak ada teks yang sedang dipilih.
 * @returns Status editor baru.
 */
export function insertLink(
  state: EditorState,
  placeholder = "teks tautan"
): EditorState {
  const { value, selectionStart, selectionEnd } = state;
  const text =
    selectionStart === selectionEnd
      ? placeholder
      : value.slice(selectionStart, selectionEnd);

  const before = value.slice(0, selectionStart);
  const after = value.slice(selectionEnd);
  const urlStart = selectionStart + text.length + 3; // Menghitung indeks awal posisi kata 'url'

  return {
    value: `${before}[${text}](url)${after}`,
    selectionStart: urlStart,
    selectionEnd: urlStart + "url".length,
  };
}
