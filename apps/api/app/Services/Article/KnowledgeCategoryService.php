<?php

namespace App\Services\Article;

use App\DTOs\Article\CreateKnowledgeCategoryData;
use App\DTOs\Article\UpdateKnowledgeCategoryData;
use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Exceptions\StateConflictException;
use App\Models\KnowledgeCategory;
use App\Models\User;
use App\Services\Audit\AuditLogger;
use Illuminate\Database\Eloquent\Collection;

class KnowledgeCategoryService
{
    public function __construct(private AuditLogger $auditLogger) {}

    /**
     * Mengambil semua kategori artikel dengan jumlah artikel terkait
     */
    public function index(): Collection
    {
        return KnowledgeCategory::query()
            ->withCount('articles')
            ->orderBy('name')
            ->get();
    }

    /**
     * Mengambil kategori artikel dengan jumlah artikel terkait
     */
    public function show(KnowledgeCategory $category): KnowledgeCategory
    {
        return $category->loadCount('articles');
    }

    /**
     * Membuat kategori artikel baru
     */
    public function store(
        CreateKnowledgeCategoryData $data,
        User $actor,
    ): KnowledgeCategory {
        $category = KnowledgeCategory::create($data->toArray());

        $this->auditLogger->log(
            $actor,
            AuditAction::Create,
            AuditModule::KnowledgeCategory,
            $category->id,
            "Kategori artikel {$category->name} dibuat",
            null,
            $category->only(['name', 'description']),
        );

        return $category;
    }

    /**
     * Memperbarui kategori artikel
     */
    public function update(
        KnowledgeCategory $category,
        UpdateKnowledgeCategoryData $data,
        User $actor,
    ): KnowledgeCategory {
        $oldData = $category->only(['name', 'description']);

        $category->update($data->toArray());

        $this->auditLogger->log(
            $actor,
            AuditAction::Update,
            AuditModule::KnowledgeCategory,
            $category->id,
            "Kategori artikel {$category->name} diperbarui",
            $oldData,
            $category->only(['name', 'description']),
        );

        return $category->fresh();
    }

    /**
     * Menghapus kategori artikel
     *
     * @throws StateConflictException Jika kategori artikel memiliki artikel terkait
     */
    public function destroy(KnowledgeCategory $category, User $actor): void
    {
        if ($category->articles()->exists()) {
            throw new StateConflictException(
                'Kategori masih memiliki artikel dan tidak dapat dihapus.',
            );
        }

        $categoryName = $category->name;
        $categoryId = $category->id;

        $category->delete();

        $this->auditLogger->log(
            $actor,
            AuditAction::Delete,
            AuditModule::KnowledgeCategory,
            $categoryId,
            "Kategori artikel {$categoryName} dihapus",
        );
    }
}
