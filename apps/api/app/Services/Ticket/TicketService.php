<?php

namespace App\Services\Ticket;

use App\DTOs\Ticket\CreateTicketData;
use App\DTOs\Ticket\UpdateTicketData;
use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Enums\RoleName;
use App\Enums\TicketStatusName;
use App\Http\Requests\Ticket\IndexTicketRequest;
use App\Models\Ticket;
use App\Models\TicketCategory;
use App\Models\TicketHistory;
use App\Models\TicketPriority;
use App\Models\TicketStatus;
use App\Models\User;
use App\Services\Audit\AuditLogger;
use App\Services\Sla\SlaService;
use App\Support\HandlesPagination;
use Illuminate\Pagination\LengthAwarePaginator;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Str;
use Symfony\Component\HttpKernel\Exception\AccessDeniedHttpException;

class TicketService
{
    use HandlesPagination;

    public function __construct(
        private readonly SlaService $slaService,
        private readonly AuditLogger $auditLogger,
    ) {}

    /**
     * Paginate tiket berdasarkan permintaan dan pengguna.
     */
    public function paginate(
        IndexTicketRequest $request,
        User $actor,
    ): LengthAwarePaginator {
        $query = Ticket::with([
            "status",
            "priority",
            "category",
            "reporter",
            "technician",
        ]);

        $isPrevileged =
            $actor->isAdmin() ||
            $actor->hasRole(RoleName::Manager, RoleName::Technician);

        if (!$isPrevileged) {
            $query->where("reporter_id", $actor->id);
        }

        $validated = $request->validated();

        if (!empty($validated["search"])) {
            $search = str_replace(
                ["%", "_"],
                ["\\%", "\\_"],
                $validated["search"],
            );
            $query->where(function ($q) use ($search) {
                $q->whereRaw("ticket_number LIKE ? ESCAPE ?", [
                    "%{$search}%",
                    "\\",
                ])->orWhereRaw("title LIKE ? ESCAPE ?", ["%{$search}%", "\\"]);
            });
        }

        foreach (["status_id", "priority_id", "category_id"] as $field) {
            if (!empty($validated[$field])) {
                $ids = array_map("intval", explode(",", $validated[$field]));
                $ids = array_values(array_filter($ids, fn($v) => $v > 0));
                $query->whereIn($field, $ids);
            }
        }

        if (!empty($validated["technician_id"])) {
            if ($validated["technician_id"] == "unassigned") {
                $query->whereNull("technician_id");
            } else {
                $query->whereIn("technician_id", [
                    (int) $validated["technician_id"],
                ]);
            }
        }

        if ($isPrevileged && !empty($validated["reporter_id"])) {
            $query->where("reporter_id", (int) $validated["reporter_id"]);
        }

        foreach (["department_id", "asset_id"] as $field) {
            if (!empty($validated[$field])) {
                $query->where($field, (int) $validated[$field]);
            }
        }

        if (!empty($validated["sla_status"])) {
            if ($validated["sla_status"] === "breached") {
                $this->slaService->scopeBreached($query);
            } else {
                $this->slaService->scopeOnTrack($query);
            }
        }

        if (!empty($validated["created_from"])) {
            $query->where(
                "created_at",
                ">=",
                $validated["created_from"] . " 00:00:00",
            );
        }

        if (!empty($validated["created_to"])) {
            $query->where(
                "created_at",
                "<=",
                $validated["created_to"] . " 23:59:59",
            );
        }

        $sortBy = $validated["sort_by"] ?? "created_at";
        $sortDir = $validated["sort_dir"] ?? "desc";

        return $query
            ->orderBy($sortBy, $sortDir)
            ->paginate($this->getPerPage($request));
    }

