<?php

namespace App\Services\Article;

use App\Enums\ArticleStatus;
use App\Enums\RoleName;
use App\Http\Requests\Article\IndexArticleRequest;
use App\Http\Resources\Article\ArticleResource;
use App\Models\KnowledgeArticle;
use App\Models\User;
use App\Support\HandlesPagination;
use Illuminate\Pagination\LengthAwarePaginator;

class ArticleQueryService
{
    use HandlesPagination;

    /**
     * Mengembalikan daftar artikel yang telah dipaginasi.
     */
    public function paginate(
        IndexArticleRequest $request,
        User $actor,
    ): LengthAwarePaginator {
        $query = KnowledgeArticle::with(['category', 'author']);

        $isEmployee =
            ! $actor->isAdmin() &&
            ! $actor->hasRole(RoleName::Manager, RoleName::Technician);

        if ($isEmployee) {
            $query->where('status', ArticleStatus::Published->value);
        }

        if ($search = $request->query('search')) {
            $term = str_replace(['%', '_'], ['\\%', '\\_'], $search);
            $query->where(function ($q) use ($term) {
                $q->whereRaw('title LIKE ? ESCAPE ?', ["%{$term}%", '\\']);
                $q->orWhereRaw('content LIKE ? ESCAPE ?', ["%{$term}%", '\\']);
            });
        }

        if ($categoryId = $request->query('category_id')) {
            $query->where('category_id', (int) $categoryId);
        }

        if (! $isEmployee && ($status = $request->query('status'))) {
            $query->where('status', $status);
        }

        if ($authorId = $request->query('author_id')) {
            $query->where('author_id', (int) $authorId);
        }

        return $this->applySorting($query, $request, [
            'created_at',
            'title',
            'view_count',
        ])->paginate($this->getPerPage($request));
    }

    /**
     * Menampilkan detail artikel berdasarkan ID
     * dengan increment view count jika status adalah published
     * serta mengambil artikel terkait berdasarkan kategori.
     */
    public function show(KnowledgeArticle $article): ArticleResource
    {
        $statusValue =
            $article->status instanceof ArticleStatus
                ? $article->status->value
                : $article->status;
        if ($statusValue === ArticleStatus::Published->value) {
            KnowledgeArticle::withoutTimestamps(
                fn () => $article->increment('view_count'),
            );
        }

        $related = KnowledgeArticle::query()
            ->where('category_id', $article->category_id)
            ->where('id', '!=', $article->id)
            ->where('status', ArticleStatus::Published->value)
            ->orderByDesc('view_count')
            ->orderByDesc('published_at')
            ->take(5)
            ->get(['id', 'title', 'slug', 'view_count']);

        return new ArticleResource(
            $article->load(['category', 'author']),
        )->additional(['related_articles' => $related]);
    }
}
