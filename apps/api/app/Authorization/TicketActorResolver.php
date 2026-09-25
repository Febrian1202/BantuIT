<?php

namespace App\Authorization;

use App\Enums\RoleName;
use App\Enums\TicketActor;
use App\Models\Ticket;
use App\Models\User;

/**
 * Menentukan set nilai TicketActor yang dimiliki pengguna untuk tiket tertentu.
 *
 * Pengguna dapat memiliki beberapa peran pelaku (actor) sekaligus (misalnya, teknisi
 * yang ditugaskan juga memiliki peran AnyTechnician). Peran Reporter hanya diberikan
 * kepada karyawan biasa (employee) agar admin/manager/teknisi tidak secara tidak
 * sengaja mendapatkan hak akses tambahan hanya karena mereka kebetulan menjadi pelapor tiket.
 */
class TicketActorResolver
{
    public function resolve(Ticket $ticket, User $user): array
    {
        $roles = [];

        if ($user->isAdmin()) {
            $roles[] = TicketActor::Admin;
        }

        if ($user->hasRole(RoleName::Manager)) {
            $roles[] = TicketActor::Manager;
        }

        if (
            $user->hasRole(RoleName::Technician) &&
            $ticket->technician_id === $user->id
        ) {
            $roles[] = TicketActor::Technician;
        }

        if ($user->hasRole(RoleName::Technician)) {
            $roles[] = TicketActor::AnyTechnician;
        }

        if (
            $ticket->reporter_id === $user->id &&
            ! $user->isAdmin() &&
            ! $user->hasRole(RoleName::Manager, RoleName::Technician)
        ) {
            $roles[] = TicketActor::Reporter;
        }

        return $roles;
    }
}
