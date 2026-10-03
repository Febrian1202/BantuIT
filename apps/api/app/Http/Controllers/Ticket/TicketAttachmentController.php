<?php

namespace App\Http\Controllers\Ticket;

use App\Http\Controllers\Controller;
use App\Http\Requests\Ticket\StoreTicketAttachmentRequest;
use App\Http\Resources\Ticket\TicketAttachmentResource;
use App\Models\Ticket;
use App\Models\TicketAttachment;
use App\Services\Ticket\TicketAttachmentServices;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\StreamedResponse;

class TicketAttachmentController extends Controller
{
    public function __construct(
        protected TicketAttachmentServices $attachmentServices,
    ) {}

    /**
     * Menampilkan daftar attachment untuk ticket tertentu.
     */
    public function index(Ticket $ticket): JsonResponse
    {
        $this->authorize('view', $ticket);

        $attachments = $this->attachmentServices->index($ticket);

        return ApiResponse::success(
            TicketAttachmentResource::collection($attachments),
            'Attachments retrieved.',
        );
    }

    /**
     * Menyimpan attachment baru untuk ticket tertentu.
     */
    public function store(
        StoreTicketAttachmentRequest $request,
        Ticket $ticket,
    ): JsonResponse {
        $this->authorize('attach', $ticket);

        $attachment = $this->attachmentServices->store(
            $ticket,
            $request->file('file'),
            $request->user(),
        );

        return ApiResponse::created(
            new TicketAttachmentResource($attachment),
            'Attachment uploaded successfully',
        );
    }

    /**
     * Download attachment dari ticket
     */
    public function download(TicketAttachment $attachment): StreamedResponse
    {
        $this->authorize('download', $attachment);

        return $this->attachmentServices->download($attachment);
    }

    /**
     * Hapus attachment dari ticket
     */
    public function destroy(
        TicketAttachment $attachment,
        Request $request,
    ): JsonResponse {
        $this->authorize('delete', $attachment);

        $this->attachmentServices->destroy($attachment, $request->user());

        return ApiResponse::success(null, 'Attachment deleted.');
    }
}
