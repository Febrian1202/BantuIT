<?php

namespace App\Services\Dashboard;

use Carbon\CarbonImmutable;
use Illuminate\Database\Eloquent\Builder;

class DashboardDateRange
{
    public function __construct(
        public readonly CarbonImmutable $fromUtc,
        public readonly CarbonImmutable $toUtc,
    ) {}

    /**
     * Mengembalikan rentang tanggal default untuk dashboard.
     */
    public static function default(): self
    {
        $todayWib = CarbonImmutable::now('Asia/Jakarta')->startOfDay();

        return new self(
            $todayWib->subDays(29)->setTimezone('UTC'),
            $todayWib->endOfDay()->setTimezone('UTC'),
        );
    }

    /**
     * Mengembalikan rentang tanggal berdasarkan tanggal awal dan akhir yang diberikan.
     */
    public static function fromDates(?string $dateFrom, ?string $dateTo): self
    {
        if ($dateFrom === null && $dateTo === null) {
            return self::default();
        }

        $tz = 'Asia/Jakarta';
        $fromWib =
            $dateFrom !== null
                ? CarbonImmutable::createFromFormat(
                    'Y-m-d',
                    $dateFrom,
                    $tz,
                )->startOfDay()
                : self::default()->fromUtc->setTimezone($tz);

        $toWib =
            $dateTo !== null
                ? CarbonImmutable::createFromFormat(
                    'Y-m-d',
                    $dateTo,
                    $tz,
                )->endOfDay()
                : $fromWib->endOfDay();

        return new self(
            $fromWib->setTimezone('UTC'),
            $toWib->setTimezone('UTC'),
        );
    }

    /**
     * Mengaplikasikan rentang tanggal ke kolom `created_at` pada query.
     */
    public function applyToCreated(Builder $query): Builder
    {
        return $query->whereBetween('created_at', [
            $this->fromUtc,
            $this->toUtc,
        ]);
    }

    /**
     * Mengaplikasikan rentang tanggal ke kolom `resolved_at` pada query.
     */
    public function applyToResolved(Builder $query): Builder
    {
        return $query
            ->whereNotNull('resolved_at')
            ->whereBetween('resolved_at', [$this->fromUtc, $this->toUtc]);
    }
}
