# AGENTS.md — BantuIT (JARVIS OPS)

IT Service Management monorepo. **Production release is active (Current Release — Tag `v1.0.14`), and the Rust API rewrite is actively in progress.** `docs/` is dense and authoritative (~6.3k lines, Indonesian); the codebase consists of a complete production-grade Laravel API + Next.js frontend, accompanied by an active native rewrite using Rust, Axum, SQLx, and PostgreSQL.

Production release highlights: 18 models, 24 migrations, 652 passing Pest backend tests (2879 assertions), full ticket lifecycle with authorization & workflows, background SLA breach scheduler, notification API with 30s background-safe polling & notification center, audit log API with scoping & timezone filters, asset management with assignment lifecycle & quick status change modal, knowledge base with HTML sanitization, 4 role-based dashboard APIs, complete Frontend Foundation & App Shell (Plus Jakarta Sans, warm cream theme `#f7f4ed`), 16 baseline UI + 9 shared components, 177 passing Vitest tests across 41 files, Playwright automated E2E testing suite, WCAG 2.2 AA accessibility compliance, multi-stage production Docker stack, Dokploy deployment, streamed CSV export, dynamic runtime versioning (`/api/version`), mobile responsive ergonomics, and critical security mitigations for PHP (CVE-2026-109618, CVE-2026-109619) and Node.js (CVE-2026-111497, CVE-2026-105481).

## Repo layout

```
apps/api/       Laravel 13 + Sanctum 4 (PHP ^8.3, FrankenPHP / Octane, MySQL 8.4)
apps/web/       Next.js 16.4 + React 19.2 + Tailwind CSS v4 (App Router, src/app)
apps/axum-api/  Rust 2024 + Axum 0.8 + SQLx 0.9 + PostgreSQL 17 (Hybrid UUID v7, structured tracing)
docs/           approved design docs & implementation guides — the spec
PRODUCT.md      product truth for the impeccable design skill
```

One `.git` at the root (`origin` = github.com/Febrian1202/JarvisOps, branch `main`). Root manifest `compose.yaml` orchestrates `postgres` (PostgreSQL 17 on 5432), `mysql` (MySQL 8.4 on 3307->3306), `api` (FrankenPHP Laravel on 8000), `scheduler` (SLA scheduler), and `web` (Next.js on 3000).

## Docs are the spec — read before coding

These docs contain locked decisions, not drafts. Do not re-litigate them.

| Before you touch | Read |
| --- | --- |
| anything | `docs/adr/DECISIONS.md` — D-01..D-24, closes ~50 decisions that would otherwise be made silently and inconsistently |
| migrations / models | `docs/architecture/ERD.md`, `docs/schema.sql` (18 tables), DECISIONS §A (esp. D-14 column+index checklist, D-15 pinned seeder IDs) |
| controllers / routes | `docs/api/API-CONTRACT.md` (envelope, §13 server-set fields), DECISIONS §C |
| application / service-layer pattern | `docs/architecture/BACKEND-ARCHITECTURE.md` (thin controllers, DTO layer, service layer, request pipeline, folder structure) — derived doc, no new decisions |
| frontend / UI architecture | `docs/architecture/FRONTEND-ARCHITECTURE.md` (BFF proxy, httpOnly cookie, folder structure, components, state) — derived doc, no new decisions |
| ticket status logic | `docs/product/STATUS-TRANSITION.md` |
| policies / gates | `docs/product/PERMISSION-MATRIX.md` (incl. when to return 404 instead of 403), DECISIONS §B |
| what to build next | `docs/product/ROADMAP.md` (10 phases; §2 = locked technical decisions) |
| Rust rewrite / migration | `docs/RUST-AXUM-IMPLEMENTATION-GUIDE.md` — full rewrite spec, Axum, SQLx, PostgreSQL, structured logging, 1:1 API parity |

**Authority order when docs conflict** (DECISIONS §2): `DECISIONS.md` > `STATUS-TRANSITION.md` / `PERMISSION-MATRIX.md` > `API-CONTRACT.md` > `ERD.md` / `schema.sql` > `ROADMAP.md` > `PRD.md`. PRD is last because it was written first.

For code: migrations are the schema source of truth. `docs/schema.sql` is a report attachment, synced in Phase 10 — not executable. `docs/api/openapi.yaml` only covers 8 paths / 20 operations, so it is a partial export — `API-CONTRACT.md` and `PERMISSION-MATRIX.md` (65 route rows) are the real inventory.

`docs/design/wireframe.pen` is an encrypted design file: access it only through the `pencil` MCP tools. Never `read`/`grep` it.

