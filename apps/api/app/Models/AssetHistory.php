<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Database\Factories\AssetHistoryFactory;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class AssetHistory extends Model
{
    /** @use HasFactory<AssetHistoryFactory> */
    use HasFactory, SerializesDatesAsIso8601;

    // Menonaktifkan kolom updated_at
    public const UPDATED_AT = null;

    protected $fillable = ['asset_id', 'action', 'description', 'action_at'];

    /**
     * Cast/konversi kolom action_at menjadi datetime.
     */
    protected function casts(): array
    {
        return [
            'action_at' => 'datetime',
        ];
    }

    /**
     * Relasi ke model Asset.
     */
    public function asset(): BelongsTo
    {
        return $this->belongsTo(Asset::class);
    }
}
