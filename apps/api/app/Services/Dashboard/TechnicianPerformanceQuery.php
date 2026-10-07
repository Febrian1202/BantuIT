<?php

namespace App\Services\Dashboard;

use App\Models\User;
use Illuminate\Database\Eloquent\Builder;

class TechnicianPerformanceQuery
{
    public function __construct(
        protected DashboardQueryService $queryService,
    ) {}

    /**
     * Menghitung performa teknisi berdasarkan peran (role) dan rentang waktu tertentu.
     */
    public function forRole(
        Builder $ticketQuery,
        DashboardDateRange $range,
    ): array {
        $diff = $this->queryService->minutesDiff('created_at', 'resolved_at');

        // Tiket dengan teknisi yang terassign
        $base = (clone $ticketQuery)->whereNotNull('technician_id');

        // Metrik tiket yang telah ditutup dalam rentang waktu tertentu
        $resolvedStats = (clone $base)
            ->whereNotNull('resolved_at')
            ->whereBetween('resolved_at', [$range->fromUtc, $range->toUtc])
            ->selectRaw(
                "
                    technician_id,
                    COUNT(*) as resolved,
                    SUM(CASE WHEN resolved_at <= sla_deadline THEN 1 ELSE 0 END) as within_sla,
                    ROUND(AVG({$diff})) as avg_minutes
                ",
            )
            ->groupBy('technician_id')
            ->get()
            ->keyBy('technician_id');

        // Jumlah tiket yang masih terbuka (snapshot saat ini, tanpa rentang waktu)
        $open = (clone $base)
            ->whereHas('status', fn ($q) => $q->where('is_closed', false))
            ->selectRaw('technician_id, COUNT(*) as open_count')
            ->groupBy('technician_id')
            ->pluck('open_count', 'technician_id');

        // Jumlah tiket yang masih terbuka dan terbreach (defensive)
        $breached = (clone $base)
            ->whereHas('status', fn ($q) => $q->where('is_closed', false))
            ->where(function ($q) {
                $q->where('sla_breached', true)->orWhere(function ($sub) {
                    $sub->whereNotNull('sla_deadline')->where(
                        'sla_deadline',
                        '<',
                        now(),
                    );
                });
            })
            ->selectRaw('technician_id, COUNT(*) as breached_count')
            ->groupBy('technician_id')
            ->pluck('breached_count', 'technician_id');

        // Jumlah tiket yang telah ditangani (semua status)
        $handled = (clone $base)
            ->selectRaw('technician_id, COUNT(*) as total')
            ->groupBy('technician_id')
            ->pluck('total', 'technician_id');

        // Build result
        // Menggabungkan ID teknisi yang telah ditangani dan yang telah ditutup dalam rentang waktu tertentu
        $techIds = collect($handled->keys())
            ->merge($resolvedStats->keys())
            ->unique()
            ->sort();

        $users = User::whereIn('id', $techIds)->pluck('full_name', 'id');

        $result = [];
        foreach ($techIds as $techId) {
            $res = $resolvedStats->get($techId);
            $totalResolved = $res ? (int) $res->resolved : 0;
            $withinSla = $res ? (int) $res->within_sla : 0;
            $compliance =
                $totalResolved > 0
                    ? round(($withinSla / $totalResolved) * 100, 1)
                    : null;

            $result[] = [
                'technician' => [
                    'id' => (int) $techId,
                    'full_name' => $users->get($techId) ?? 'Unknown',
                ],
                'handled' => (int) ($handled->get($techId) ?? 0),
                'resolved' => $totalResolved,
                'open' => (int) ($open->get($techId) ?? 0),
                'breached' => (int) ($breached->get($techId) ?? 0),
                'avg_resolution_minutes' => $res?->avg_minutes !== null
                        ? (int) $res->avg_minutes
                        : null,
                'sla_compliance_percentage' => $compliance,
            ];
        }

        // Mengurutkan hasil berdasarkan jumlah tiket yang telah ditutup secara descending
        usort($result, fn ($a, $b) => $b['resolved'] <=> $a['resolved']);

        return $result;
    }
}