## Hard conventions that differ from framework defaults

- **Every API response uses one envelope**: `{ success, message, data, meta? }` / `{ success, message, errors }`. Never return a bare Eloquent Resource or a raw 422 body. No `links` in `meta`. `204` is never used — always return a body.
- **Frontend never calls Laravel directly.** Next.js Route Handlers act as a BFF; the Sanctum token lives in an httpOnly cookie and is attached server-side. A client component hitting `http://localhost:8000` is a bug.
- **Authorization is core Laravel Gate/Policy only.** One role per user via `users.role_id`. Admin passes `Gate::before` with two hard exceptions (D-16). Do not add `spatie/laravel-permission`.
- **SLA is snapshotted onto each ticket** (`sla_duration_minutes`, `sla_deadline`) at creation — never joined live from the priority table. Flat 24/7 calendar (D-01), exactly 5 statuses with no pause (D-02). Breach state is also recomputed defensively at query time, not trusted from the scheduler.
- Every create returns **201** (D-22). Status/assign transitions accept optional `expected_status_id` and return **409** on mismatch (D-21).
- All durations are integer **minutes**; timestamps ISO 8601 UTC (server and DB are UTC, frontend converts to Asia/Jakarta); fields `snake_case`; routes `kebab-case` plural.
- **Message language is split** (D-24): API envelope in English, validation errors (custom `messages()`) in Indonesian, in-app notification bodies and audit descriptions in Indonesian.
- Attachments go to a private disk, served only through a controller that runs the parent ticket's policy. 5 MB max; JPG/JPEG/PNG/PDF only; validate MIME **and** extension server-side.
- Notifications are DB rows + 30s frontend polling. No WebSocket, no broadcasting, no email/SMTP.
- The SLA scheduler is its own process (`php artisan schedule:work`, separate container in prod) — FrankenPHP classic mode only serves HTTP, so without it SLA checks never run.
- Frontend stack is pre-chosen (ROADMAP §2): shadcn/ui, TanStack Query, Recharts, react-hook-form, zod. Don't introduce alternatives.
- Docs and user-facing prose are **Indonesian**. Match it when editing `docs/`, `README.md`, or `PRODUCT.md`.
- Branch strategy (ROADMAP Phase 0): work on `feat/<phase>-<topic>`, merge to `main` via PR even when solo.
- **Git Tags & Changelog:** Setiap kali membuat atau merilis Git tag baru (misal `vX.Y.Z`), **wajib** memperbarui `CHANGELOG.md` terlebih dahulu dengan rincian perubahan (`Added`, `Changed`, `Fixed`, dsb.) sebelum menyematkan tag dan melakukan push.

## Backend commands (`apps/api`)

```bash
php artisan test                         # run Pest test suite (652 tests, 2879 assertions)
php artisan test --filter=SomeTest       # narrowest run; prefer this for focused testing
vendor/bin/pest --filter=SomeTest        # Pest runner directly
vendor/bin/pint --dirty --format agent   # required after editing any PHP
composer dev                             # php artisan dev (concurrent dev processes)
```

`laravel/pao` is installed, so `pint` and tests emit single-line JSON instead of the usual TAP-ish output. That is expected, not a misconfiguration.

Pest 5 is installed with `pest-plugin-laravel` and PHPUnit 13.3. Tests run against **SQLite in-memory** (`phpunit.xml`) while `apps/api/.env` points at **MySQL** (`DB_DATABASE=JarvisOps` / `BantuIT`, jarvisops/secret in Docker, root/root local).

Health endpoint: `/api/health` (`app/Http/Controllers/HealthController.php`), not the `/up` from bootstrap. API exceptions already render as JSON for `api/*`.

## Frontend commands (`apps/web`)

```bash
npm run dev        # next dev
npm run build      # next build (production build)
npm run lint       # eslint
npm run typecheck  # tsc --noEmit
npm run test       # vitest run (177 unit & component tests across 41 files)
npm run test:e2e   # playwright test
```

`apps/web/.env.example` exists and contains the internal BFF URL.

## Rust Axum API commands (`apps/axum-api`)

```bash
cargo check                              # quick compile check
cargo test                               # run test suite (10 unit tests passing in tests/tests.rs)
cargo run --bin bantuit-server           # run Axum HTTP server on port 8000
cargo run --bin seed                     # run PostgreSQL initial database seeder
sqlx migrate run --source migrations     # execute pending SQLx database migrations
```

