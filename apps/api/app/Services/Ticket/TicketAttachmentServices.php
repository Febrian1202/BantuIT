<?php

namespace App\Services\Ticket;

use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Models\Ticket;
use App\Models\TicketAttachment;
use App\Models\User;
use App\Services\Audit\AuditLogger;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Http\UploadedFile;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Str;
use Symfony\Component\HttpFoundation\StreamedResponse;
use Symfony\Component\HttpKernel\Exception\NotFoundHttpException;

class TicketAttachmentServices
{
    public function __construct(private AuditLogger $auditLogger) {}

    /**
     * Mendapatkan daftar attachment untuk ticket tertentu.
     */
    public function index(Ticket $ticket): Collection
    {
        return $ticket
            ->attachments()
            ->with('uploader')
            ->orderBy('created_at')
            ->get();
    }

    /**
     * Menyimpan attachment baru untuk ticket tertentu.
     */
    public function store(
        Ticket $ticket,
        UploadedFile $file,
        User $actor,
    ): TicketAttachment {
        $ulid = (string) Str::ulid();
        $extension = strtolower($file->getClientOriginalExtension());
        $path = "tickets/{$ticket->id}/{$ulid}.{$extension}";

        // Simpan file ke storage
        Storage::disk('private')->put($path, $file->get());

        try {
            return DB::transaction(function () use (
                $ticket,
                $file,
                $actor,
                $path,
            ): TicketAttachment {
                $attachment = TicketAttachment::create([
                    'ticket_id' => $ticket->id,
                    'uploaded_by' => $actor->id,
                    'original_filename' => $file->getClientOriginalName(),
                    'stored_filename' => basename($path),
                    'mime_type' => $file->getMimeType(),
                    'file_size' => $file->getSize(),
                    'storage_path' => $path,
                ]);

                $attachment->load('uploader');

                $this->auditLogger->log(
                    $actor,
                    AuditAction::Create,
                    AuditModule::Ticket,
                    $ticket->id,
                    "File {$file->getClientOriginalName()} ditambahkan ke ticket #{$ticket->ticket_number}.",
                );

                return $attachment;
            });
        } catch (\Throwable $e) {
            if (Storage::disk('private')->exists($path)) {
                Storage::disk('private')->delete($path);
            }
            throw $e;
        }
    }

    /**
     * Download file attachment untuk ticket
     */
    public function download(TicketAttachment $attachment): StreamedResponse
    {
        $filepath = $attachment->storage_path;

        if (! Storage::disk('private')->exists($filepath)) {
            throw new NotFoundHttpException('Attachment file not found.');
        }

        return Storage::disk('private')->download(
            $filepath,
            $attachment->original_filename,
            [
                'Content-Type' => $attachment->mime_type,
            ],
        );
    }

    /**
     * Hapus file attachment dari ticket
     */
    public function destroy(TicketAttachment $attachment, User $actor): void
    {
        DB::transaction(function () use ($attachment, $actor): void {
            $attachment->delete();

            $this->auditLogger->log(
                $actor,
                AuditAction::Delete,
                AuditModule::Ticket,
                $attachment->ticket_id,
                "File {$attachment->originale_filename} dihapus dari ticket #{$attachment->ticket->ticket_number}.",
            );
        });
    }
}
