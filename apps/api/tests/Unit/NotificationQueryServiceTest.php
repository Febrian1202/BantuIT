<?php

use App\Models\Notification;
use App\Models\User;
use App\Services\Notification\NotificationQueryService;
use Illuminate\Foundation\Testing\RefreshDatabase;

uses(RefreshDatabase::class);

test(
    'paginasi dibatasi secata ketat kepada user dan filter secara tepat',
    function () {
        $user = User::factory()->employee()->create();
        $other = User::factory()->employee()->create();

        Notification::factory()
            ->count(3)
            ->create(['user_id' => $user->id, 'is_read' => false]);
        Notification::factory()
            ->count(2)
            ->create(['user_id' => $user->id, 'is_read' => true]);
        Notification::factory()
            ->count(4)
            ->create(['user_id' => $other->id]);

        $service = new NotificationQueryService;
        $result = $service->paginate(
            $user,
            ['is_read' => false],
            perPage: 10,
            sortBy: 'created_at',
            sortDir: 'desc',
        );

        expect($result->total())->toBe(3);
    },
);

test(
    'getUnreadCount mengembalikan jumlah notifikasi belum dibaca yang tepat',
    function () {
        $user = User::factory()->employee()->create();
        Notification::factory()
            ->count(4)
            ->create(['user_id' => $user->id, 'is_read' => false]);
        Notification::factory()
            ->count(2)
            ->create(['user_id' => $user->id, 'is_read' => true]);

        $service = new NotificationQueryService;
        expect($service->getUnreadCount($user))->toBe(4);
    },
);

test('markAsRead memperbarui is_read dan read_at timestamp', function () {
    $user = User::factory()->employee()->create();
    $notif = Notification::factory()->create([
        'user_id' => $user->id,
        'is_read' => false,
        'read_at' => null,
    ]);

    $service = new NotificationQueryService;
    $service->markAsRead($notif);

    $notif->refresh();
    expect($notif->is_read)->toBeTrue()->and($notif->read_at)->not->toBeNull();
});

test(
    'markAllAsRead menandai semua notifikasi user sebagai sudah dibaca',
    function () {
        $user = User::factory()->employee()->create();
        $other = User::factory()->employee()->create();

        $n1 = Notification::factory()->create([
            'user_id' => $user->id,
            'is_read' => false,
        ]);
        $n2 = Notification::factory()->create([
            'user_id' => $user->id,
            'is_read' => false,
        ]);
        $n3 = Notification::factory()->create([
            'user_id' => $other->id,
            'is_read' => false,
        ]);

        $service = new NotificationQueryService;
        $count = $service->markAllAsRead($user);

        expect($count)
            ->toBe(2)
            ->and($n1->refresh()->is_read)
            ->toBeTrue()
            ->and($n2->refresh()->is_read)
            ->toBeTrue()
            ->and($n3->refresh()->is_read)
            ->toBeFalse();
    },
);
