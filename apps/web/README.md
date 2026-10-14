# BantuIT — Frontend Web Application

Aplikasi antarmuka web modern untuk platform **BantuIT Enterprise IT Service Management (ITSM)**, dibangun menggunakan **Next.js 16 (App Router)**, **React 19**, **Tailwind CSS v4**, dan **TypeScript**.

Dirancang dengan mengedepankan keamanan berbasis arsitektur _Backend-for-Frontend_ (BFF), aksesibilitas berstandar **WCAG 2.2 AA**, ergonomi antarmuka mobile-first, serta visualisasi analitik interaktif.

---

## 1. Arsitektur & Pola Rekayasa Frontend

### Pola Backend-for-Frontend (BFF) & Keamanan Kredensial

Aplikasi web mengimplementasikan pola BFF menggunakan Next.js Route Handlers:

- **Isolasi Token Bearer:** Token otentikasi Sanctum disimpan pada cookie browser dengan atribut `httpOnly`, `Secure`, dan `SameSite=Lax`. Kode JavaScript di sisi klien tidak memiliki akses baca langsung ke token, mengeliminasi vektor serangan pencurian kredensial melalui XSS (_Cross-Site Scripting_).
- **Proksi Terpusat (`/api/proxy/[...path]`):** Seluruh komunikasi data dari komponen klien dialihkan melalui Route Handler BFF internal. Handler ini secara otomatis membaca cookie sesi, menyematkan header `Authorization: Bearer <token>`, dan meneruskannya ke backend API Laravel di jaringan privat.

```
[Browser / Komponen Klien]
          │
          │ Permintaan HTTP (Cookie httpOnly terpasang otomatis)
          ▼
[Next.js BFF Route Handler: /api/proxy/[...path]]
          │
          │ Membaca cookie & menyematkan Bearer Token di header
          ▼
[Backend Laravel REST API: http://api:8000/api/...]
```

### Sinkronisasi State & Data (TanStack Query v5)

- **Server State Caching:** Mengelola sinkronisasi data server, pagination, filter, dan revalidasi mutasi secara deklaratif tanpa boilerplate state management global yang berlebihan.
- **Polling Notifikasi Hemat Daya:** Pusat notifikasi memperbarui status setiap 30 detik secara otomatis, dan secara cerdas menghentikan proses polling ketika tab browser sedang tidak aktif (`refetchIntervalInBackground: false`).

### Ergonomi Antarmuka & Desain Responsif

- **Adaptasi Mobile Cerdas:** Tabel data bervolume tinggi (tiket, inventaris aset, dan log audit) secara otomatis bertransformasi menjadi tampilan _Card View_ adaptif pada layar smartphone.
- **Filter Bottom Sheet:** Panel filter pencarian dibuka melalui _Bottom Sheet_ dari bagian bawah layar pada viewport mobile, memudahkan jangkauan satu tangan.
- **Sticky Mobile Action Bar:** Tombol aksi operasional tiket (ambil tugas, eskalasi prioritas, perbarui status) tersemat di bilah bawah layar ponsel (_thumb-zone_).
- **Target Sentuh Ergonomis:** Seluruh elemen interaktif memiliki area sentuh minimal 44x44 piksel sesuai standar ergonomi perangkat genggam.

### Kepatuhan Aksesibilitas (WCAG 2.2 AA)

- Dibangun di atas primitif **Radix UI** yang menjamin semantik HTML, navigasi keyboard penuh (Tab/Shift+Tab, Arrow keys, Esc), dan _focus trapping_ pada jendela dialog modal.
- Kontras warna teks dan latar belakang telah lolos audit rasio kontras 4.5:1 untuk teks normal dan 3:1 untuk teks besar serta komponen grafis.

---

## 2. Struktur Direktori Proyek

```
apps/web/
├── e2e/                       # Skenario pengujian Playwright E2E
│   ├── auth.spec.ts           # Alur login, validasi sesi, & logout
│   ├── tickets.spec.ts        # Alur hidup tiket (golden path)
│   ├── responsive.spec.ts     # Pengujian layout mobile & tablet
│   └── accessibility.spec.ts  # Audit kepatuhan WCAG 2.2 AA
├── public/                    # Aset statis & logo resmi BantuIT
└── src/
    ├── app/                   # Next.js App Router (File-based routing)
    │   ├── (auth)/            # Route Group otentikasi (login, ganti password)
    │   ├── (app)/             # Route Group aplikasi terproteksi
    │   │   ├── dashboard/     # Dashboard analitik per peran
    │   │   ├── tiket/         # Daftar tiket, buat tiket, detail tiket
    │   │   ├── aset/          # Inventaris aset TI & riwayat pemegang
    │   │   ├── kb/            # Basis pengetahuan (Knowledge Base)
    │   │   ├── admin/         # Manajemen pengguna, master data, audit logs
    │   │   └── profil/        # Pengaturan profil pengguna
    │   └── api/
    │       ├── auth/          # Route handlers untuk login/logout/set-cookie
    │       └── proxy/         # Route handler proksi BFF ke backend
    ├── components/
    │   ├── ui/                # 16 Komponen primitif UI (Button, Input, Dialog, dll.)
    │   ├── shared/            # 9 Komponen bisnis pakai-ulang:
    │   │                      # DataTable, FilterBar, SearchInput, StatusBadge,
    │   │                      # PriorityBadge, SlaIndicator, RelativeTime,
    │   │                      # FileUpload, ConfirmDialog, EmptyState
    │   ├── layout/            # App Shell: Sidebar, Topbar, Mobile Drawer
    │   └── dashboard/         # Visualisasi grafik analitik (Recharts)
    ├── hooks/                 # Custom React Hooks & query hooks
    └── lib/                   # Utilitas, skema Zod, klien API, dan formatters
```

