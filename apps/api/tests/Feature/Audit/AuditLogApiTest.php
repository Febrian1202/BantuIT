<?php

use App\Models\AuditLog;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

test(
    'employee dan technician tidak bisa akses audit log endpoints (403)',
    function () {
        $employee = User::factory()->employee()->create();
        $technician = User::factory()->technician()->create();
        $log = AuditLog::factory()->create();

        Sanctum::actingAs($employee);
        $this->getJson('/api/audit-logs')->assertStatus(403);
        $this->getJson("/api/audit-logs/{$log->id}")->assertStatus(403);

        Sanctum::actingAs($technician);
        $this->getJson('/api/audit-logs')->assertStatus(403);
        $this->getJson("/api/audit-logs/{$log->id}")->assertStatus(403);
    },
);

test(
    'manager hanya melihat modul operasional  (ticket, asset, article)',
    function () {
        $manager = User::factory()->manager()->create();

        AuditLog::factory()->create(['module' => 'ticket']);
        AuditLog::factory()->create(['module' => 'asset']);
        AuditLog::factory()->create(['module' => 'article']);
        AuditLog::factory()->create(['module' => 'user']);
        AuditLog::factory()->create(['module' => 'auth']);

        Sanctum::actingAs($manager);
        $response = $this->getJson('/api/audit-logs');

        $response->assertStatus(200)->assertJsonPath('meta.total', 3);
    },
);

test(
    'manager filtering modul terlarang menerima 200 dengan list kosong',
    function () {
        $manager = User::factory()->manager()->create();
        AuditLog::factory()->create(['module' => 'user']);

        Sanctum::actingAs($manager);
        $response = $this->getJson('/api/audit-logs?module=user');

        $response
            ->assertStatus(200)
            ->assertJsonPath('meta.total', 0)
            ->assertJsonCount(0, 'data');
    },
);

test(
    'manager mengakses detail dari module terlarang menerima 404',
    function () {
        $manager = User::factory()->manager()->create();
        $userLog = AuditLog::factory()->create(['module' => 'user']);

        Sanctum::actingAs($manager);
        $this->getJson("/api/audit-logs/{$userLog->id}")->assertStatus(404);
    },
);

test(
    'admin punya full akses untuk semua audit logs dan detail payload termasuk old/new data',
    function () {
        $admin = User::factory()->admin()->create();
        $userLog = AuditLog::factory()->create([
            'module' => 'user',
            'old_data' => ['status' => 'inactive'],
            'new_data' => ['status' => 'active'],
        ]);

        Sanctum::actingAs($admin);
        $response = $this->getJson("/api/audit-logs/{$userLog->id}");

        $response
            ->assertStatus(200)
            ->assertJsonPath('data.module', 'user')
            ->assertJsonPath('data.old_data.status', 'inactive')
            ->assertJsonPath('data.new_data.status', 'active');
    },
);

test(
    'date range filter secara benar memfilter logs dalam rentang Asia/Jakarta',
    function () {
        $admin = User::factory()->admin()->create();

        // Created at 2026-09-01 10:00:00 WIB (03:00:00 UTC)
        $log1 = AuditLog::factory()->create([
            'created_at' => '2026-09-01T03:00:00Z',
        ]);

        // Created at 2026-09-02 10:00:00 WIB (03:00:00 UTC)
        $log2 = AuditLog::factory()->create([
            'created_at' => '2026-09-02T03:00:00Z',
        ]);

        Sanctum::actingAs($admin);
        $response = $this->getJson(
            '/api/audit-logs?date_from=2026-09-01&date_to=2026-09-01',
        );

        $response
            ->assertStatus(200)
            ->assertJsonPath('meta.total', 1)
            ->assertJsonPath('data.0.id', $log1->id);
    },
);
