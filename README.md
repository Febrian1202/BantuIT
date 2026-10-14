# BantuIT — Enterprise IT Service Management (ITSM) Platform

BantuIT adalah platform IT Service Management (ITSM) terstandarisasi yang dirancang untuk mengelola siklus hidup tiket insiden, permintaan layanan, inventaris aset TI, serta basis pengetahuan operasional secara terpusat.

Platform ini dikembangkan sebagai **Capstone Project Rekayasa Perangkat Lunak** dengan mengadopsi standar arsitektur industri: pola monorepo, *Backend-for-Frontend* (BFF), *Service Layer Pattern*, *Deterministic SLA Engine*, pengujian otomatis bertingkat (*Test-Driven Development*), kepatuhan aksesibilitas WCAG 2.2 AA, dan kesiapan kontainerisasi produksi berkinerja tinggi.

---

## 1. Arsitektur & Prinsip Rekayasa Sistem

Sistem mengadopsi prinsip desain yang memisahkan tanggung jawab antarkomponen secara tegas:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Klien Browser & Mobile                          │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ HTTPS (Cookie httpOnly)
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│               apps/web: Next.js 16 (App Router & BFF)                  │
│  - React 19 + Tailwind CSS v4 + Radix UI Primitives                    │
│  - BFF Proxy Route Handler (/api/proxy/[...path])                      │
│  - State & Data Synchronization (TanStack Query v5)                    │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Private Network / Bearer Token
                                    ▼
┌────────────────────────────────────────────────────────────────────────┐
│                   apps/api: Laravel 13 (REST API)                      │
│  - Request Pipeline: FormRequest -> DTO -> Service Layer -> Resource   │
│  - Single Envelope Standard ({ success, message, data, meta/errors })  │
│  - Otorisasi Native Gate & Policy (Tanpa Spatie Permission)            │
│  - Deterministic SLA Engine & State Transition Matrix                  │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                  ┌─────────────────┴─────────────────┐
                  ▼                                   ▼
