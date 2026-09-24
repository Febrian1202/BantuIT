<?php

namespace App\Policies;

use App\Models\Notification;
use App\Models\User;
use Illuminate\Auth\Access\Response;

/**
 * Akses notifikasi terlingkupi secara ketat hanya pada user pemiliknya saja.
 * Admin tidak punya pengecualian menyeluruh di sini.
 */
class NotificationPolicy
{
    /**
     * Melihat notification tunggal tidak diekspos lewat API; method ini dibuat
     * hanya untuk menguji skenario otorisasi 404 (not found) di unit test.
     */
    public function view(User $user, Notification $notification): Response
    {
        return Response::denyAsNotFound();
    }

    /**
     * Melihat daftar notification diizinkan untuk semua user yang terautentikasi;
     * scoping diterapkan pada saat query berjalan.
     */
    public function viewAny(User $user): bool
    {
        return true;
    }

    /**
     * User hanya bisa mark as read notification jika notifikasi tersebut miliknya sendiri.
     */
    public function markAsRead(User $user, Notification $notification): bool
    {
        return $notification->user_id === $user->id;
    }

    /**
     * Mark-all di-scope khusus untuk user yang sedang terautentikasi pada saat query.
     */
    public function markAllAsRead(User $user): bool
    {
        return true;
    }
}
