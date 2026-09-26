<?php

namespace App\Services\Notification;

use App\Models\Notification;
use App\Models\User;
use Illuminate\Contracts\Pagination\LengthAwarePaginator;

class NotificationQueryService
{
    /**
     * Mengambil daftar notifikasi berdasarkan filter
     * dan yang sudah dipaginasi.
     */
    public function paginate(
        User $user,
        array $filters = [],
        int $perPage = 10,
        string $sortBy = "created_at",
        string $sortDirection = "desc",
    ): LengthAwarePaginator {
        $query = Notification::query()->where("user_id", $user->id);

        if (isset($filters["is_read"])) {
            $query->where(
                "is_read",
                filter_var($filters["is_read"], FILTER_VALIDATE_BOOLEAN),
            );
        }

        if ($type = $filters["type"]) {
            $query->where("type", $type);
        }

        return $query->orderBy($sortBy, $sortDirection)->paginate($perPage);
    }

    /**
     * Mengambil jumlah notifikasi yang belum dibaca.
     */
    public function getUnreadCount(User $user): int
    {
        return Notification::query()
            ->where("user_id", $user->id)
            ->where("is_read", false)
            ->count();
    }

    /**
     * Menandai notifikasi sebagai sudah dibaca.
     */
    public function markAsRead(Notification $notification): void
    {
        if (!$notification->is_read) {
            $notification->update([
                "is_read" => true,
                "read_at" => now(),
            ]);
        }
    }

    /**
     * Menandai semua notifikasi sebagai sudah dibaca.
     */
    public function markAllAsRead(User $user): int
    {
        return Notification::query()
            ->where("user_id", $user->id)
            ->where("is_read", false)
            ->update([
                "is_read" => true,
                "read_at" => now(),
            ]);
    }
}
