<?php

namespace App\Http\Controllers\Article;

use App\DTOs\Article\CreateKnowledgeCategoryData;
use App\DTOs\Article\UpdateKnowledgeCategoryData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Article\StoreKnowledgeCategoryRequest;
use App\Http\Requests\Article\UpdateKnowledgeCategoryRequest;
use App\Http\Resources\Article\KnowledgeCategoryResource;
use App\Models\KnowledgeCategory;
use App\Services\Article\KnowledgeCategoryService;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class KnowledgeCategoryController extends Controller
{
    public function __construct(
        private KnowledgeCategoryService $categoryService,
    ) {}

    /**
     * Mengambil daftar kategori artikel.
     */
    public function index(): JsonResponse
    {
        $this->authorize("knowledge-category.viewAny");

        $categories = $this->categoryService->index();
        return ApiResponse::success(
            KnowledgeCategoryResource::collection($categories),
            "Knowledge categories retrieved successfully.",
        );
    }

    public function show(KnowledgeCategory $knowledgeCategory): JsonResponse
    {
        $this->authorize("knowledge-category.viewAny");

        return ApiResponse::success(
            new KnowledgeCategoryResource(
                $knowledgeCategory->loadCount("articles"),
            ),
            "Knowledge category retrieved successfully.",
        );
    }

    public function store(StoreKnowledgeCategoryRequest $request): JsonResponse
    {
        $this->authorize("knowledge-category.manage");

        $dto = CreateKnowledgeCategoryData::fromArray($request->validated());
        $category = $this->categoryService->store(
            data: $dto,
            actor: $request->user(),
        );

        return ApiResponse::success(
            new KnowledgeCategoryResource($category),
            "Knowledge category created successfully.",
        );
    }

    public function update(
        UpdateKnowledgeCategoryRequest $request,
        KnowledgeCategory $knowledgeCategory,
    ): JsonResponse {
        $this->authorize("knowledge-category.manage");

        $dto = UpdateKnowledgeCategoryData::fromArray($request->validated());
        $category = $this->categoryService->update(
            data: $dto,
            actor: $request->user(),
            category: $knowledgeCategory,
        );

        return ApiResponse::success(
            new KnowledgeCategoryResource($category),
            "Knowledge category updated successfully.",
        );
    }

    public function destroy(
        KnowledgeCategory $knowledgeCategory,
        Request $request,
    ): JsonResponse {
        $this->authorize("knowledge-category.manage");

        $this->categoryService->destroy(
            actor: $request->user(),
            category: $knowledgeCategory,
        );

        return ApiResponse::success(null, "Knowledge category deleted.");
    }
}
