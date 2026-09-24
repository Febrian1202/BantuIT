<?php

use App\Models\Ticket;
use Illuminate\Foundation\Testing\RefreshDatabase;

uses(RefreshDatabase::class);

test(
    "State 'open' pada ticket factory menghasilkan tiket dengan status_id 1.",
    function () {
        $ticket = Ticket::factory()->open()->create();

        expect($ticket->status_id)->toBe(1);
        expect($ticket->technician_id)->toBeNull();
        expect($ticket->resolved_at)->toBeNull();
        expect($ticket->closed_at)->toBeNull();
    },
);

test(
    "State 'assigned' pada ticket factory mengisi nilai status dan teknisi.",
    function () {
        $ticket = Ticket::factory()->assigned()->create();

        expect($ticket->status_id)->toBe(2);
        expect($ticket->technician_id)->not->toBeNull();
    },
);

test(
    "State 'inProgress' pada ticket factory mengisi nilai status dan teknisi.",
    function () {
        $ticket = Ticket::factory()->inProgress()->create();

        expect($ticket->status_id)->toBe(3);
        expect($ticket->technician_id)->not->toBeNull();
    },
);

test(
    "State 'resolved' pada ticket factory mengisi nilai status dan tanggal penyelesaian (resolved_at).",
    function () {
        $ticket = Ticket::factory()->resolved()->create();

        expect($ticket->status_id)->toBe(4);
        expect($ticket->resolved_at)->not->toBeNull();
    },
);

test(
    "State 'closed' pada ticket factory mengisi nilai status dan tanggal penutupan (closed_at).",
    function () {
        $ticket = Ticket::factory()->closed()->create();

        expect($ticket->status_id)->toBe(5);
        expect($ticket->closed_at)->not->toBeNull();
    },
);

test(
    "State 'breached' pada ticket factory mengisi nilai atribut-atribut SLA yang terlampaui.",
    function () {
        $ticket = Ticket::factory()->breached()->create();

        expect($ticket->sla_breached)->toBeTrue();
        expect($ticket->sla_breached_at)->not->toBeNull();
        expect($ticket->sla_deadline->isPast())->toBeTrue();
    },
);