┌───────────────────────────────────┐ ┌───────────────────────────────────┐
│     Database Relasional           │ │    Penyimpanan Berkas Privat      │
│     MySQL 8.4 / MariaDB 11        │ │    Local Storage Disk             │
│     (18 Tabel Relasional)         │ │    (Validasi MIME & Ekstensi)     │
└───────────────────────────────────┘ └───────────────────────────────────┘
```

### Keputusan Arsitektur Utama
1. **BFF Proxy & Keamanan Kredensial:**
   Klien peramban tidak pernah mengakses API Laravel secara langsung. Seluruh permintaan dilewatkan melalui Next.js Route Handler (`/api/proxy/[...path]`). Token Bearer Laravel Sanctum disimpan di dalam cookie `httpOnly` dengan atribut `SameSite=Lax` dan `Secure`, memitigasi risiko serangan Cross-Site Scripting (XSS).
2. **Format Respons Tunggal (*Single Envelope*):**
   Seluruh endpoint mengembalikan struktur respons yang konsisten:
   - Sukses: `{ "success": true, "message": "...", "data": { ... }, "meta"?: { ... } }` (HTTP 200/201).
   - Gagal: `{ "success": false, "message": "...", "errors": { "field": ["pesan"] } }` (HTTP 401/403/404/409/422).
3. **State Machine Tiket Deterministik:**
   Siklus hidup tiket dibatasi pada 5 status pasti (`OPEN`, `ASSIGNED`, `IN_PROGRESS`, `RESOLVED`, `CLOSED`) tanpa status pause/hold. Transisi status divalidasi ketat melalui *State Transition Matrix* dan pengecekan optimistik (`expected_status_id`) untuk mencegah *race condition* (HTTP 409).
4. **SLA Snapshotted & Perhitungan Defensif:**
   Target durasi dan batas waktu SLA dihitung dan disimpan permanen pada tiket saat dibuat berdasarkan prioritas layanan. Status pelanggaran SLA dihitung defensif pada level kueri dan diverifikasi berkala oleh background worker scheduler (`php artisan schedule:work`).
5. **Ekspor Data Hemat Memori (*Streamed Response*):**
   Ekspor CSV untuk tiket, aset, dan log audit menggunakan pola *streamed response* (pemrosesan bertahap 500 baris dengan UTF-8 BOM), menjaga konsumsi memori backend tetap rendah (< 20MB) meski memproses ribuan data.

---

## 2. Cakupan Modul & Fitur

- **Manajemen Tiket (Helpdesk & Service Desk):**
  - Pembuatan tiket gangguan teknis dan permintaan layanan dengan lampiran berkas terverifikasi ganda (MIME + ekstensi).
  - Alur kerja terstruktur: Penugasan teknisi, eskalasi prioritas, pengerjaan, resolusi, dan penutupan tiket.
  - Thread diskusi kolaboratif (komentar publik dan internal) serta riwayat audit (*timeline*) setiap perubahan tiket.
- **Manajemen Aset TI (IT Asset Management):**
  - Pencatatan inventaris perangkat keras dan lisensi perangkat lunak institusi.
  - Alur penugasan (*assign*) dan pengembalian (*release*) aset ke karyawan dengan pencatatan riwayat pemegang aset.
- **Basis Pengetahuan (Knowledge Base):**
  - Portal artikel panduan teknis mandiri (*self-service*) berformat Markdown dengan sanitasi HTML aman (*rehype-sanitize*).
- **Dashboard Analitik Berbasis Peran:**
  - **Employee:** Status tiket pribadi, daftar aset yang dipegang, dan jalan pintas pembuatan tiket.
  - **Technician:** Beban tiket aktif, metrik kepatuhan SLA personal, dan daftar tiket prioritas.
  - **Manager:** Metrik kepatuhan SLA operasional, tren tiket harian, statistik kinerja teknisi, dan distribusi kategori.
  - **Administrator:** Ringkasan kesehatan sistem, jumlah pengguna aktif, dan jalan pintas pengelolaan master data.
- **Notifikasi & Jejak Audit:**
  - Pusat notifikasi in-app terintegrasi dengan polling efisien 30 detik.
  - Pencatatan audit trail menyeluruh untuk setiap mutasi data, dilengkapi proteksi redaksi data sensitif (masking password).
- **Aksesibilitas & Ergonomi Antarmuka:**
  - Memenuhi standar WCAG 2.2 AA (kontras rasio teruji, navigasi keyboard penuh, *focus trap* pada modal).
  - Tampilan adaptif mobile dengan komponen *Card View*, *Bottom Sheet* filter, dan *Sticky Action Bar* bawah layar.

---

## 3. Matriks Teknologi

| Komponen | Teknologi | Versi | Peran dalam Sistem |
|---|---|---|---|
| **Frontend Framework** | Next.js (App Router) | 16.4+ | Server-Side Rendering, Route Handlers (BFF), Standalone Output |
| **UI Library** | React | 19.x | Concurrent rendering, Server Actions, Action State |
| **CSS Engine** | Tailwind CSS | v4.x | Variabel CSS modern, utilitas performa tinggi |
| **State & Data Fetching** | TanStack Query | v5.x | Caching, revalidasi data, polling notifikasi |
| **Form & Schema Validation** | React Hook Form & Zod | - | Validasi skema input tipe-aman |
| **Visualisasi Data** | Recharts | - | Grafik tren tiket harian dan visualisasi metrik SLA |
| **Backend Framework** | Laravel | 13.x | REST API Engine, Eloquent ORM, Database Migrations |
| **Bahasa Pemrograman Backend** | PHP | 8.4+ | Typed properties, constructor promotion, enumerations |
| **Otentikasi API** | Laravel Sanctum | 4.x | Pengelolaan sesi berbasis Bearer Token di cookie HttpOnly |
| **Server Runtime** | FrankenPHP & Laravel Octane | - | Worker mode untuk pemrosesan HTTP berkecepatan tinggi |
| **Database** | MySQL / MariaDB | 8.4 / 11.x | Relational Database Management System (18 tabel) |
| **Pengujian Backend** | Pest PHP & PHPUnit | 5.x / 13.x | 652 test suite (Unit & Feature) |
| **Pengujian Frontend** | Vitest | v4.x | 138 unit & component tests |
| **Pengujian End-to-End** | Playwright | 1.50+ | 42 skenario E2E multi-device (Desktop & Mobile) |
| **Otomasi CI/CD & Deploy** | GitHub Actions & Docker | - | Linting, automated testing, GHCR container image publish |

---

## 4. Struktur Repositori Monorepo

```
bantuit/
├── apps/
│   ├── api/                   # Backend Laravel 13 (REST API, Policies, Services, DTOs)
│   │   ├── app/
│   │   │   ├── Authorization/ # Matriks hak akses & transisi tiket
│   │   │   ├── DTOs/          # Data Transfer Objects untuk isolasi input
│   │   │   ├── Http/          # Controllers, Requests, Resources
│   │   │   ├── Models/        # 18 Model Eloquent
│   │   │   ├── Policies/      # Otorisasi murni Gate/Policy
│   │   │   └── Services/      # Lapisan logika bisnis & kueri analitik
│   │   ├── database/          # Migrasi skema (18 tabel) & seeders
│   │   ├── routes/            # Rute API RESTful & health check
│   │   └── tests/             # 652 Pest Unit & Feature tests
│   └── web/                   # Frontend Next.js 16 (App Router & BFF)
│       ├── e2e/               # Skenario pengujian Playwright E2E
│       └── src/
│           ├── app/           # Route Groups: (auth) dan (app)
│           ├── components/    # Primitif UI, shared components, dan layout
│           ├── hooks/         # Custom hooks TanStack Query
│           └── lib/           # Klien BFF, parser token, dan utilitas
├── docker/                    # Skrip inisialisasi lingkungan lokal
├── docs/                      # Dokumentasi teknis, ADR, spesifikasi produk & API
├── .github/workflows/         # Pipeline CI testing & release Docker image
├── compose.yaml               # Docker Compose lingkungan pengembangan lokal
├── compose.prod.yaml          # Docker Compose lingkungan produksi (Dokploy ready)
├── compose.prod.octane.yaml   # Konfigurasi override mode Octane FrankenPHP
└── Makefile                   # Perintah otomasi manajemen repositori
```

---

## 5. Akun Pengguna Uji Coba (Demo Accounts)

Sistem menyertakan data awal (*seeders*) untuk simulasi seluruh peran pengguna:

| Peran | Alamat Email | Kata Sandi | Wewenang Operasional |
|---|---|---|---|
| **Employee** | `employee@bantuit.test` | `Password123!` | Membuat tiket kendala, melacak status, mengelola aset pribadi |
| **Technician** | `technician@bantuit.test` | `Password123!` | Mengambil tugas tiket, memperbarui progres pengerjaan, resolusi |
| **Manager** | `manager@bantuit.test` | `Password123!` | Menugaskan teknisi, eskalasi prioritas, evaluasi kepatuhan SLA |
| **Admin** | `admin@bantuit.test` | `Password123!` | Mengelola master data, akun pengguna, kategori, serta audit log |

*(Catatan: Pengguna yang melakukan login pertama kali dengan kata sandi bawaan seeder `password` akan secara otomatis diarahkan ke alur `/ganti-password` sesuai kebijakan keamanan D-17).*

---

## 6. Panduan Menjalankan Lingkungan Lokal

### Prasyarat
- Git
- PHP 8.4+ dan Composer 2.x
- Node.js 22.x dan npm
- Docker & Docker Compose (opsional, disarankan)

### Metode 1: Otomatisasi dengan Docker Compose & Makefile (Direkomendasikan)
1. Clone repositori:
   ```bash
   git clone https://github.com/Febrian1202/JarvisOps.git bantuit
   cd bantuit
   ```
2. Jalankan instalasi awal:
   ```bash
   make setup
   ```
   *Perintah ini akan menyalin `.env.example`, mengunduh dependensi PHP dan Node, membuat application key, menjalankan migrasi, dan menyemai data awal.*
3. Jalankan seluruh layanan:
   ```bash
   make up
   ```
4. Akses antarmuka aplikasi:
   - Frontend Web: `http://localhost:3000`
   - Backend API: `http://localhost:8000`

