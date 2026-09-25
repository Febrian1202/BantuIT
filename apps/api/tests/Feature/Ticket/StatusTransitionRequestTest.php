<?php

use App\Models\Ticket;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

uses()->group("ticket");

beforeEach(function () {
    $this->ticket = Ticket::factory()->open()->create();
    Sanctum::actingAs(User::factory()->manager()->create());
});

test("status transition memerlukan status_id yang valid", function () {
    $this->postJson("/api/tickets/{$this->ticket->id}/status", [
        "status_id" => 99,
    ])->assertStatus(422);
});

test(
    "assign memerlukan technician_id yang valid dengan role technician",
    function () {
        $this->postJson("/api/tickets/{$this->ticket->id}/assign", [
            "technician_id" => 999,
        ])->assertStatus(422);
    },
);

test("assign menolak non-technician user", function () {
    $employee = User::factory()->employee()->create();
    $this->postJson("/api/tickets/{$this->ticket->id}/assign", [
        "technician_id" => $employee->id,
    ])
        ->assertStatus(422)
        ->assertJsonPath(
            "errors.technician_id.0",
            "Teknisi yang dipilih tidak valid atau tidak aktif.",
        );
});

test("priority change memerlukan priority_id yang valid", function () {
    $this->postJson("/api/tickets/{$this->ticket->id}/priority", [
        "priority_id" => 99,
    ])->assertStatus(422);
});

test("expected_status_id harus integer jika disediakan", function () {
    $this->postJson("/api/tickets/{$this->ticket->id}/status", [
        "status_id" => 3,
        "expected_status_id" => "abc",
    ])->assertStatus(422);
});
