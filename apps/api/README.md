# BantuIT — Backend API Service

Service backend RESTful API untuk platform **BantuIT Enterprise IT Service Management (ITSM)**, dibangun menggunakan **Laravel 13** dan **PHP 8.4+** dengan arsitektur *Service Layer Pattern*, penegakan otorisasi *Pure Gate & Policy*, mesin SLA deterministik, serta format respons terstandarisasi.

---

## 1. Arsitektur & Pola Rekayasa Perangkat Lunak

Backend dirancang dengan memisahkan logika bisnis dari lapisan HTTP secara disiplin:

```
[HTTP Request]
       │
       ▼
[FormRequest]  ───────> Validasi input ketat & sanitasi payload
       │
       ▼
[DTO (Data Transfer Object)] ─> Enkapsulasi data bertipe aman (type-safe)
       │
       ▼
[Controller]  ────────> "Thin Controller" (hanya orkestrator alur kerja)
       │
       ▼
[Service Layer] ──────> Logika bisnis, kalkulasi SLA, State Machine, Audit Log
       │
       ▼
[Eloquent Models] ────> Transaksi database & integritas relasi data
       │
       ▼
[JsonResource] ───────> Transformasi data ke format Single Envelope seragam
       │
       ▼
[HTTP Response]
```

### Standar Teknis Utama

1. **Format Single Envelope Konsisten:**
   Seluruh endpoint tanpa pengecualian mengembalikan struktur envelope standar:
   - **Respons Sukses (HTTP 200/201):**
     ```json
     {
       "success": true,
       "message": "Deskripsi hasil operasi",
       "data": { ... },
       "meta": { "total": 100, "page": 1, "per_page": 15 }
     }
     ```
   - **Respons Gagal (HTTP 401/403/404/409/422):**
     ```json
     {
       "success": false,
       "message": "Pesan kesalahan dalam bahasa Inggris",
       "errors": {
         "field_name": ["Pesan validasi bahasa Indonesia"]
       }
     }
     ```
   *(Catatan: HTTP 204 No Content ditiadakan; seluruh respons selalu menyertakan payload JSON).*

2. **Otorisasi Murni Tanpa Third-Party Package:**
   - Hak akses dikendalikan penuh oleh **Laravel Gate & Policy** berbasis `users.role_id` (4 peran: Employee, Technician, Manager, Administrator).
   - Menghindari kompleksitas dan *overhead* kueri dari paket eksternal (seperti spatie/laravel-permission).
   - Aturan khusus Administrator (`Gate::before`) diterapkan secara selektif dengan pengecualian ketat untuk integritas data kepemilikan tiket dan alur eskalasi (ADR D-16).

3. **Deterministic SLA Engine (24/7 Flat Calendar):**
   - Durasi SLA (`sla_duration_minutes`) dan tenggat waktu (`sla_deadline`) dibekukan (*snapshotted*) ke dalam tabel tiket saat tiket pertama kali dibuat. Nilai ini tidak terpengaruh jika master prioritas diubah di kemudian hari.
   - Perhitungan status pelanggaran SLA (`is_breached`) dihitung secara defensif pada waktu kueri (*query time*) serta dipindai secara berkala oleh command penjadwal background (`tickets:check-sla`).

4. **State Machine Tiket dengan Optimistic Concurrency Control:**
   - Status tiket dibatasi pada 5 state: `OPEN` (1), `ASSIGNED` (2), `IN_PROGRESS` (3), `RESOLVED` (4), dan `CLOSED` (5). Tidak ada status jeda (*hold/pause*).
   - Validasi transisi status divalidasi oleh `TicketTransitionMatrix`.
   - Endpoint transisi menerima parameter opsional `expected_status_id`. Jika status saat ini tidak cocok dengan ekspektasi klien, sistem mengembalikan **HTTP 409 Conflict** untuk mencegah *race condition*.

5. **Penyimpanan Berkas Privat & Validasi Ganda:**
   - Lampiran tiket disimpan di storage disk privat (`local_private`) dan hanya dapat diunduh melalui endpoint terproteksi yang memverifikasi `AttachmentPolicy`.
   - Validasi berkas memverifikasi MIME type aktual dan ekstensi berkas secara ganda (JPG, JPEG, PNG, PDF; batas maksimal 5 MB).

