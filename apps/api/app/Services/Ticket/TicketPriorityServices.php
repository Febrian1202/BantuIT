<?php

namespace App\Services\Ticket;

use App\DTOs\Admin\CreateTicketPriorityData;
use App\DTOs\Admin\UpdateTicketPriorityData;
use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Models\Ticket;
use App\Models\TicketPriority;
use App\Models\User;
use App\Services\Admin\ReferentialIntegrityGuard;
use App\Services\Audit\AuditLogger;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Support\Facades\DB;

class TicketPriorityServices
{
    public function __construct(
        private AuditLogger $auditLogger,
        private ReferentialIntegrityGuard $guard,
    ) {}

    /**
     * Mengambil daftar prioritas tiket.
     */
    public function index(): Collection
    {
        return TicketPriority::orderBy("sla_minutes")->get();
    }

    /**
     * Menyimpan prioritas tiket baru.
     */
    public function store(
        CreateTicketPriorityData $data,
        User $actor,
    ): TicketPriority {
        return DB::transaction(function () use ($data, $actor): TicketPriority {
            $priority = TicketPriority::create($data->toArray());
            $this->auditLogger->log(
                $actor,
                AuditAction::Create,
                AuditModule::TicketPriority,
                $priority->id,
                "Prioritas tiket {$priority->name} dibuat.",
                null,
                $priority->only([
                    "name",
                    "level",
                    "sla_minutes",
                    "description",
                ]),
            );

            return $priority;
        });
    }

    /**
     * Mengupdate prioritas tiket.
     */
    public function update(
        UpdateTicketPriorityData $data,
        TicketPriority $priority,
        User $actor,
    ): TicketPriority {
        $old = $priority->only(["name", "level", "sla_minutes", "description"]);

        return DB::transaction(function () use (
            $data,
            $priority,
            $actor,
            $old,
        ): TicketPriority {
            $priority->update($data->toArray());

            $this->auditLogger->log(
                $actor,
                AuditAction::Update,
                AuditModule::TicketPriority,
                $priority->id,
                "Prioritas tiket {$priority->name} diperbarui.",
                $old,
                $priority->only([
                    "name",
                    "level",
                    "sla_minutes",
                    "description",
                ]),
            );

            return $priority->fresh();
        });
    }

    /**
     * Hapus prioritas tiket.
     */
    public function destroy(TicketPriority $priority, User $actor): void
    {
        $this->guard->assertUnreferenced([
            "Prioritas tiket" => Ticket::where("priority_id", $priority->id),
        ]);

        DB::transaction(function () use ($priority, $actor): void {
            $priorityName = $priority->name;
            $priorityId = $priority->id;

            $priority->delete();

            $this->auditLogger->log(
                $actor,
                AuditAction::Delete,
                AuditModule::TicketPriority,
                $priorityId,
                "Prioritas tiket {$priorityName} dihapus.",
            );
        });
    }
}
