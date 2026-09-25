<?php

namespace App\Services\Sla;

use App\Models\Ticket;
use App\Models\TicketPriority;
use Carbon\CarbonInterface;
use Illuminate\Database\Eloquent\Builder;

class SlaService
{
    public function calculateDeadline(
        CarbonInterface $createdAt,
        int $durationMinutes,
    ): CarbonInterface {
        // TODO: Implement nanti
        return $createdAt->addMinutes($durationMinutes);
    }

    public function snapshot(
        Ticket $ticket,
        TicketPriority $ticketPriority,
    ): void {
        // TODO: Implement nanti
    }

    public function recalculateFromCreation(
        Ticket $ticket,
        TicketPriority $priority,
    ): void {
        // TODO: Implement nanti
    }

    public function markBreached(Ticket $ticket): void
    {
        // TODO: Implement nanti
    }

    public function isBreached(Ticket $ticket): bool
    {
        // TODO: Implement nanti
        return false;
    }

    public function remainingMinutes(Ticket $ticket): ?int
    {
        // TODO: Implement nanti
        return null;
    }

    public function breachCandidates(): Builder
    {
        // TODO: Implement nanti
        return Ticket::query();
    }

    public function scopeBreached(Builder $query): Builder
    {
        // TODO: Implement nanti
        return $query;
    }

    public function scopeOnTrack(Builder $query): Builder
    {
        // TODO: Implement nanti
        return $query;
    }
}
