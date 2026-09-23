<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;

class AssetAssignment extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi secara massal
     *
     * @var list<string>
     */
    protected $fillable = [
        "asset_id",
        "user_id",
        "assigned_at",
        "released_at",
        "notes",
    ];

    /**
     * Mengonversi atribut
     *
     * @return array<string, string>
     */
    protected function casts(): array
    {
        return [
            "assigned_at" => "datetime",
            "released_at" => "datetime",
        ];
    }

    /**
     * Mengambil data yang aktif (tidak di-release).
     */
    public function scopeActive(Builder $query): Builder
    {
        return $query->whereNull("released_at");
    }

    /**
     * Relasi ke model Asset.
     */
    public function asset(): BelongsTo
    {
        return $this->belongsTo(Asset::class);
    }

    /**
     * Relasi ke model User.
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }
}