    /**
     * Membuat tiket baru berdasarkan data yang diberikan dan pengguna yang membuatnya.
     */
    public function create(CreateTicketData $data, User $actor): Ticket
    {
        return DB::transaction(function () use ($data, $actor) {
            $priority = TicketPriority::findOrFail($data->priorityId);
            $status = TicketStatus::find(TicketStatusName::Open->id());

            $ticket = Ticket::create([
                "ticket_number" => "TMP-" . Str::ulid(), // Placeholder
                "title" => $data->title,
                "description" => $data->description,
                "category_id" => $data->categoryId,
                "priority_id" => $data->priorityId,
                "status_id" => $status->id,
                "reporter_id" => $actor->id,
                "department_id" => $actor->department_id,
                "asset_id" => $data->assetid,
                "sla_duration_minutes" => (int) $priority->sla_minutes,
            ]);

            $ticket
                ->forceFill([
                    "ticket_number" => sprintf("TCK-%04d", $ticket->id),
                ])
                ->save();

            $this->slaService->snapshot($ticket, $priority);
            $ticket->save();

            TicketHistory::create([
                "ticket_id" => $ticket->id,
                "user_id" => $actor->id,
                "field_changed" => "status_id",
                "old_value" => null,
                "new_value" => TicketStatusName::Open->label(),
            ]);

            $this->auditLogger->log(
                $actor,
                AuditAction::Create,
                AuditModule::Ticket,
                $ticket->id,
                "Ticket #{$ticket->ticket_numner} dibuat.",
            );

            return $ticket->load([
                "status",
                "priority",
                "category",
                "reporter",
                "technician",
                "department",
                "asset",
            ]);
        });
    }

    /**
     * Mencari tiket berdasarkan ID dan memuat relasi yang terkait.
     */
    public function find(int $id): Ticket
    {
        return Ticket::query()
            ->with([
                "status",
                "priority",
                "category",
                "reporter.department",
                "technician",
                "department",
                "asset" => fn($q) => $q->withTrashed(),
                "comments",
                "attachments",
            ])
            ->findOrFail($id);
    }

    /**
     * Memperbarui tiket yang sudah ada berdasarkan data yang diberikan
     * dan pengguna yang memperbarui.
     */
    public function update(
        Ticket $ticket,
        UpdateTicketData $data,
        User $actor,
    ): Ticket {
        if ((bool) ($ticket->status?->is_final ?? false)) {
            throw new AccessDeniedHttpException(
                "This ticket is closed and cannot be edited.",
            );
        }

        return DB::transaction(function () use (
            $ticket,
            $data,
            $actor,
        ): Ticket {
            $old = $ticket->only($data->fields);

            foreach ($data->fields as $field) {
                if ($field === "category_id") {
                    $ticket->category_id = $data->categoryId;
                } elseif ($field === "title") {
                    $ticket->title = $data->title;
                } elseif ($field === "description") {
                    $ticket->description = $data->description;
                }
            }
            $ticket->save();

            foreach ($data->fields as $field) {
                $newValue = $ticket->getAttribute($field);
                if (
                    (string) ($old[$field] ?? "") !== (string) ($newValue ?? "")
                ) {
                    TicketHistory::create([
                        "ticket_id" => $ticket->id,
                        "user_id" => $actor->id,
                        "field_changed" => $field,
                        "old_value" => $this->displayValue(
                            $field,
                            $old[$field] ?? null,
                        ),
                        "new_value" => $this->displayValue(
                            $field,
                            $newValue ?? null,
                        ),
                    ]);
                }
            }

            $this->auditLogger->log(
                $actor,
                AuditAction::Update,
                AuditModule::Ticket,
                $ticket->id,
                "Ticket #{$ticket->ticket_number} diperbarui.",
                $old,
                $ticket->only($data->fields),
            );

            return $ticket
                ->fresh()
                ->load([
                    "status",
                    "priority",
                    "category",
                    "reporter",
                    "technician",
                    "department",
                    "asset",
                ]);
        });
    }

    /**
     * Menghapus tiket yang sudah ada dan mencatat log penghapusan.
     */
    public function delete(Ticket $ticket, User $actor): void
    {
        DB::transaction(function () use ($ticket, $actor): void {
            $ticket->delete();

            $this->auditLogger->log(
                $actor,
                AuditAction::Delete,
                AuditModule::Ticket,
                $ticket->id,
                "Ticket #{$ticket->ticket_number} dihapus.",
            );
        });
    }

    /**
     * Mengonversi nilai field menjadi string yang dapat ditampilkan.
     */
    private function displayValue(string $field, mixed $value): ?string
    {
        return match ($field) {
            "category_id" => TicketCategory::find($value)?->name,
            default => $value,
        };
    }
}
