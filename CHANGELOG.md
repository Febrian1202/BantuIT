# Changelog

Semua perubahan penting pada proyek **BantuIT** akan didokumentasikan dalam berkas ini.
Format ini mengacu pada [Keep a Changelog](https://keepachangelog.com/id/1.0.0/) dan mematuhi [Semantic Versioning](https://semver.org/).

---

## [1.0.10] - 2026-10-17

### Added
- **Dynamic Application Versioning (`/api/version` & `useAppVersion`):**
  - Membuat route handler dinamis `apps/web/src/app/api/version/route.ts` (`force-dynamic`, `no-store`) untuk mengekspos versi aplikasi runtime container (`NEXT_PUBLIC_APP_VERSION` atau `APP_VERSION`) ke antarmuka klien.
  - Menambahkan utilitas `formatAppVersion()` pada `apps/web/src/lib/version.ts` dan hook klien `useAppVersion()` pada `apps/web/src/hooks/use-app-version.ts` untuk menyinkronkan label versi footer tanpa layout shift.
  - Meneruskan prop `appVersion` dari Server Component `apps/web/src/app/(app)/layout.tsx` ke komponen `AppSidebar`, `AppTopbar`, dan `MobileNav`.
  - Menambahkan dukungan argumen build `ARG NEXT_PUBLIC_APP_VERSION` dan `ARG APP_VERSION` pada `apps/web/Dockerfile`, serta menyalurkan tag rilis via workflow `.github/workflows/release.yml`.
  - Mengonfigurasi variabel `NEXT_PUBLIC_APP_VERSION` dan `APP_VERSION` pada container service `web` di `compose.prod.yaml` dan `compose.prod.octane.yaml`.
  - Menambahkan test suite `apps/web/src/test/shell/app-version.test.tsx` (7 pengujian).

### Changed
- **Penyelarasan Bahasa Label Role Pengguna:**
  - Memperbarui fungsi `getRoleLabel()` pada `apps/web/src/lib/labels.ts` agar mendukung penanganan case-insensitive untuk role pengguna (`Administrator`, `Manager`, `Teknisi`, `Karyawan`).
  - Mengubah opsi pemilihan role pada dropdown form pengguna (`apps/web/src/components/admin/UserFormDialog.tsx`) dan dropdown filter pengguna (`apps/web/src/components/admin/UserFilters.tsx`) sehingga menampilkan nama peran dalam bahasa Indonesia baku.

### Fixed
- **Integrasi Shadcn/UI Date Picker pada Tanggal Bergabung:**
  - Mengganti elemen native `<input type="date">` pada modal pembuatan dan pengubahan pengguna (`UserFormDialog`) dengan komponen shadcn/ui `Popover` dan `Calendar` (mode single, terformat lokal Indonesia, dilengkapi tombol bersihkan tanggal).
  - Menambahkan test suite `apps/web/src/test/admin/user-form-dialog.test.tsx` (3 pengujian).
- **Styling Scrollbar Modal Dialog:**
  - Menambahkan token desain `--scrollbar-thumb` dan `--scrollbar-thumb-hover` untuk mode terang dan gelap pada `apps/web/src/app/globals.css`.
  - Menambahkan utilitas `@utility scrollbar-thin` dan menerapkannya pada `DialogContent` `UserFormDialog` agar scrollbar modal tampil tipis (6px) dan sesuai dengan skema warna desain sistem.

---

## [1.0.9] - 2026-10-16

### Fixed
- **Deskripsi Audit Log Pembuatan Tiket (`TicketService`):**
  - Memperbaiki kesalahan ketik (typo) pemanggilan properti `$ticket->ticket_numner` menjadi `$ticket->ticket_number` pada `apps/api/app/Services/Ticket/TicketService.php`. Sebelumnya, kesalahan ketik ini menyebabkan nomor tiket bernilai kosong (`null`) pada log audit sehingga pesan deskripsi tercatat sebagai `"Ticket # dibuat."`. Setelah perbaikan, nomor tiket terinterpolasi dengan benar menjadi `"Ticket #TCK-XXXX dibuat."`.
  - Menambahkan pengujian regresi eksplisit pada `apps/api/tests/Feature/Ticket/CreateTicketTest.php` untuk memverifikasi pencatatan deskripsi audit log saat tiket baru dibuat.
- **Deskripsi Audit Log Penghapusan Lampiran Tiket (`TicketAttachmentServices`):**
  - Memperbaiki kesalahan ketik (typo) properti `$attachment->originale_filename` menjadi `$attachment->original_filename` pada `apps/api/app/Services/Ticket/TicketAttachmentServices.php` saat mencatat log audit aksi penghapusan berkas lampiran tiket.
  - Memperbarui pengujian regresi pada `apps/api/tests/Feature/Attachment/AttachmentDeleteTest.php` untuk memvalidasi teks deskripsi penghapusan berkas lampiran tiket.

---

## [1.0.8] - 2026-10-16

### Fixed
- **Aksi Hapus Tiket pada Detail Tiket (`useTicketActions`):**
  - Memperbaiki penanganan aksi `"delete"` pada fungsi `openDialog` di `apps/web/src/hooks/use-ticket-actions.ts` dengan menambahkan `case "delete": setActiveDialog("delete"); break;`. Sebelumnya aksi `"delete"` jatuh ke percabangan `default` yang menyetel dialog ke `null`, mengakibatkan dialog konfirmasi (`ConfirmDialog`) tidak pernah terbuka dan request penghapusan ke API tidak terkirim.
  - Memperbarui definisi tipe `TicketActionOrDelete` untuk mencakup aksi administratif penghapusan tiket pada `useTicketActions` dan `ActionDialogs`.
  - Menghapus unsafe casting `action as TicketAction` pada `apps/web/src/app/(app)/tickets/[id]/page-client.tsx`.
  - Menambahkan pengujian unit di `apps/web/src/test/Ticket/ticket-detail-banner.test.tsx` dan `apps/web/src/test/Ticket/use-ticket-actions.test.tsx` guna mengunci alur pembukaan dialog konfirmasi penghapusan tiket.

---

## [1.0.7] - 2026-10-16

### Fixed
- **Judul Halaman pada Topbar (`AppTopbar`):**
  - Memperbaiki pemetaan judul modul pada fungsi `getPageTitle` di `apps/web/src/components/shell/app-topbar.tsx` agar rute `/tickets` (dan subrutenya seperti `/tickets/new` dan `/tickets/[id]`) menampilkan teks judul `"Daftar Tiket"` alih-alih jatuh ke fallback default `"BantuIT"`.
  - Menyesuaikan nilai fallback default judul halaman menjadi format kapitalisasi brand standar `"BantuIT"`.
  - Menambahkan test suite unit testing di `apps/web/src/test/shell/topbar.test.tsx` untuk memastikan konsistensi judul modul di Topbar untuk rute tiket dan modul lainnya.

---

## [1.0.6] - 2026-10-15

### Added
- **Halaman Pusat Notifikasi (`/notifications`):**
  - Mengimplementasikan halaman penuh Pusat Notifikasi yang terintegrasi dengan tautan *"Lihat semua notifikasi"* di dropdown notifikasi Topbar.
  - Mengikuti konvensi arsitektur Next.js App Router standar proyek dengan Server Component `page.tsx` (menyediakan metadata `Pusat Notifikasi | BantuIT` dan Suspense fallback) serta Client Component `page-client.tsx`.
  - Fitur lengkap:
    - Tab filter status notifikasi (`Semua`, `Belum Dibaca`, `Sudah Dibaca`) yang tersinkronisasi via query parameters URL.
    - Tombol aksi *"Tandai Semua Dibaca"* untuk menandai seluruh notifikasi sekaligus.
    - Aksi per notifikasi: tombol *"Buka Tiket"* yang mengarahkan ke halaman detail tiket bersangkutan dan otomatis menandai notifikasi telah dibaca, serta tombol aksi centang untuk menandai dibaca tanpa berpindah halaman.
    - Ikon semantik berdasarkan tipe peristiwa notifikasi (`TICKET_SLA_BREACHED`, `TICKET_COMMENTED`, dan penugasan/perubahan status tiket).
    - Status kosong (`EmptyState`) dan kontrol paginasi (`DataTablePagination`).
  - Menambahkan test suite unit testing di `apps/web/src/test/notifications/notifications-page.test.tsx`.

---

## [1.0.5] - 2026-10-15

### Added
- **Kartu Pintasan Akun Demo (1-Click Fill) di Halaman Login:**
  - Menambahkan komponen terpisah `DemoAccountCards` (`apps/web/src/components/auth/demo-account-cards.tsx`) yang diletakkan di bawah form login (`LoginPage`).
  - Menggunakan tata letak 1 kartu per baris (single-column stack) dengan ikon role, deskripsi peran, dan email yang rapi serta nyaman di layar mobile maupun desktop.
  - Menyediakan 4 kartu akun demo yang mencakup seluruh peran pengguna di sistem:
    - **Administrator:** `admin@bantuit.test` (Admin Sistem)
    - **Manager:** `manager@bantuit.test` (IT Manager)
    - **Technician:** `technician@bantuit.test` (Teknisi IT)
    - **Employee:** `employee@bantuit.test` (Karyawan)
  - Mengklik salah satu kartu akan mengisi otomatis nilai field email dan kata sandi (`Password123!`) pada form login dan menghapus pesan error yang aktif untuk presentasi dan demo yang cepat.
  - Menambahkan test suite unit testing pada `apps/web/src/test/auth/demo-account-cards.test.tsx` dan `apps/web/src/test/auth/login-demo-accounts.test.tsx`.

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
