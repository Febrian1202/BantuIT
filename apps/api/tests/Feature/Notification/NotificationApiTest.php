<?php

use App\Models\Notification;
use App\Models\Ticket;
use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\Artisan;
use Illuminate\Support\Facades\DB;
use Laravel\Sanctum\Sanctum;

uses(RefreshDatabase::class);

test(
    'request yang tidak terautentikasi ke endpoint notifikasi mengembalikan 401',
    function () {
        $this->getJson('/api/notifications')->assertStatus(401);
        $this->getJson('/api/notifications/unread-count')->assertStatus(401);
        $this->postJson('/api/notifications/1/read')->assertStatus(401);
        $this->postJson('/api/notifications/read-all')->assertStatus(401);
    },
);

test(
    'user melihat notifikasi hanya miliknya dalam envelope paginasi',
    function () {
        $user = User::factory()->employee()->create();
        $other = User::factory()->employee()->create();

        Notification::factory()
            ->count(3)
            ->create(['user_id' => $user->id]);
        Notification::factory()
            ->count(5)
            ->create(['user_id' => $other->id]);

        Sanctum::actingAs($user);

        $response = $this->getJson('/api/notifications');

        $response
            ->assertStatus(200)
            ->assertJsonPath('success', true)
            ->assertJsonPath('meta.total', 3)
            ->assertJsonCount(3, 'data');
    },
);

test(
    'user tidak dapat menandai notifikasi milik orang lain sebagai sudah dibaca',
    function () {
        $user = User::factory()->employee()->create();
        $other = User::factory()->employee()->create();

        $notif = Notification::factory()->create([
            'user_id' => $other->id,
            'is_read' => false,
        ]);

        Sanctum::actingAs($user);

        $this->postJson("/api/notifications/{$notif->id}/read")->assertStatus(
            404,
        );

        expect($notif->refresh()->is_read)->toBeFalse();
    },
);

test(
    'admin tidak dapat menandai notifikasi milik orang lain sebagai sudah dibaca',
    function () {
        $admin = User::factory()->admin()->create();
        $user = User::factory()->employee()->create();

        $notif = Notification::factory()->create([
            'user_id' => $user->id,
            'is_read' => false,
        ]);

        Sanctum::actingAs($admin);

        $this->postJson("/api/notifications/{$notif->id}/read")->assertStatus(
            404,
        );
    },
);

test(
    'unread-count menjalankan query untuk menghitung notifikasi yang belum dibaca',
    function () {
        $user = User::factory()->employee()->create();
        Notification::factory()
            ->count(3)
            ->create(['user_id' => $user->id, 'is_read' => false]);

        Sanctum::actingAs($user);

        DB::enableQueryLog();
        $response = $this->getJson('/api/notifications/unread-count');
        $queries = DB::getQueryLog();

        $response->assertStatus(200)->assertJsonPath('data.unread_count', 3);

        // Verify query count: user auth resolution + single count query on notifications
        expect(count($queries))->toBeLessThanOrEqual(2);
    },
);

test('read-all hanya menandai notifikasi user saat ini', function () {
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

    Sanctum::actingAs($user);

    $this->postJson('/api/notifications/read-all')
        ->assertStatus(200)
        ->assertJsonPath('data', null)
        ->assertJsonPath('message', 'All notifications marked as read.');

    expect($n1->refresh()->is_read)
        ->toBeTrue()
        ->and($n2->refresh()->is_read)
        ->toBeTrue()
        ->and($n3->refresh()->is_read)
        ->toBeFalse();
});

test(
    'SLA breach yang di generate oleh scheduler bisa dibaca melalui API notification oleh technician dan manager',
    function () {
        $manager = User::factory()->manager()->create();
        $technician = User::factory()->technician()->create();

        $ticket = Ticket::factory()->create([
            'status_id' => 2,
            'technician_id' => $technician->id,
            'sla_deadline' => now()->subMinutes(10),
            'sla_breached' => false,
        ]);

        // Jalankan scheduler
        Artisan::call('tickets:check-sla');

        // 1. Cek dari sisi Teknisi
        Sanctum::actingAs($technician);
        $resTech = $this->getJson(
            '/api/notifications?type=TICKET_SLA_BREACHED',
        );
        $resTech
            ->assertStatus(200)
            ->assertJsonPath('meta.total', 1)
            ->assertJsonPath('data.0.data.actor_name', 'Sistem');

        // 2. Cek dari sisi Manager
        Sanctum::actingAs($manager);
        $resMgr = $this->getJson('/api/notifications/unread-count');
        $resMgr->assertStatus(200)->assertJsonPath('data.unread_count', 1);
    },
);
