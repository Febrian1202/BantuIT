<?php

namespace App\Services\Ticket;

use App\DTOs\Admin\CreateTicketCategoryData;
use App\DTOs\Admin\UpdateTicketCategoryData;
use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Models\Ticket;
use App\Models\TicketCategory;
use App\Models\User;
use App\Services\Admin\ReferentialIntegrityGuard;
use App\Services\Audit\AuditLogger;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Support\Facades\DB;

class TicketCategoryServices
{
    public function __construct(
        private AuditLogger $auditLogger,
        private ReferentialIntegrityGuard $guard,
    ) {}

    /**
     * Mendapatkan daftar kategori tiket.
     */
    public function index(): Collection
    {
        return TicketCategory::with('parent')->orderBy('name')->get();
    }

    /**
     * Menampilkan detail kategori tiket.
     */
    public function show(TicketCategory $ticketCategory): TicketCategory
    {
        return $ticketCategory->load('parent');
    }

    /**
     * Membuat kategori tiket baru.
     */
    public function store(
        CreateTicketCategoryData $data,
        User $actor,
    ): TicketCategory {
        return DB::transaction(function () use ($data, $actor): TicketCategory {
            $ticketCategory = TicketCategory::create([
                'name' => $data->name,
                'parent_id' => $data->parentId,
                'description' => $data->description,
            ]);

            $this->auditLogger->log(
                $actor,
                AuditAction::Create,
                AuditModule::TicketCategory,
                $ticketCategory->id,
                "Kategori tiket {$ticketCategory->name} dibuat.",
                null,
                $ticketCategory->only(['name', 'description', 'parent_id']),
            );

            return $ticketCategory;
        });
    }

    /**
     * Memperbarui kategori tiket yang sudah ada.
     */
    public function update(
        UpdateTicketCategoryData $data,
        TicketCategory $ticketCategory,
        User $actor,
    ): TicketCategory {
        $oldData = $ticketCategory->only(['name', 'description', 'parent_id']);

        return DB::transaction(function () use (
            $data,
            $ticketCategory,
            $actor,
            $oldData,
        ): TicketCategory {
            $ticketCategory->update([
                'name' => $data->name,
                'parent_id' => $data->parentId,
                'description' => $data->description,
            ]);

            $this->auditLogger->log(
                $actor,
                AuditAction::Update,
                AuditModule::TicketCategory,
                $ticketCategory->id,
                "Kategori tiket {$ticketCategory->name} diperbarui.",
                $oldData,
                $ticketCategory->only(['name', 'description', 'parent_id']),
            );

            return $ticketCategory->fresh()->load('parent');
        });
    }

    /**
     * Menghapus kategori tiket.
     */
    public function destroy(TicketCategory $ticketCategory, User $actor): void
    {
        $this->guard->assertUnreferenced([
            'Kategori tiket' => Ticket::where(
                'category_id',
                $ticketCategory->id,
            ),
            'Kategori tiket (induk)' => TicketCategory::where(
                'parent_id',
                $ticketCategory->id,
            ),
        ]);

        DB::transaction(function () use ($ticketCategory, $actor): void {
            $categoryName = $ticketCategory->name;
            $categoryId = $ticketCategory->id;

            $ticketCategory->delete();

            $this->auditLogger->log(
                $actor,
                AuditAction::Delete,
                AuditModule::TicketCategory,
                $categoryId,
                "Kategori tiket {$categoryName} dihapus.",
            );
        });
    }
}