Crate `bantuit-api` uses Rust 2024 edition, Axum 0.8, Tokio 1.54, SQLx 0.9, and PostgreSQL 17. Environment configuration is read from `.env` via `dotenvy`.

## Instruction files and tooling

- `apps/api/AGENTS.md` is **generated by Laravel Boost** and `apps/web/AGENTS.md` is **regenerated by `next dev`**. Don't hand-edit expecting it to survive; commit the regenerated block rather than reverting it.
- `apps/api/CLAUDE.md` is the stale pre-Boost bootstrap stub telling you to install Boost. Boost 2.7+ is already installed — ignore it.
- Boost's `.ai/rules` mechanism is referenced by `apps/api/AGENTS.md` but the directory does not exist. Skip that step until it does.
- `apps/api/boost.json` registers the `laravel-boost` MCP server; config resolution walks up from cwd. Start antigravity in `apps/api` for direct backend work.
- Skills live in `.antigravity/skills/`: `laravel-best-practices`, `testing-best-practices`, `tailwindcss-development`, `infer-conventions`, `impeccable`.
- UI work goes through the `impeccable` skill. Its product truth is the tracked root `PRODUCT.md`; its settings are `.impeccable/config.json` (`buildPath: comp` — new surfaces start from a generated comp). The visual spec is root `DESIGN.md` (warm cream theme); it is implemented via the Tailwind v4 theme in `apps/web`, see `docs/architecture/FRONTEND-ARCHITECTURE.md` §3.
- **Frontend work (Phases 8+) must load the same skills used by the phase-8 sessions:** `impeccable`, `next-best-practices`, `vercel-react-best-practices`, `shadcn`, `frontend-design`, `tailwindcss-development`, `test-driven-development`. Invoke them via the `skill` tool before writing any frontend code (see `docs/tasks/phase-8/README.md` and `docs/tasks/phase-9/README.md`).

## Current state, concretely

### 1. Production Monorepo (`v1.0.14`)
- **Backend API (`apps/api`)**: 23 migrations, 18 models, 37 API routes (53 endpoint operations), 652 passing Pest tests (2879 assertions). Core ticket lifecycle CRUD + transitions + comments + history + query + references, background SLA breach scheduler (`tickets:check-sla`), full notification API (`/api/notifications`), full audit log API (`/api/audit-logs`), full asset management API (`/api/assets`, `/api/my-assets`, `/api/assets/{id}/assign`, `/api/assets/{id}/release`, `/api/assets/{id}/history`), full knowledge base API (`/api/articles`, `/api/articles/{slug}`, `/api/articles/{id}/publish`, `/api/articles/{id}/unpublish`, `/api/knowledge-categories`), full dashboard APIs (`/api/dashboard/employee`, `/api/dashboard/technician`, `/api/dashboard/manager`, `/api/dashboard/admin`), and streamed CSV export. Hardened with PHP 8.4.27, `security.ini` limits, and `--max-requests=500` worker recycling against CVE-2026-109618 & CVE-2026-109619.
- **Frontend Web (`apps/web`)**: Next.js 16.4 + React 19.2 + Tailwind CSS v4, Plus Jakarta Sans, warm-neutral theme (`#f7f4ed`), Route Groups `(auth)` dan `(app)`, responsive App Shell (desktop Sidebar + mobile Drawer), Topbar dengan 30s background-safe polling notifikasi, Pusat Notifikasi penuh (`/notifications`) dengan tab filter dan aksi tandai dibaca, Quick-Action Notification Popover, AuthProvider & QueryProvider, production Login form dengan 1-click fill demo account cards, modal `ChangeStatusDialog` aset & aksi ubah/edit aset, dynamic runtime versioning (`/api/version`), shadcn/ui popover date picker kalender, custom thin scrollbar styling, active route highlighting, 16 baseline UI + 9 shared components, 33 app routes, 177 unit & component tests passing across 41 files di Vitest, automated Playwright E2E testing suite, audit kepatuhan aksesibilitas WCAG 2.2 AA. Runtime di-patch ke Node.js v22.23.4 melawan CVE-2026-111497 & CVE-2026-105481.
- **Production Releases & Patches**:
  - `v1.0.0` — Production release: full ITSM platform with Dokploy & Docker deployment.
  - `v1.0.1` - `v1.0.4` — Startup entrypoint view cache fixes, Dokploy networking, always-pull container images, sidebar active route highlight, brand typography, logout redirect clearing.
  - `v1.0.5` - `v1.0.6` — Single-column stack demo accounts 1-click fill cards, full notifications center (`/notifications`).
  - `v1.0.7` - `v1.0.9` — Topbar page title consistency, ticket delete confirmation flow fix, audit log ticket number & attachment filename interpolation fix.
  - `v1.0.10` - `v1.0.12` — Dynamic application versioning (`/api/version`), shadcn date picker for user form, custom thin scrollbar, Indonesian role labels, asset change status dialog & navigation, minimal demo cards.
  - `v1.0.13` — Security: PHP core vulnerabilities mitigation (CVE-2026-109618 & CVE-2026-109619, PHP 8.4.27).
  - `v1.0.14` — Security: Node.js security releases mitigation (CVE-2026-111497, CVE-2026-105481, Node.js v22.23.4).

