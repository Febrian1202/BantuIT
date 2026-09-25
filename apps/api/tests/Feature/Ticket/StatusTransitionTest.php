<?php

use App\Models\Ticket;
use App\Models\TicketCategory;
use App\Models\TicketPriority;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

uses()->group("ticket");

// Golden Path

test(
    "siklus hidup utuh (full lifecycle) yang dijalankan via HTTP request.",
    function () {
        $employee = User::factory()->employee()->create();
        $manager = User::factory()->manager()->create();
        $technician = User::factory()->technician()->create();

        Sanctum::actingAs($employee);
        $ticket = $this->postJson("/api/tickets", [
            "title" => "Laptop mati",
            "description" => "Tidak boot",
            "category_id" => TicketCategory::where("name", "Laptop")->first()
                ->id,
            "priority_id" => 1,
        ])
            ->assertStatus(201)
            ->assertJsonPath("data.status.name", "OPEN")
            ->assertJsonPath("data.reporter.id", $employee->id)
            ->json("data");
        $id = $ticket["id"];

        Sanctum::actingAs($manager);
        $this->postJson("/api/tickets/{$id}/assign", [
            "technician_id" => $technician->id,
        ])
            ->assertStatus(200)
            ->assertJsonPath("data.status.name", "ASSIGNED")
            ->assertJsonPath("data.technician.id", $technician->id);

        Sanctum::actingAs($technician);
        $this->postJson("/api/tickets/{$id}/status", ["status_id" => 3])
            ->assertStatus(200)
            ->assertJsonPath("data.status.name", "IN_PROGRESS");

        $this->postJson("/api/tickets/{$id}/status", ["status_id" => 4])
            ->assertStatus(200)
            ->assertJsonPath("data.status.name", "RESOLVED")
            ->assertJsonPath("data.resolved_at", fn($v) => $v !== null);

        Sanctum::actingAs($employee);
        $this->postJson("/api/tickets/{$id}/status", ["status_id" => 5])
            ->assertStatus(200)
            ->assertJsonPath("data.status.name", "CLOSED")
            ->assertJsonPath("data.closed_at", fn($v) => $v !== null);
    },
);

// Illegal Transitions lewat HTTP request.

test(
    "transisi status dari OPEN ke RESOLVED akan di-reject dengan HTTP 422 dan pesan eror bahasa Indonesia.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 4])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari OPEN ke RESOLVED.",
            );
    },
);

test(
    "transisi status dari ASSIGNED ke RESOLVED akan di-reject dengan HTTP 422 dan pesan eror bahasa Indonesia.",
    function () {
        $ticket = Ticket::factory()->assigned()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 4])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari ASSIGNED ke RESOLVED.",
            );
    },
);

test(
    "transisi ke status yang sama (same status transition) akan di-reject dengan HTTP 422 dan pesan eror bahasa Indonesia.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 1])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari OPEN ke OPEN.",
            );
    },
);

test(
    "transisi status dari CLOSED ke IN_PROGRESS akan di-reject dengan HTTP 422 dan pesan eror bahasa Indonesia.",
    function () {
        $ticket = Ticket::factory()->closed()->create();
        Sanctum::actingAs(User::factory()->admin()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 3])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari CLOSED ke IN_PROGRESS.",
            );
    },
);

test(
    "transisi status dari RESOLVED ke OPEN akan di-reject dengan HTTP 422 dan pesan eror bahasa Indonesia.",
    function () {
        $ticket = Ticket::factory()->resolved()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 1])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari RESOLVED ke OPEN.",
            );
    },
);

test(
    "transisi status dari IN_PROGRESS ke OPEN akan di-reject dengan HTTP 422 dan pesan eror bahasa Indonesia.",
    function () {
        $ticket = Ticket::factory()->inProgress()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 1])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari IN_PROGRESS ke OPEN.",
            );
    },
);

test(
    "transisi ke status ASSIGNED via endpoint status akan di-reject dengan HTTP 422.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 2])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari OPEN ke ASSIGNED melalui endpoint ini.",
            );
    },
);

// Konkurensi 409 vs 422 vs 403/404

test(
    "pengiriman expected_status_id yang stale akan memicu response HTTP 409 Conflict.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 3,
            "expected_status_id" => 3,
        ])
            ->assertStatus(409)
            ->assertJsonPath(
                "message",
                "Ticket status has changed since it was loaded. Please refresh and try again.",
            );
    },
);

