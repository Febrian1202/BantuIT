# Changelog

Semua perubahan penting pada proyek **BantuIT** akan didokumentasikan dalam berkas ini.
Format ini mengacu pada [Keep a Changelog](https://keepachangelog.com/id/1.0.0/) dan mematuhi [Semantic Versioning](https://semver.org/).

---

## [1.0.4] - 2026-10-15

### Changed
- **Tipografi Brand Header Sidebar:**
  - Memperbesar ukuran teks dan menebalkan tulisan "BantuIT" di sidebar navigasi desktop (`AppSidebar`) serta drawer navigasi mobile (`MobileNav`) menggunakan kelas `text-foreground text-xl font-bold tracking-tight`.

### Fixed
- **Alur Logout & Penghapusan Sesi:**
  - Memperbaiki alur logout agar langsung mengarahkan pengguna ke halaman `/login` seketika melalui navigasi hard redirect (`window.location.href = "/login"`), membersihkan seluruh memori sesi TanStack Query dan state React secara total.
  - Memastikan penghapusan cookie sesi `auth_token` di Next.js Route Handler (`/api/auth/logout`) dan session helper dilakukan dengan parameter `path: "/"`, `maxAge: 0`, dan tanggal kadaluarsa definitif.
  - Menambahkan feedback status `Keluar...` dan pencegahan multiple-click (`e.preventDefault()`) pada menu logout di dropdown profil (`NavUser`).

---

## [1.0.3] - 2026-10-15

### Fixed
- **Frontend App Shell Navigation:**
  - Memperbaiki penandaan status aktif (`isActive`) pada menu sidebar desktop (`AppSidebar`) dan mobile drawer (`MobileNav`) untuk halaman Dashboard. Sebelumnya pengecekan hanya menguji `pathname === "/"`, sedangkan halaman dashboard pengguna terdistribusi ke rute berbasis peran (`/dashboard/admin`, `/dashboard/technician`, `/dashboard/manager`, `/dashboard/employee`).
  - Menambahkan atribut aksesibilitas `aria-current="page"` pada link navigasi yang sedang aktif.
  - Menambahkan automated unit test pada `apps/web/src/test/shell/sidebar-highlight.test.tsx` untuk menjamin highlight dashboard selalu aktif pada `/` dan `/dashboard/*`.

---

## [1.0.2] - 2026-10-15

### Fixed
- **Docker Compose Production:**
  - Menambahkan `pull_policy: always` pada semua layanan di `compose.prod.yaml` agar Dokploy/Docker selalu menarik image container terbaru dari GHCR dan tidak menggunakan image cache lama di VPS.
  - Menghapus pemanggilan `view:cache` sepenuhnya dari `apps/api/docker-entrypoint.sh` karena backend BantuIT adalah RESTful API murni tanpa view Blade.

---

## [1.0.1] - 2026-10-15

### Fixed
- **Backend API Booting & Entrypoint:**
  - Memperbaiki pengecekan direktori `resources/views` pada `docker-entrypoint.sh` agar tidak mengeksekusi `php artisan view:cache` jika direktori tidak ada atau kosong, mencegah crash startup di container produksi.
  - Menambahkan direktori `resources/views` dengan file `.gitkeep` dan penanganan `mkdir -p` di `Dockerfile` untuk mencegah fatal error Symfony Finder.
  - Menghubungkan stack Compose produksi ke jaringan eksternal `dokploy-network`.

---

## [1.0.0] - 2026-10-15

### Added
- **Backend API (Laravel 13 & PHP 8.4):**
  - Implementasi *Service Layer Pattern* dan *Data Transfer Objects* (DTO) untuk memisahkan logika bisnis dari HTTP controller (*Thin Controller*).
  - Standarisasi format respons *Single Envelope* untuk seluruh endpoint (`{ success, message, data, meta? }` dan `{ success, message, errors }`).
  - Otorisasi murni menggunakan Laravel Gate & Policy berbasis `role_id` (4 peran: Employee, Technician, Manager, Administrator) tanpa ketergantungan paket pihak ketiga.
  - *Deterministic SLA Engine* dengan snapshot batas waktu 24/7 dan kalkulasi defensif pelanggaran (*breach status*).
  - *State Machine* siklus hidup tiket dengan 5 status terdefinisi (`OPEN`, `ASSIGNED`, `IN_PROGRESS`, `RESOLVED`, `CLOSED`) dan pencegahan *race condition* menggunakan `expected_status_id` (HTTP 409).
  - Modul Manajemen Aset TI (perangkat keras & lisensi perangkat lunak) dengan alur *assignment*, *release*, dan jejak kepemilikan.
  - Basis Pengetahuan (*Knowledge Base*) mandiri berformat Markdown dengan sanitasi HTML (*rehype-sanitize*).
  - Endpoint analitik dashboard khusus untuk 4 peran pengguna.
  - Pusat notifikasi in-app dan pencatatan jejak audit (*Audit Trail*).
  - Ekspor data tiket, aset, dan audit log berformat CSV dengan *streamed response* hemat memori.
- **Frontend Web (Next.js 16, React 19, & Tailwind CSS v4):**
  - Arsitektur *Backend-for-Frontend* (BFF) melalui Route Handler `/api/proxy/[...path]` yang mengisolasi token otentikasi Sanctum di dalam cookie `httpOnly`.
  - Sinkronisasi state dan caching menggunakan TanStack Query v5 dengan polling notifikasi hemat daya setiap 30 detik.
  - 16 komponen primitif UI berbasis Radix UI dan 9 komponen bisnis bersama (*DataTable*, *FilterBar*, *SearchInput*, *StatusBadge*, *PriorityBadge*, *SlaIndicator*, *RelativeTime*, *FileUpload*, *ConfirmDialog*, *EmptyState*).
  - Adaptasi tampilan responsif mobile: *Card View adapter*, *Bottom Sheet* filter, *Sticky Mobile Action Bar*, dan kepatuhan area sentuh minimal 44x44 piksel.
  - Kepatuhan aksesibilitas berstandar WCAG 2.2 AA (navigasi keyboard, *focus trapping*, kontras warna semantik).
  - Visualisasi grafik tren tiket dan metrik operasional interaktif menggunakan Recharts.
- **Infrastruktur, DevOps & Deployment:**
  - Multi-stage Dockerfile produksi untuk backend (FrankenPHP / Octane) dan frontend (Next.js standalone runner berbasis Node 22 Alpine).
  - Konfigurasi `compose.prod.yaml` yang dioptimalkan untuk VPS Dokploy dengan integrasi *shared MariaDB*.
  - Template konfigurasi lingkungan produksi `env.prod.example` untuk Dokploy.
  - Pipeline GitHub Actions untuk CI testing (`ci.yaml`) dan rilis otomatis container image ke GitHub Container Registry (`release.yml`).
  - Skrip pengemasan dan verifikasi submission bersih (`make package`).

### Security
- Isolasi penuh token otentikasi Bearer pada cookie browser dengan atribut `httpOnly`, `Secure`, dan `SameSite=Lax`.
- Validasi ganda tipe MIME aktual dan ekstensi berkas untuk lampiran tiket privat.
- Redaksi otomatis (*masking*) data sensitif (seperti kata sandi) pada pencatatan log audit.
- Alur wajib ganti kata sandi sementara (`/ganti-password`) pada login perdana.
- Proteksi rate limiting berbasis IP pada endpoint otentikasi.
