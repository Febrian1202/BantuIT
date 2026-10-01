<?php

namespace App\Http\Controllers\Admin;

use App\DTOs\Admin\CreateTicketCategoryData;
use App\DTOs\Admin\UpdateTicketCategoryData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Admin\StoreTicketCategoryRequest;
use App\Http\Requests\Admin\UpdateTicketCategoryRequest;
use App\Models\TicketCategory;
use App\Services\Ticket\TicketCategoryServices;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class TicketCategoryController extends Controller
{
    public function __construct(
        private TicketCategoryServices $categoryServices,
    ) {}

    /**
     * Mendapatkan daftar kategori tiket.
     */
    public function index(): JsonResponse
    {
        $this->authorize('ticket-category.viewAny');

        $categories = $this->categoryServices->index();

        return ApiResponse::success(
            $categories,
            'Ticket categories retrieved.',
        );
    }

    /**
     * Menampilkan detail kategori tiket.
     */
    public function show(TicketCategory $ticketCategory): JsonResponse
    {
        $this->authorize('ticket-category.viewAny');

        $category = $this->categoryServices->show($ticketCategory);

        return ApiResponse::success($category, 'Ticket category retrieved.');
    }

    /**
     * Membuat kategori tiket baru.
     */
    public function store(StoreTicketCategoryRequest $request): JsonResponse
    {
        $this->authorize('ticket-category.manage');

        $dto = CreateTicketCategoryData::fromArray($request->validated());
        $category = $this->categoryServices->store($dto, $request->user());

        return ApiResponse::created(
            $category,
            'Ticket category created successfully.',
        );
    }

    /**
     * Memperbarui kategori tiket yang sudah ada.
     */
    public function update(
        UpdateTicketCategoryRequest $request,
        TicketCategory $ticketCategory,
    ): JsonResponse {
        $this->authorize('ticket-category.manage');

        $dto = UpdateTicketCategoryData::fromArray($request->validated());
        $category = $this->categoryServices->update(
            $dto,
            $ticketCategory,
            $request->user(),
        );

        return ApiResponse::success(
            $category,
            'Ticket category updated successfully.',
        );
    }

    /**
     * Menghapus kategori tiket yang sudah ada.
     */
    public function destroy(
        TicketCategory $ticketCategory,
        Request $request,
    ): JsonResponse {
        $this->authorize('ticket-category.manage');

        $this->categoryServices->destroy($ticketCategory, $request->user());

        return ApiResponse::success(
            null,
            'Ticket category deleted successfully.',
        );
    }
}
