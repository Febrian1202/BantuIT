<?php

namespace App\Services;

use App\Enums\RoleName;
use App\Enums\UserStatus;
use App\Models\Role;
use App\Models\TicketCategory;
use App\Models\TicketPriority;
use App\Models\TicketStatus;
use App\Models\User;
use Illuminate\Database\Eloquent\Collection;

class ReferenceService
{
    /**
     * Mengambil semua kategori tiket.
     */
    public function ticketCategories(): Collection
    {
        return TicketCategory::all(['id', 'name']);
    }

    /**
     * Mengambil semua prioritas tiket.
     */
    public function ticketPriorities(): Collection
    {
        return TicketPriority::all('id', 'name', 'sla_minutes');
    }

    /**
     * Mengambil semua status tiket.
     */
    public function ticketStatuses(): Collection
    {
        return TicketStatus::all('id', 'name');
    }

    /**
     * Mengambil daftar teknisi yang aktif.
     */
    public function technicianList(): Collection
    {
        $technicianRoleId = Role::where(
            'name',
            RoleName::Technician->value,
        )->value('id');

        return User::where('role_id', $technicianRoleId)
            ->where('status', UserStatus::Active->value)
            ->get(['id', 'full_name']);
    }
}
