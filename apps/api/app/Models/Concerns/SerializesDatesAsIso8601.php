<?php

namespace App\Models\Concerns;

use DateTimeInterface;
use Illuminate\Support\Carbon;

trait SerializesDatesAsIso8601
{
    /**
     * Mempersiapkan tanggal untuk array / JSON serialisasi.
     */
    protected function serializeDate(DateTimeInterface $date): string
    {
        return Carbon::instance($date)
            ->setTimezone('UTC')
            ->format("Y-m-d\TH:i:s\Z");
    }
}
