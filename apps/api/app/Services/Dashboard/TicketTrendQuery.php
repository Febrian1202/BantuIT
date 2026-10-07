<?php

namespace App\Services\Dashboard;

use Illuminate\Database\Eloquent\Builder;

class TicketTrendQuery
{
    public function __construct(
        protected DashboardQueryService $queryService,
    ) {}

    /**
     * Mengambil data tren tiket per hari dalam rentang waktu tertentu.
     */
    public function daily(
        Builder $ticketQuery,
        DashboardDateRange $range,
    ): array {
        $bucket = $this->queryService->dateBucket("created_at");
        $resolvedBucket = $this->queryService->dateBucket("resolved_at");

        // Membuat query untuk menghitung jumlah tiket yang dibuat per hari
        $created = (clone $ticketQuery)
            ->selectRaw("{$bucket} as date, COUNT(*) as count")
            ->whereBetween("created_at", [$range->fromUtc, $range->toUtc])
            ->groupBy("date")
            ->orderBy("date")
            ->pluck("count", "date")
            ->toArray();

        // Menghitung jumlah tiket yang selesai (diterima) per hari
        $resolved = (clone $ticketQuery)
            ->whereNotNull("resolved_at")
            ->selectRaw("{$resolvedBucket} as date, COUNT(*) as count")
            ->whereBetween("resolved_at", [$range->fromUtc, $range->toUtc])
            ->groupBy("date")
            ->orderBy("date")
            ->pluck("count", "date")
            ->toArray();

        // Mengisi semua hari dalam rentang waktu dengan data yang tersedia
        $current = $range->fromUtc->setTimezone("Asia/Jakarta")->startOfDay();
        $end = $range->toUtc->setTimezone("Asia/Jakarta")->startOfDay();
        $result = [];
        while ($current->lte($end)) {
            $date = $current->format("Y-m-d");
            $result[] = [
                "date" => $date,
                "created" => (int) ($created[$date] ?? 0),
                "resolved" => (int) ($resolved[$date] ?? 0),
            ];
            $current = $current->addDay();
        }

        return $result;
    }
}