6. **Ekspor CSV Hemat Memori (*Streamed Response*):**
   - Fitur ekspor data tiket, aset, dan log audit menggunakan `StreamedResponse` dengan pemrosesan chunk 500 baris dan *byte order mark* (BOM) UTF-8, menjaga penggunaan memori PHP di bawah 20 MB.

---

## 2. Struktur Direktori `app/`

```
app/
├── Authorization/             # Matriks hak akses & transisi tiket
│   ├── AbilityMatrix.php      # Definisi 66 hak akses granular per peran
│   └── TicketTransitionMatrix.php # Aturan transisi status tiket yang diizinkan
├── Console/
│   └── Commands/              # Scheduled commands (e.g. CheckSlaBreachesCommand)
├── DTOs/                      # Data Transfer Objects untuk isolasi input
│   ├── TicketDTO.php
│   ├── AssetDTO.php
│   └── ...
├── Enums/                     # PHP 8.4 Enums (RoleEnum, TicketStatusEnum, PriorityEnum)
├── Http/
│   ├── Controllers/           # Thin Controllers (Auth, Ticket, Asset, KB, Dashboard, dll.)
│   ├── Middleware/            # ForcePasswordChange, VerifyCsrfToken, dll.
│   ├── Requests/              # FormRequests dengan aturan validasi & pesan berbahasa Indonesia
│   └── Resources/             # JsonResource untuk enkapsulasi envelope respons
├── Models/                    # 18 Model Eloquent dengan relasi data dan scope kueri
├── Policies/                  # Laravel Policies (Ticket, Asset, Article, Attachment, dll.)
├── Providers/                 # Service Providers & registrasi Gate
└── Services/                  # Lapisan logika bisnis
    ├── TicketService.php
    ├── SlaService.php
    ├── AuditLogger.php
    ├── AssetService.php
    ├── ArticleService.php
    └── Dashboards/            # Layanan agregasi analitik untuk 4 peran
```

---

## 3. Ringkasan Endpoint API

| Kelompok | Method | Endpoint | Keterangan |
|---|---|---|---|
| **Otentikasi** | `POST` | `/api/auth/login` | Login & terbitkan Bearer Token |
| | `POST` | `/api/auth/logout` | Revoke token Sanctum aktif |
| | `GET` | `/api/auth/me` | Profil pengguna saat ini & daftar *abilities* |
| | `POST` | `/api/auth/change-password` | Ganti kata sandi mandiri |
| **Tiket** | `GET` | `/api/tickets` | Daftar tiket (filter, pencarian, sort, pagination) |
| | `POST` | `/api/tickets` | Buat tiket baru |
| | `GET` | `/api/tickets/{id}` | Detail tiket, riwayat status, dan komentar |
| | `PATCH` | `/api/tickets/{id}/assign` | Tugaskan tiket ke teknisi |
| | `PATCH` | `/api/tickets/{id}/status` | Perbarui status tiket |
| | `PATCH` | `/api/tickets/{id}/priority` | Eskalasi tingkat prioritas tiket |
| | `POST` | `/api/tickets/{id}/comments` | Tambah komentar publik atau catatan internal |
| | `GET` | `/api/tickets/export/csv` | Ekspor data tiket format CSV (*streamed*) |
| **Aset TI** | `GET` | `/api/assets` | Inventaris aset perusahaan |
| | `POST` | `/api/assets` | Tambah aset baru |
| | `PATCH` | `/api/assets/{id}/assign` | Serah terima aset ke pengguna |
| | `PATCH` | `/api/assets/{id}/release` | Pengembalian aset ke gudang/stok |
| | `GET` | `/api/my-assets` | Aset yang sedang dipinjam oleh user aktif |
| **Knowledge Base** | `GET` | `/api/articles` | Pencarian & daftar artikel basis pengetahuan |
| | `GET` | `/api/articles/{slug}` | Detail artikel teknis (Markdown) |
| | `POST` | `/api/articles` | Buat artikel baru (Draft/Published) |
| **Dashboard** | `GET` | `/api/dashboard/employee` | Metrik & ringkasan operasional Karyawan |
| | `GET` | `/api/dashboard/technician` | Metrik beban kerja & SLA Teknisi |
| | `GET` | `/api/dashboard/manager` | Metrik analitik performa SLA & tren tiket Manajer |
| | `GET` | `/api/dashboard/admin` | Kesehatan sistem & ringkasan Master Data Admin |
| **Audit & Notifikasi** | `GET` | `/api/audit-logs` | Jejak audit mutasi data (filter rentang tanggal) |
| | `GET` | `/api/notifications` | Daftar notifikasi in-app pengguna |
| | `PATCH` | `/api/notifications/{id}/read` | Tandai notifikasi telah dibaca |