### Metode 2: Eksekusi Manual pada Host Mesin
1. **Inisialisasi Backend (`apps/api`):**
   ```bash
   cd apps/api
   cp .env.example .env
   composer install
   php artisan key:generate
   php artisan migrate --seed
   php artisan serve --port=8000
   ```
2. **Inisialisasi Frontend (`apps/web`):**
   ```bash
   cd apps/web
   cp .env.example .env
   npm ci
   npm run dev
   ```

---

## 7. Eksekusi Pengujian & Validasi Kualitas

Seluruh suite pengujian otomatis dapat dijalankan secara terisolasi:

### Pengujian Backend (Pest PHP & Linter)
```bash
cd apps/api

# Menjalankan 652 pengujian backend
php artisan test

# Menjalankan pengecekan format kode PHP (Laravel Pint)
vendor/bin/pint --test
```

### Pengujian Frontend (Vitest, Typecheck & Linter)
```bash
cd apps/web

# Verifikasi tipe TypeScript (Strict Mode)
npm run typecheck

# Validasi linter dan format kode
npm run lint
npm run format:check

# Menjalankan 138 unit & component tests
npm run test
```

### Pengujian End-to-End (Playwright Multi-Device)
Pengujian E2E mencakup otentikasi, pembatasan hak akses berbasis peran, *golden path* siklus hidup tiket, audit aksesibilitas, serta layout mobile:
```bash
cd apps/web
npx playwright test
```
*Mencakup eksekusi otomatis pada 3 viewport: Desktop Chromium, Google Pixel 7 (Android), dan Apple iPhone 14 (iOS).*

---

## 8. Panduan Deployment Produksi (Dokploy)

Konfigurasi `compose.prod.yaml` dirancang khusus untuk deployment VPS modern menggunakan Dokploy dengan shared MariaDB:

1. **Koneksi Database Terpusat:**
   Aplikasi backend dan background scheduler langsung terhubung ke layanan MariaDB bersama tanpa membuat kontainer basis data terpisah.
2. **Variabel Lingkungan Produksi:**
   Konfigurasikan variabel lingkungan pada dashboard Dokploy:
   ```env
   APP_ENV=production
   APP_DEBUG=false
   APP_URL=https://api.domainanda.com
   DB_CONNECTION=mysql
   DB_HOST=mariadb        # Nama kontainer atau alamat IP shared database
   DB_PORT=3306
   DB_DATABASE=BantuIT
   DB_USERNAME=bantuit_user
   DB_PASSWORD=kata_sandi_rahasia
   API_BASE_URL=http://api:8000/api
   NEXT_PUBLIC_APP_URL=https://helpdesk.domainanda.com
   ```
3. **Eksekusi Stack Produksi:**
   ```bash
   docker compose -f compose.prod.yaml up -d --build
   ```

---

## 9. Lisensi

Proyek ini dirilis di bawah naungan lisensi terbuka [MIT License](LICENSE).
Dikembangkan untuk memenuhi Capstone Project Rekayasa Sistem Perangkat Lunak.
