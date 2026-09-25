<?php

use App\Enums\AssetStatus;
use App\Models\Asset;
use App\Models\AssetAssignment;
use App\Models\AuditLog;
use App\Models\Ticket;
use App\Models\TicketCategory;
use App\Models\TicketPriority;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

uses()->group('ticket');

test(
    'proses create mengatur status ke OPEN dan menyalin/membekukan (snapshot) konfigurasi SLA.',
    function () {
        $employee = User::factory()->employee()->create();
        Sanctum::actingAs($employee);
        $priority = TicketPriority::find(2); // High, 240 menit

        $this->postJson('/api/tickets', [
            'title' => 'Laptop mati',
            'description' => 'Tidak bisa boot',
            'category_id' => TicketCategory::where('name', 'Laptop')->first()
                ->id,
            'priority_id' => $priority->id,
        ])
            ->assertStatus(201)
            ->assertJsonPath('data.status.name', 'OPEN')
            ->assertJsonPath('data.reporter.id', $employee->id)
            ->assertJsonPath('data.sla_duration_minutes', 240);

        $ticket = Ticket::first();
        expect($ticket->status_id)->toBe(1);
        expect($ticket->technician_id)->toBeNull();
        expect($ticket->ticket_number)->toMatch('/^TCK-\d{4,}$/');
        expect($ticket->department_id)->toBe($employee->department_id);
        expect($ticket->sla_deadline)->not->toBeNull();
    },
);

test(
    'proses create mengabaikan field reporter_id, status_id, dan sla dari request client.',
    function () {
        $employee = User::factory()->employee()->create();
        $other = User::factory()->employee()->create();
        Sanctum::actingAs($employee);

        $this->postJson('/api/tickets', [
            'title' => 'Test',
            'description' => 'Test',
            'category_id' => TicketCategory::first()->id,
            'priority_id' => 1,
            'reporter_id' => $other->id,
            'status_id' => 5,
            'sla_duration_minutes' => 9999,
            'sla_deadline' => '2026-09-01T00:00:00Z',
            'department_id' => 999,
        ])->assertStatus(201);

        $ticket = Ticket::first();
        expect($ticket->reporter_id)->toBe($employee->id);
        expect($ticket->status_id)->toBe(1);
        expect($ticket->sla_duration_minutes)->toBe(120); // priority 1 = Critical
        expect($ticket->department_id)->toBe($employee->department_id);
    },
);

test(
    'proses create secara bersamaan (concurrent) tetap menghasilkan nomor tiket yang unik tanpa ada yang bentrok.',
    function () {
        $employee = User::factory()->employee()->create();
        Sanctum::actingAs($employee);

        for ($i = 0; $i < 20; $i++) {
            $this->postJson('/api/tickets', [
                'title' => "Test Ticket {$i}",
                'description' => 'Test Description',
                'category_id' => TicketCategory::where(
                    'name',
                    'Laptop',
                )->first()->id,
                'priority_id' => 1,
            ])->assertStatus(201);
        }

        $numbers = Ticket::pluck('ticket_number');
        expect($numbers->unique()->count())->toBe(20);
    },
);

test(
    'proses create menggunakan aset milik user lain akan di-reject dengan HTTP 422.',
    function () {
        $employeeA = User::factory()->employee()->create();
        $employeeB = User::factory()->employee()->create();
        Sanctum::actingAs($employeeA);

        $asset = Asset::factory()->create(['status' => AssetStatus::Assigned]);
        AssetAssignment::factory()->create([
            'asset_id' => $asset->id,
            'user_id' => $employeeB->id,
            'released_at' => null,
        ]);

        $this->postJson('/api/tickets', [
            'title' => 'Test',
            'description' => 'Test',
            'category_id' => TicketCategory::where('name', 'Laptop')->first()
                ->id,
            'priority_id' => 1,
            'asset_id' => $asset->id,
        ])
            ->assertStatus(422)
            ->assertJsonPath('errors.asset_id', [
                'Asset yang dipilih tidak sedang ter-assign kepada Anda.',
            ]);
    },
);

test(
    'proses create menggunakan aset milik sendiri berhasil berproses (200/201 OK).',
    function () {
        $employee = User::factory()->employee()->create();
        Sanctum::actingAs($employee);

        $asset = Asset::factory()->create(['status' => AssetStatus::Assigned]);
        AssetAssignment::factory()->create([
            'asset_id' => $asset->id,
            'user_id' => $employee->id,
            'released_at' => null,
        ]);

        $this->postJson('/api/tickets', [
            'title' => 'Test',
            'description' => 'Test',
            'category_id' => TicketCategory::where('name', 'Laptop')->first()
                ->id,
            'priority_id' => 1,
            'asset_id' => $asset->id,
        ])->assertStatus(201);
    },
);

test(
    'proses create otomatis mencatat status history dan audit log.',
    function () {
        $employee = User::factory()->employee()->create();
        Sanctum::actingAs($employee);

        $this->postJson('/api/tickets', [
            'title' => 'Test',
            'description' => 'Test',
            'category_id' => TicketCategory::where('name', 'Laptop')->first()
                ->id,
            'priority_id' => 1,
        ])->assertStatus(201);

        $ticket = Ticket::first();
        expect($ticket->histories()->count())->toBe(1);
        expect($ticket->histories()->first()->field_changed)->toBe('status_id');
        expect($ticket->histories()->first()->new_value)->toBe('OPEN');
        expect(
            AuditLog::where('module', 'ticket')
                ->where('action', 'create')
                ->count(),
        )->toBe(1);
    },
);
