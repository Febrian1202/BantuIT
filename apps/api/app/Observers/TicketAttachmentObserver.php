<?php

namespace App\Observers;

use App\Models\TicketAttachment;
use Illuminate\Support\Facades\Storage;

class TicketAttachmentObserver
{
    public bool $afterCommit = true;

    public function deleted(TicketAttachment $ticketAttachment): void
    {
        if (
            $ticketAttachment->storage_path &&
            Storage::disk('private')->exists($ticketAttachment->storage_path)
        ) {
            Storage::disk('private')->delete($ticketAttachment->storage_path);
        }
    }
}
