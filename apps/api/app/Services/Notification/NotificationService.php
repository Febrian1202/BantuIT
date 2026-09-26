<?php

namespace App\Services\Notification;

use App\Enums\NotificationType;
use App\Models\Notification;
use App\Models\User;
use Illuminate\Support\Collection;

class NotificationService
{
    /**
     * Mengirim notifikasi ke pengguna yang ditentukan.
     */
    public function notify(
        User $recipient,
        NotificationType $type,
        array $data,
    ): Notification {
        return Notification::create([
            'user_id' => $recipient->id,
            'type' => $type->value,
            'data' => $data,
            'is_read' => false,
            'read_at' => null,
        ]);
    }

    /**
     * Mengirim notifikasi ke banyak pengguna yang ditentukan.
     */
    public function notifyMany(
        Collection $recipients,
        NotificationType $type,
        array $data,
        ?User $actor = null,
    ): void {
        $actorId = $actor?->id;

        $uniqueRecipients = $recipients
            ->filter(
                fn ($u) => $u instanceof User &&
                    ($actorId === null || $u->id !== $actorId),
            )
            ->unique('id');

        foreach ($uniqueRecipients as $recipient) {
            $this->notify($recipient, $type, $data);
        }
    }
}
