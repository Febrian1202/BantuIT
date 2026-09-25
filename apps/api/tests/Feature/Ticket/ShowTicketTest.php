<?php

use App\Models\Asset;
use App\Models\Ticket;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

uses()->group('ticket');

test(
    'endpoint show mengembalikan response dengan data ticket shape yang utuh/lengkap.',
    function () {
        $employee = User::factory()->employee()->create();
        $ticket = Ticket::factory()
            ->open()
            ->create(['reporter_id' => $employee->id]);
        Sanctum::actingAs($employee);

        $this->getJson("/api/tickets/{$ticket->id}")
            ->assertStatus(200)
            ->assertJsonStructure([
                'success',
                'message',
                'data' => [
                    'id',
                    'ticket_number',
                    'title',
                    'description',
                    'status',
                    'priority',
                    'category',
                    'reporter',
                    'technician',
                    'department',
                    'asset',
                    'sla_duration_minutes',
                    'sla_deadline',
                    'sla_breached',
                    'sla_status',
                    'sla_remaining_minutes',
                    'resolved_at',
                    'closed_at',
                    'comments_count',
                    'attachments_count',
                    'available_actions',
                    'editable_fields',
                    'created_at',
                    'updated_at',
                ],
            ])
            ->assertJsonPath('data.available_actions', [
                'comment',
                'attach',
                'edit',
            ])
            ->assertJsonPath('data.editable_fields', ['title', 'description']);
    },
);

test(
    'employee yang melihat ticket milik user lain mendapat response 404 Not Found.',
    function () {
        $owner = User::factory()->employee()->create();
        $intruder = User::factory()->employee()->create();
        $ticket = Ticket::factory()
            ->open()
            ->create(['reporter_id' => $owner->id]);
        Sanctum::actingAs($intruder);

        $this->getJson("/api/tickets/{$ticket->id}")->assertStatus(404);
    },
);

test(
    'endpoint show menyertakan soft-deleted asset lengkap dengan flag deleted.',
    function () {
        $asset = Asset::factory()->create();
        $employee = User::factory()->employee()->create();
        $ticket = Ticket::factory()
            ->open()
            ->create([
                'reporter_id' => $employee->id,
                'asset_id' => $asset->id,
            ]);
        $asset->delete();
        Sanctum::actingAs($employee);

        $this->getJson("/api/tickets/{$ticket->id}")->assertJsonPath(
            'data.asset.deleted',
            true,
        );
    },
);
