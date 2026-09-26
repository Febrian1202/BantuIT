<?php

namespace App\Console\Commands;

use App\Services\Sla\SlaBreachDetector;
use Illuminate\Console\Attributes\Description;
use Illuminate\Console\Attributes\Signature;
use Illuminate\Console\Command;

#[Signature('tickets:check-sla {--chunk=250 : Jumlah ticket per chunk}')]
#[
    Description(
        'Scan ticket aktif yang melewati SLA deadline, tandai sebagai breached, dan memicu notifactions',
    ),
]
class CheckTicketSlaCommand extends Command
{
    public function handle(SlaBreachDetector $detector): int
    {
        $this->info('Memulai pengecekan SLA...');

        $chunk = (int) $this->option('chunk');
        $result = $detector->scan($chunk);

        $this->info(
            sprintf(
                'Pemeriksaan SLA selesai: %d diperiksa, %d terdeteksi breached, %d notifications dibuat.',
                $result->checkedCount,
                $result->breachedCount,
                $result->notifiedCount,
            ),
        );

        return self::SUCCESS;
    }
}