### 2. Rust API Rewrite (`apps/axum-api`)
Pembangunan ulang backend API menggunakan **Rust (2024 edition)**, **Axum 0.8**, **SQLx 0.9**, dan **PostgreSQL 17** dengan arsitektur **Hybrid UUID v7** (UUID v7 untuk transaksi/entitas, SERIAL/INT untuk tabel master referensi) serta paritas kontrak 1:1 terhadap API Laravel:
- **Observability & Telemetry**: Structured logging berbasis `tracing` dan `tracing-subscriber` dengan JSON format & correlation/request ID, serta middleware `TraceLayer`.
- **Database Skema & Migrasi**: 10 file migrasi SQLx di `apps/axum-api/migrations/`, sequences khusus `ticket_number_seq` dan `employee_code_seq`, serta binary database seeder `src/bin/seed.rs` (`cargo run --bin seed`) dengan referensi pinned IDs sesuai spesifikasi D-15 (Roles 1-4, Statuses 1-5, Priorities 1-4, Departments 1-5, Categories 1-5, dan default user credentials).
- **Core Foundation**: Single envelope response pattern (`ApiResponse::success`, `ApiResponse::created`, `ApiResponse::paginated`), sistem error `AppError` dengan implementasi `IntoResponse` (memetakan kode HTTP 200, 201, 400, 401, 403, 404, 409, 422, 500 tanpa 204), extractor `ValidatedJson<T>` terintegrasi `validator`, serta domain models untuk seluruh 18 tabel.
- **Domain Enums & Serialization**: Seluruh enum sistem (`TicketStatusName`, `RoleName`, `UserStatus`, `AssetStatus`, `AssetHistoryAction`, `AuditAction`, `AuditModule`, `NotificationType`, `TicketAction`, `TicketActor`, `ArticleStatus`) dioptimasi dengan `strum::IntoStaticStr` zero-allocation dan penanganan TryFrom/From ID status tiket.
- **Otorisasi & State Machine**:
  - `resolve_ticket_actors`: Resolusi aktor tiket multi-peran (10 unit tests passing di `tests/tests.rs`).
  - `TicketTransitionMatrix`: Matriks transisi status tiket 1:1 terhadap `STATUS-TRANSITION.md`.
  - Middleware `EnsurePasswordChanged` untuk memproteksi pengguna dengan kewajiban ganti password (`must_change_password`).
  - Extractor `AuthUser` dan kueri SQL personal access token (Sanctum SHA-256 parity).
- **Server Router & Health**: Server Axum modular dengan endpoint `GET /health` yang memvalidasi kesiapan koneksi database pool.

### 3. Roadmap Status

- Phase 0 - 11 (Laravel Backend & Next.js Web) — **complete (Production Releases v1.0.0 through v1.0.14)**
- Rust API Rewrite (7 Fase Roadmap — `docs/RUST-AXUM-IMPLEMENTATION-GUIDE.md`):
  - **Fase 1: Setup Workspace & Observability** — **complete**
  - **Fase 2: Database Schema & Seeder PostgreSQL (Hybrid UUID v7)** — **complete**
  - **Fase 3: Foundation Core (Envelope, Error, Extractors, Domain Models & Enums)** — **complete**
  - **Fase 4: Autentikasi Sanctum Token & Profil** — **in progress** (queries token/user, `AuthUser` extractor, `EnsurePasswordChanged` middleware)
  - **Fase 5: Core Tiket, State Machine, & SLA Worker** — **in progress** (`TicketActorResolver` & `TicketTransitionMatrix` complete dengan 10 unit tests; pending transition endpoints & background SLA task)
  - **Fase 6: Modul Aset, Knowledge Base, Notifikasi & Audit Log** — **models & enums ready, endpoints pending**
  - **Fase 7: Dashboard Analytics, Streaming CSV, & Verifikasi Paritas Frontend** — **planned**
