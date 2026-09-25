<?php

namespace App\Services\Notification;

use App\Enums\NotificationType;
use App\Models\Notification;
use App\Models\User;
use Illuminate\Support\Collection;

class NotificationService
{
    public function notify(
        User $recipient,
        NotificationType $type,
        array $data,
    ): Notification {
        // TODO: Implementasi nanti
        return new Notification();
    }

    public function notifyMany(
        Collection $recipients,
        NotificationType $type,
        array $data,
        ?User $actor = null,
    ): Collection {
        // TODO: Implementasi nanti
        return new Collection();
    }
}
