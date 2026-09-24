<?php
namespace App\Authorization;

use App\Enums\TicketActor;
use App\Enums\TicketStatusName;

/**
 * Membentuk/merealisasikan matriks transisi status tiket secara konkret.
 *
 * Baris = status asal, kolom = status tujuan. Setiap sel mendaftar nilai
 * TicketActor yang diizinkan untuk transisi tersebut; sel yang tidak ada (atau
 * baris CLOSED yang kosong) berarti transisi tersebut tidak valid/ilegal.
 * Penanda T(own) merepresentasikan TicketActor::Technician — kepemilikan
 * ditegakkan oleh policy/service, bukan di kelas ini. Penanda T* (self-assign)
 * adalah TicketActor::AnyTechnician dan hanya ada pada transisi OPEN -> IN_PROGRESS.
 */
class TicketTransitionMatrix
{
    /**
     * Matriks transisi status tiket.
     *
     * @var array<string, array<string, array<TicketActor>>>
     */
    private const TRANSITIONS = [
        "OPEN" => [
            "ASSIGNED" => [TicketActor::Manager, TicketActor::Admin],
            "IN_PROGRESS" => [
                TicketActor::AnyTechnician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            "CLOSED" => [TicketActor::Manager, TicketActor::Admin],
        ],
        "ASSIGNED" => [
            "OPEN" => [TicketActor::Manager, TicketActor::Admin],
            "IN_PROGRESS" => [
                TicketActor::Technician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            "CLOSED" => [TicketActor::Manager, TicketActor::Admin],
        ],
        "IN_PROGRESS" => [
            "ASSIGNED" => [TicketActor::Manager, TicketActor::Admin],
            "RESOLVED" => [
                TicketActor::Technician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            "CLOSED" => [TicketActor::Manager, TicketActor::Admin],
        ],
        "RESOLVED" => [
            "IN_PROGRESS" => [
                TicketActor::Reporter,
                TicketActor::Technician,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
            "CLOSED" => [
                TicketActor::Reporter,
                TicketActor::Manager,
                TicketActor::Admin,
            ],
        ],
        "CLOSED" => [],
    ];

    /**
     * Mendapatkan actor yang diperbolehkan melakukan transisi,
     * atau [] ketika illegal
     */
    public static function allowedRoles(
        TicketStatusName $from,
        TicketStatusName $to,
    ): array {
        return self::TRANSITIONS[$from->value][$to->value] ?? [];
    }

    public static function allows(
        TicketStatusName $from,
        TicketStatusName $to,
        TicketActor $actor,
    ): bool {
        if ($from === $to) {
            return false;
        }
        return in_array($actor, self::allowedRoles($from, $to), true);
    }
}
