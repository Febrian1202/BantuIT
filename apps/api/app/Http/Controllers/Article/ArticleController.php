<?php

namespace App\Http\Controllers\Article;

use App\DTOs\Article\CreateArticleData;
use App\DTOs\Article\UpdateArticleData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Article\IndexArticleRequest;
use App\Http\Requests\Article\StoreArticleRequest;
use App\Http\Requests\Article\UpdateArticleRequest;
use App\Http\Resources\Article\ArticleListResource;
use App\Http\Resources\Article\ArticleResource;
use App\Models\KnowledgeArticle;
use App\Services\Article\ArticleQueryService;
use App\Services\Article\ArticleService;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class ArticleController extends Controller
{
    public function __construct(
        private ArticleQueryService $queryService,
        private ArticleService $articleService,
    ) {}

    /**
     * Menampilkan daftar artikel
     */
    public function index(IndexArticleRequest $request): JsonResponse
    {
        $this->authorize("viewAny", KnowledgeArticle::class);

        $paginator = $this->queryService->paginate($request, $request->user());

        return ApiResponse::paginated(
            $paginator,
            "Articles retrieved successfully.",
            ArticleListResource::class,
        );
    }

    /**
     * Menampilkan detail artikel berdasarkan ID
     */
    public function show(KnowledgeArticle $article): JsonResponse
    {
        $this->authorize("view", $article);

        $resource = $this->queryService->show($article);

        return ApiResponse::success(
            $resource,
            "Article retrieved successfully.",
        );
    }

    /**
     * Mengambil artikel untuk diedit
     */
    public function edit(KnowledgeArticle $article): JsonResponse
    {
        $this->authorize("update", $article);

        return ApiResponse::success(
            new ArticleResource($article->load(["category", "author"])),
            "Article retrieved for editing.",
        );
    }

    /**
     * Membuat artikel baru
     */
    public function store(StoreArticleRequest $request): JsonResponse
    {
        $this->authorize("create", KnowledgeArticle::class);

        $dto = CreateArticleData::fromArray($request->validated());
        $article = $this->articleService->create($dto, $request->user());

        return ApiResponse::success(
            new ArticleResource($article),
            "Article created successfully.",
        );
    }

    /**
     * Mengupdate artikel yang sudah ada
     */
    public function update(
        UpdateArticleRequest $request,
        KnowledgeArticle $article,
    ): JsonResponse {
        $this->authorize("update", $article);

        $dto = UpdateArticleData::fromArray($request->validated());
        $updated = $this->articleService->update(
            $article,
            $dto,
            $request->user(),
        );

        return ApiResponse::success(
            new ArticleResource($updated),
            "Article updated successfully.",
        );
    }

    /**
     * Menghapus artikel yang sudah ada
     */
    public function destroy(
        KnowledgeArticle $article,
        Request $request,
    ): JsonResponse {
        $this->authorize("delete", $article);
        $this->articleService->delete($article, $request->user());

        return ApiResponse::success(null, "Article deleted successfully.");
    }

    /**
     * Mengpublish artikel yang sudah ada
     */
    public function publish(
        Request $request,
        KnowledgeArticle $article,
    ): JsonResponse {
        $this->authorize("publish", $article);

        $published = $this->articleService->publish($article, $request->user());

        return ApiResponse::success(
            new ArticleResource($published),
            "Article published successfully.",
        );
    }

    /**
     * Membatalkan publish artikel yang sudah ada
     */
    public function unpublish(
        Request $request,
        KnowledgeArticle $article,
    ): JsonResponse {
        $this->authorize("unpublish", $article);

        $unpublished = $this->articleService->unpublish(
            $article,
            $request->user(),
        );

        return ApiResponse::success(
            new ArticleResource($unpublished),
            "Article unpublished successfully.",
        );
    }
}