---

## 3. Komponen Bersama Utama (_Shared Components_)

Platform mengonsolidasikan elemen antarmuka melalui 9 komponen bersama tingkat tinggi:

1. **`DataTable`:** Komponen tabel data responsif dengan dukungan pengurutan kolom, indikator pemuatan, dan adapter _Card View_ otomatis pada viewport mobile.
2. **`FilterBar`:** Panel penyaring data yang terintegrasi dengan URL query parameters dan mendukung mode _Bottom Sheet_ pada layar kecil.
3. **`SearchInput`:** Input pencarian teks dengan mekanisme _debounce_ otomatis untuk mengoptimalkan panggilan API.
4. **`StatusBadge` & `PriorityBadge`:** Label status siklus hidup dan prioritas tiket dengan palet warna semantik terstandarisasi.
5. **`SlaIndicator`:** Indikator visual batas waktu SLA yang menghitung sisa waktu pengerjaan dan menandai status pelanggaran (_breached_) secara real-time.
6. **`RelativeTime`:** Komponen format waktu relatif (misal: "5 menit yang lalu") yang secara otomatis diperbarui dan dilengkapi tooltip stempel waktu absolut (WIB).
7. **`FileUpload`:** Komponen unggah berkas drag-and-drop dengan validasi tipe berkas di sisi klien (PDF, PNG, JPG maksimal 5MB) dan indikator progres.
8. **`ConfirmDialog`:** Modal konfirmasi tindakan destruktif atau mutasi status penting dengan proteksi pembatalan tidak sengaja.
9. **`EmptyState`:** Tampilan representatif saat data kosong dengan ilustrasi kontekstual dan tombol ajakan aksi (_Call-to-Action_).

---

## 4. Panduan Menjalankan Frontend Secara Lokal

### Prasyarat

- Node.js 22.x LTS
- npm 10.x atau lebih baru
- Backend API BantuIT yang sudah aktif di port `8000`

### Langkah Instalasi

1. Masuk ke direktori `apps/web`:
   ```bash
   cd apps/web
   ```
2. Salin berkas konfigurasi variabel lingkungan:
   ```bash
   cp .env.example .env.local
   ```
3. Pasang dependensi Node.js:
   ```bash
   npm ci
   ```
4. Jalankan server pengembangan lokal:
   ```bash
   npm run dev
   ```
5. Akses aplikasi melalui peramban di `http://localhost:3000`.

---

## 5. Eksekusi Pengujian & Validasi Kualitas

### 1. Pengecekan Statis (Typecheck, Lint & Format)

```bash
# Verifikasi tipe data TypeScript tanpa emit berkas
npm run typecheck

# Memeriksa kepatuhan linter ESLint
npm run lint

# Memeriksa konsistensi format kode Prettier
npm run format:check
```

### 2. Pengujian Unit & Komponen (Vitest)

Menjalankan 138 unit & component tests untuk memvalidasi logika hook, formatters, skema validasi Zod, dan perilaku rendering komponen:

```bash
npm run test
```

### 3. Pengujian End-to-End (Playwright)

Memvalidasi alur kerja pengguna sesungguhnya pada berbagai lingkungan peramban dan perangkat:

```bash
# Menjalankan seluruh skenario E2E (Desktop Chromium, Pixel 7, iPhone 14)
npx playwright test

# Menjalankan pengujian secara interaktif dengan UI mode
npx playwright test --ui

# Menjalankan pengujian spesifik untuk audit aksesibilitas
npx playwright test accessibility.spec.ts
```

---

## 6. Variabel Lingkungan (_Environment Variables_)

Berkas konfigurasi lingkungan frontend:

| Variabel              | Deskripsi                                                              | Nilai Bawaan Lokal          |
| --------------------- | ---------------------------------------------------------------------- | --------------------------- |
| `API_BASE_URL`        | Alamat endpoint REST API backend yang dihubungi oleh BFF Route Handler | `http://localhost:8000/api` |
| `NEXT_PUBLIC_APP_URL` | Alamat URL publik aplikasi frontend                                    | `http://localhost:3000`     |

---

## 7. Optimasi Build Produksi & Docker Standalone

Konfigurasi `next.config.ts` menggunakan opsi `output: "standalone"`:

- Menghasilkan bundle server mandiri yang hanya menyertakan dependensi esensial produksi dari `node_modules`, mereduksi ukuran container Docker secara signifikan.
- Container image diproduksi menggunakan multi-stage build berbasis `node:22-alpine` dengan eksekusi aman di bawah pengguna non-root (`nextjs:nodejs`).