---

## 4. Skema Basis Data & Pinned Seeder IDs

Basis data terdiri dari 18 tabel dengan integritas referensial dan indeks komposit untuk optimasi kueri:
- **Tabel Pengguna & Struktur:** `roles`, `departments`, `users`
- **Tabel Tiket & Siklus Hidup:** `ticket_statuses`, `ticket_priorities`, `ticket_categories`, `tickets`, `ticket_comments`, `ticket_status_histories`, `ticket_attachments`
- **Tabel Aset TI:** `asset_categories`, `assets`, `asset_assignments`
- **Tabel Basis Pengetahuan:** `knowledge_categories`, `knowledge_articles`
- **Tabel Sistem:** `audit_logs`, `notifications`, `personal_access_tokens`

### ID Master Terpin (Pinned IDs — ADR D-15)
Pengujian dan logika sistem mengandalkan ID statis berikut untuk menjamin determinisme:
- **Peran:** `1: Employee`, `2: Technician`, `3: Manager`, `4: Administrator`
- **Status Tiket:** `1: OPEN`, `2: ASSIGNED`, `3: IN_PROGRESS`, `4: RESOLVED`, `5: CLOSED`
- **Prioritas Tiket:** `1: LOW`, `2: MEDIUM`, `3: HIGH`, `4: URGENT`

---

## 5. Instalasi & Menjalankan Backend Secara Lokal

### Prasyarat
- PHP 8.4+ dengan ekstensi: `pdo_mysql`, `mbstring`, `fileinfo`, `intl`, `bcmath`
- Composer 2.x
- Server MySQL 8.4 atau MariaDB 11.x

### Langkah Instalasi
1. Masuk ke direktori `apps/api`:
   ```bash
   cd apps/api
   ```
2. Salin environment configuration:
   ```bash
   cp .env.example .env
   ```
3. Pasang dependensi PHP via Composer:
   ```bash
   composer install
   ```
4. Generate kunci enkripsi aplikasi:
   ```bash
   php artisan key:generate
   ```
5. Konfigurasi kredensial database pada berkas `.env`, lalu jalankan migrasi dan seeder:
   ```bash
   php artisan migrate --seed
   ```
6. Jalankan server lokal:
   ```bash
   php artisan serve --port=8000
   ```
7. Jalankan scheduler SLA di sesi terminal terpisah:
   ```bash
   php artisan schedule:work
   ```

---

## 6. Pengujian Otomatis & Pemeliharaan Kode

### Eksekusi Pengujian (Pest PHP)
Suite pengujian dijalankan pada memori SQLite (`:memory:`) untuk kecepatan dan isolasi state:
```bash
# Menjalankan seluruh test suite (652 tests)
php artisan test

# Menjalankan test secara spesifik berdasarkan nama class/fitur
php artisan test --filter=TicketLifecycleTest

# Menjalankan langsung melalui binary Pest
vendor/bin/pest
```

### Format Kode & Linting (Laravel Pint)
Format kode mengikuti panduan gaya PSR-12 dan konvensi Laravel resmi:
```bash
# Memeriksa kepatuhan format tanpa mengubah berkas
vendor/bin/pint --test

# Memperbaiki format kode secara otomatis pada berkas yang diubah
vendor/bin/pint --dirty
```

---

## 7. Kesiapan Runtime Produksi (Octane & FrankenPHP)

Aplikasi telah diuji dan siap dijalankan menggunakan server berkinerja tinggi **Laravel Octane** dengan engine **FrankenPHP**:
- State leak prevention: Layanan singleton yang menyimpan state request diuji melalui `OctaneStateLeakTest`.
- Konfigurasi `config/octane.php` telah disesuaikan untuk manajemen memori worker dan re-inisialisasi context antarsiklus request.
- Skrip `docker-entrypoint.sh` secara otomatis memvalidasi kesiapan database sebelum menjalankan migrasi paksa dan server Octane di lingkungan container.
