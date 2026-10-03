<?php

namespace App\Policies;

use App\Enums\RoleName;
use App\Models\TicketAttachment;
use App\Models\User;
use Illuminate\Auth\Access\Response;

class TicketAttachmentPolicy
{
    public function view(
        User $user,
        TicketAttachment $ticketAttachment,
    ): Response|bool {
        return $user->can('view', $ticketAttachment->ticket)
            ? true
            : Response::denyAsNotFound();
    }

    public function download(
        User $user,
        TicketAttachment $ticketAttachment,
    ): Response|bool {
        return $this->view($user, $ticketAttachment);
    }

    public function create(User $user, TicketAttachment $ticketAttachment): bool
    {
        return true;
    }

    public function delete(User $user, TicketAttachment $ticketAttachment): bool
    {
        if ($user->isAdmin() || $user->hasRole(RoleName::Manager)) {
            return true;
        }

        return $ticketAttachment->uploaded_by === $user->id;
    }
}
