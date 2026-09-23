<?php

namespace App\Models;

use App\Enums\ArticleStatus;
use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;

class KnowledgeArticle extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi (fillable)
     */
    protected $fillable = [
        "category_id",
        "author_id",
        "title",
        "slug",
        "content",
        "status",
        "view_count",
        "published_at",
    ];

    /**
     * Casting atribut ke tipe data yang sesuai
     */
    protected function casts(): array
    {
        return [
            "status" => ArticleStatus::class,
            "view_count" => "integer",
            "published_at" => "datetime",
        ];
    }

    /**
     * Relasi ke kategori artikel pengetahuan
     */
    public function category(): BelongsTo
    {
        return $this->belongsTo(KnowledgeCategory::class, "category_id");
    }

    /**
     * Relasi ke penulis artikel pengetahuan (users)
     */
    public function author(): BelongsTo
    {
        return $this->belongsTo(User::class, "author_id");
    }
}
