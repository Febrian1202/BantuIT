<?php

namespace App\Enums;

enum TicketStatusName: string
{
    case Open = 'OPEN';
    case Assigned = 'ASSIGNED';
    case InProgress = 'IN_PROGRESS';
    case Resolved = 'RESOLVED';
    case Closed = 'CLOSED';

    /**
     * Helper untuk mendapatkan enum dari ID
     */
    public static function fromId(int $id): self
    {
        return match ($id) {
            1 => self::Open,
            2 => self::Assigned,
            3 => self::InProgress,
            4 => self::Resolved,
            5 => self::Closed,
            default => throw new \ValueError("Invalid status ID: {$id}"),
        };
    }

    /**
     * Helper untuk mendapatkan ID dari enum
     */
    public function id(): int
    {
        return match ($this) {
            self::Open => 1,
            self::Assigned => 2,
            self::InProgress => 3,
            self::Resolved => 4,
            self::Closed => 5,
        };
    }

    /**
     * Helper untuk memeriksa apakah status adalah Closed atau Resolved
     */
    public function isClosed(): bool
    {
        return match ($this) {
            self::Closed, self::Resolved => true,
            default => false,
        };
    }

    /**
     * Helper untuk memeriksa apakah status sudah final
     */
    public function isFinal(): bool
    {
        return $this === self::Closed;
    }

    /**
     * Helper untuk mendapatkan label dari status
     */
    public function label(): string
    {
        return $this->value;
    }
}
