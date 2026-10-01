<?php

namespace App\Http\Controllers;

use App\Services\ReferenceService;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;

class ReferenceController extends Controller
{
    public function __construct(private ReferenceService $referenceService) {}

    /**
     * Mengambil semua kategori tiket.
     */
    public function categories(): JsonResponse
    {
        $this->authorize('ticket-category.viewAny');

        return ApiResponse::success(
            $this->referenceService->ticketCategories(),
            'Ticket categories retrieved.',
        );
    }

    /**
     * Mengambil semua prioritas tiket.
     */
    public function priorities(): JsonResponse
    {
        $this->authorize('ticket-priority.viewAny');

        return ApiResponse::success(
            $this->referenceService->ticketPriorities(),
            'Ticket priorities retrieved.',
        );
    }

    /**
     * Mengambil semua status tiket.
     */
    public function statuses(): JsonResponse
    {
        $this->authorize('ticket-status.viewAny');

        return ApiResponse::success(
            $this->referenceService->ticketStatuses(),
            'Ticket statuses retrieved.',
        );
    }

    /**
     * Mengambil semua teknisi yang aktif.
     */
    public function technicians(): JsonResponse
    {
        $this->authorize('technician.list');

        return ApiResponse::success(
            $this->referenceService->technicianList(),
            'Technicians retrieved.',
        );
    }
}
