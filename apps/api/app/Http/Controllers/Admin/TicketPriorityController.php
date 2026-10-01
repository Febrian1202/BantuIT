<?php

namespace App\Http\Controllers\Admin;

use App\DTOs\Admin\CreateTicketPriorityData;
use App\DTOs\Admin\UpdateTicketPriorityData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Admin\StoreTicketPriorityRequest;
use App\Http\Requests\Admin\UpdateTicketPriorityRequest;
use App\Models\TicketPriority;
use App\Services\Ticket\TicketPriorityServices;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class TicketPriorityController extends Controller
{
    public function __construct(
        private TicketPriorityServices $priorityServices,
    ) {}

    /**
     * Menampilkan daftar prioritas tiket.
     */
    public function index(): JsonResponse
    {
        $this->authorize("ticket-priority.viewAny");
        $priorities = $this->priorityServices->index();
        return ApiResponse::success(
            $priorities,
            "Ticket priorities retrieved.",
        );
    }

    /**
     * Menampilkan detail prioritas tiket.
     */
    public function show(TicketPriority $ticketPriority): JsonResponse
    {
        $this->authorize("ticket-priority.viewAny");

        return ApiResponse::success(
            $ticketPriority,
            "Ticket priority retrieved.",
        );
    }

    /**
     * Menyimpan prioritas tiket baru.
     */
    public function store(StoreTicketPriorityRequest $request): JsonResponse
    {
        $this->authorize("ticket-priority.manage");

        $dto = CreateTicketPriorityData::fromArray($request->validated());
        $priority = $this->priorityServices->store($dto, $request->user());

        return ApiResponse::created(
            $priority,
            "Ticket priority created successfully.",
        );
    }

    /**
     * Mengupdate prioritas tiket yang sudah ada.
     */
    public function update(
        UpdateTicketPriorityRequest $request,
        TicketPriority $ticketPriority,
    ): JsonResponse {
        $this->authorize("ticket-priority.manage");

        $dto = UpdateTicketPriorityData::fromArray($request->validated());
        $priority = $this->priorityServices->update(
            $dto,
            $ticketPriority,
            $request->user(),
        );

        return ApiResponse::success(
            $priority,
            "Ticket priority updated successfully.",
        );
    }

    /**
     * Menghapus prioritas tiket yang sudah ada.
     */
    public function destroy(
        TicketPriority $ticketPriority,
        Request $request,
    ): JsonResponse {
        $this->authorize("ticket-priority.manage");

        $this->priorityServices->destroy($ticketPriority, $request->user());

        return ApiResponse::success(
            null,
            "Ticket priority deleted successfully.",
        );
    }
}