test(
    "Pengiriman expected_status_id yang stale walaupun dengan transisi yang ilegal tetap mengembalikan HTTP 409, bukan 422.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 2,
            "expected_status_id" => 2,
        ])->assertStatus(409);
    },
);

test(
    "Unauthorized user yang mengirim expected_status_id stale akan mendapat response 403/404, bukan 409.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->employee()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 3,
            "expected_status_id" => 3,
        ])->assertStatus(404);
    },
);

// Self-assign, Reopen, Priority, Cancel via HTTP

test("technician self-assigns via HTTP dengan dua baris riwayat", function () {
    $employee = User::factory()->employee()->create();
    $technician = User::factory()->technician()->create();
    $ticket = Ticket::factory()
        ->open()
        ->create(["reporter_id" => $employee->id]);
    Sanctum::actingAs($technician);

    $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 3])
        ->assertStatus(200)
        ->assertJsonPath("data.status.name", "IN_PROGRESS")
        ->assertJsonPath("data.technician.id", $technician->id);

    expect($ticket->fresh()->histories()->count())->toBe(2);
});

test(
    "proses reopen tetap mempertahankan sla_deadline dan status sla_breached yang lama.",
    function () {
        $employee = User::factory()->employee()->create();
        $ticket = Ticket::factory()
            ->resolved()
            ->breached()
            ->create(["reporter_id" => $employee->id]);
        $deadline = $ticket->sla_deadline;
        Sanctum::actingAs($employee);

        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 3,
        ])->assertStatus(200);

        $t = $ticket->fresh();
        expect($t->resolved_at)->toBeNull();
        expect($t->sla_deadline->equalTo($deadline))->toBeTrue();
        expect($t->sla_breached)->toBeTrue();
    },
);

test(
    "Proses cancel dari status IN_PROGRESS tanpa mencantumkan note akan di-reject dengan HTTP 422.",
    function () {
        $ticket = Ticket::factory()->inProgress()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", ["status_id" => 5])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Status tidak dapat diubah dari IN_PROGRESS ke CLOSED tanpa alasan. Sertakan note untuk membatalkan ticket.",
            );
    },
);

test(
    "Proses cancel dari status IN_PROGRESS yang disertai note berhasil diproses dan otomatis membuat comment.",
    function () {
        $ticket = Ticket::factory()->inProgress()->create();
        $manager = User::factory()->manager()->create();
        Sanctum::actingAs($manager);
        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 5,
            "note" => "Dibatalkan oleh user",
        ])
            ->assertStatus(200)
            ->assertJsonPath("data.status.name", "CLOSED");

        expect(
            $ticket
                ->fresh()
                ->comments()
                ->where("body", "Dibatalkan oleh user")
                ->exists(),
        )->toBeTrue();
    },
);

test(
    "Proses priority change akan menghitung ulang sla_deadline berpatokan dari created_at.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $newPriority = TicketPriority::find(1); // 120 menit
        $this->postJson("/api/tickets/{$ticket->id}/priority", [
            "priority_id" => 1,
        ])
            ->assertStatus(200)
            ->assertJsonPath("data.priority.id", 1);

        $t = $ticket->fresh();
        expect(
            $t->sla_deadline->equalTo(
                $t->created_at->copy()->addMinutes($newPriority->sla_minutes),
            ),
        )->toBeTrue();
    },
);

test(
    "Priority change pada ticket yang sudah CLOSED akan di-reject dengan HTTP 422.",
    function () {
        $ticket = Ticket::factory()->closed()->create();
        Sanctum::actingAs(User::factory()->manager()->create());
        $this->postJson("/api/tickets/{$ticket->id}/priority", [
            "priority_id" => 1,
        ])
            ->assertStatus(422)
            ->assertJsonPath(
                "errors.status_id.0",
                "Prioritas tidak dapat diubah pada ticket yang sudah ditutup/diresolusi.",
            );
    },
);

// Authorisasi dan Aturan Proteksi

test(
    "Admin tidak bisa melakukan reopen pada ticket yang berstatus CLOSED.",
    function () {
        $ticket = Ticket::factory()->closed()->create();
        Sanctum::actingAs(User::factory()->admin()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 3,
        ])->assertStatus(422);
    },
);

