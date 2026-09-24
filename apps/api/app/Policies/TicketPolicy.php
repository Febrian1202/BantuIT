<?php

namespace App\Policies;

use App\Enums\RoleName;
use App\Models\Ticket;
use App\Models\User;
use Illuminate\Auth\Access\Response;
use Monolog\Handler\RotatingFileHandler;

class TicketPolicy
{
    /**
     * Menentukan apakah pengguna dapat melihat daftar tiket.
     */
    public function viewAny(User $user): bool
    {
        return true;
    }

    /**
     * Menentukan apakah pengguna dapat melihat tiket tertentu.
     */
    public function view(User $user, Ticket $ticket): Response|bool
    {
        if (
            $user->isAdmin() ||
            $user->hasRole(RoleName::Manager, RoleName::Technician)
        ) {
            return true;
        }

        return $ticket->reporter_id === $user->id
            ? true
            : Response::denyAsNotFound();
    }

    /**
     * Menentukan apakah pengguna dapat membuat tiket baru.
     */
    public function create(User $user): bool
    {
        return true;
    }

    /**
     * Menentukan apakah pengguna dapat memperbarui tiket tertentu.
     */
    public function update(User $user, Ticket $ticket): Response|bool
    {
        if ((bool) ($ticket->status?->is_final ?? false)) {
            return false;
        }

        if (
            $user->isAdmin() ||
            $user->hasRole(RoleName::Manager, RoleName::Technician)
        ) {
            return true;
        }

        return $ticket->reporter_id === $user->id
            ? true
            : Response::denyAsNotFound();
    }

    /**
     * Menentukan apakah pengguna dapat menghapus tiket tertentu.
     */
    public function delete(User $user, Ticket $ticket): bool
    {
        return $user->isAdmin();
    }

    /**
     * Menentukan apakah pengguna dapat menugaskan tiket tertentu.
     */
    public function assign(User $user, Ticket $ticket): bool
    {
        return $user->isAdmin() || $user->hasRole(RoleName::Manager);
    }

    /**
     * Menentukan apakah pengguna dapat membatalkan tugas pada tiket tertentu.
     */
    public function unassign(User $user, Ticket $ticket): bool
    {
        return $user->isAdmin() || $user->hasRole(RoleName::Manager);
    }

    /**
     * Menentukan apakah pengguna dapat mengubah status tiket tertentu.
     */
    public function changeStatus(User $user, Ticket $ticket): bool
    {
        if ($user->isAdmin() || $user->hasRole(RoleName::Manager)) {
            return true;
        }

        if ($user->hasRole(RoleName::Technician)) {
            if ($ticket->technician_id === $user->id) {
                return true;
            }

            if (
                (int) $ticket->status_id === 1 &&
                $ticket->technician_id === null
            ) {
                return true;
            }

            return false;
        }

        return $ticket->reporter_id === $user->id
            ? true
            : Response::denyAsNotFound();
    }

    /**
     * Menentukan apakah pengguna dapat menugaskan dirinya sebagai teknisi pada tiket tertentu.
     */
    public function selfAssign(User $user, Ticket $ticket): bool
    {
        return $user->hasRole(RoleName::Technician) &&
            (int) $ticket->status_id === 1 &&
            $ticket->technician_id === null;
    }

    /**
     * Menentukan apakah pengguna dapat mengubah prioritas tiket tertentu.
     */
    public function changePriority(User $user, Ticket $ticket): bool
    {
        if ($user->isAdmin() || $user->hasRole(RoleName::Manager)) {
            return true;
        }

        if ($user->hasRole(RoleName::Technician)) {
            return $ticket->technician_id === $user->id;
        }

        return false;
    }

    /**
     * Menentukan apakah pengguna dapat memberikan komentar pada tiket tertentu.
     */
    public function comment(User $user, Ticket $ticket): Response|bool
    {
        if (
            (int) $ticket->status_id === 5 ||
            (bool) ($ticket->status?->is_final ?? false)
        ) {
            return false;
        }

        if ($user->isAdmin() || $user->hasRole(RoleName::Manager)) {
            return true;
        }

        if ($user->hasRole(RoleName::Technician)) {
            return $ticket->technician_id === $user->id
                ? true
                : Response::denyAsNotFound();
        }

        return $ticket->reporter_id === $user->id
            ? true
            : Response::denyAsNotFound();
    }

    /**
     * Menentukan apakah pengguna dapat melihat riwayat tiket tertentu.
     */
    public function viewHistory(User $user, Ticket $ticket): Response|bool
    {
        return $this->view($user, $ticket);
    }

    /**
     * Menentukan apakah pengguna dapat menambahkan lampiran pada tiket tertentu.
     */
    public function attach(User $user, Ticket $ticket): Response|bool
    {
        return $this->isParticipant($user, $ticket);
    }

    /**
     * Menentukan apakah pengguna dapat menjadi bagian dari tiket tertentu.
     */
    public function isParticipant(User $user, Ticket $ticket): Response|bool
    {
        if ($user->isAdmin() || $user->hasRole(RoleName::Manager)) {
            return true;
        }

        if ($user->hasRole(RoleName::Technician)) {
            return $ticket->technician_id === $user->id
                ? true
                : Response::denyAsNotFound();
        }

        return $ticket->reporter_id === $user->id
            ? true
            : Response::denyAsNotFound();
    }
}