test("Technician tidak bisa close ticket RESOLVED milik sendiri.", function () {
    $technician = User::factory()->technician()->create();
    $ticket = Ticket::factory()
        ->resolved()
        ->create(["technician_id" => $technician->id]);
    Sanctum::actingAs($technician);
    $this->postJson("/api/tickets/{$ticket->id}/status", [
        "status_id" => 5,
    ])->assertStatus(422);
});

test(
    "Employee yang bukan reporter saat ubah status mendapat response 404.",
    function () {
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs(User::factory()->employee()->create());
        $this->postJson("/api/tickets/{$ticket->id}/status", [
            "status_id" => 3,
        ])->assertStatus(404);
    },
);

test("Manager tidak bisa assign employee yang bukan technician.", function () {
    $employee = User::factory()->employee()->create();
    $ticket = Ticket::factory()->open()->create();
    Sanctum::actingAs(User::factory()->manager()->create());
    $this->postJson("/api/tickets/{$ticket->id}/assign", [
        "technician_id" => $employee->id,
    ])
        ->assertStatus(422)
        ->assertJsonPath(
            "errors.technician_id.0",
            "Teknisi yang dipilih tidak valid atau tidak aktif.",
        );
});

test("Manager tidak bisa unassign ticket via HTTP", function () {
    $ticket = Ticket::factory()->assigned()->create();
    $manager = User::factory()->manager()->create();
    Sanctum::actingAs($manager);

    $this->postJson("/api/tickets/{$ticket->id}/unassign")
        ->assertStatus(200)
        ->assertJsonPath("data.status.name", "OPEN")
        ->assertJsonPath("data.technician", null);

    $t = $ticket->fresh();
    expect($t->status_id)->toBe(1);
    expect($t->technician_id)->toBeNull();
});

test("Technician tidak bisa unassign ticket via HTTP", function () {
    $technician = User::factory()->technician()->create();
    $ticket = Ticket::factory()
        ->assigned()
        ->create(["technician_id" => $technician->id]);
    Sanctum::actingAs($technician);

    $this->postJson("/api/tickets/{$ticket->id}/unassign")->assertStatus(403);
});

// Detail available_actions & editable_fields via HTTP

test(
    "show mengembalikan available_actions dan editable_fields untuk manager pada ticket OPEN",
    function () {
        $manager = User::factory()->manager()->create();
        $ticket = Ticket::factory()->open()->create();
        Sanctum::actingAs($manager);
        $this->getJson("/api/tickets/{$ticket->id}")
            ->assertOk()
            ->assertJsonPath("data.available_actions", [
                "assign",
                "cancel",
                "change_priority",
                "comment",
                "attach",
                "edit",
            ])
            ->assertJsonPath("data.editable_fields", [
                "title",
                "description",
                "category_id",
            ]);
    },
);

test(
    "show mengembalikan available_actions dan editable_fields untuk technician pemegang pada ticket IN_PROGRESS",
    function () {
        $technician = User::factory()->technician()->create();
        $ticket = Ticket::factory()
            ->inProgress()
            ->create(["technician_id" => $technician->id]);
        Sanctum::actingAs($technician);
        $this->getJson("/api/tickets/{$ticket->id}")
            ->assertOk()
            ->assertJsonPath("data.available_actions", [
                "resolve",
                "change_priority",
                "comment",
                "attach",
                "edit",
            ])
            ->assertJsonPath("data.editable_fields", [
                "title",
                "description",
                "category_id",
            ]);
    },
);

test(
    "show mengembalikan available_actions dan editable_fields untuk Employee reporter pada ticket RESOLVED",
    function () {
        $employee = User::factory()->employee()->create();
        $ticket = Ticket::factory()
            ->resolved()
            ->create(["reporter_id" => $employee->id]);
        Sanctum::actingAs($employee);
        $this->getJson("/api/tickets/{$ticket->id}")
            ->assertOk()
            ->assertJsonPath("data.available_actions", [
                "close",
                "reopen",
                "comment",
                "attach",
                "edit",
            ])
            ->assertJsonPath("data.editable_fields", ["title", "description"]);
    },
);

test(
    "show mengembalikan available_actions dan editable_fields untuk Admin pada ticket CLOSED",
    function () {
        $admin = User::factory()->admin()->create();
        $ticket = Ticket::factory()->closed()->create();
        Sanctum::actingAs($admin);
        $this->getJson("/api/tickets/{$ticket->id}")
            ->assertOk()
            ->assertJsonPath("data.available_actions", [])
            ->assertJsonPath("data.editable_fields", []);
    },
);
