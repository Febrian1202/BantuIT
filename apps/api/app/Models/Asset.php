<?php

namespace App\Models;

use App\Enums\AssetStatus;
use App\Models\Concerns\SerializesDatesAsIso8601;
use App\Policies\AssetPolicy;
use Illuminate\Database\Eloquent\Attributes\UsePolicy;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\SoftDeletes;

#[UsePolicy(AssetPolicy::class)]
class Asset extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi secara massal
     *
     * @var list<string>
     */
    protected $fillable = [
        'asset_tag',
        'name',
        'category',
        'brand',
        'model',
        'serial_number',
        'purchase_date',
        'status',
        'notes',
    ];

    /**
     * Mendapatkan atribut yang dikonversi
     *
     * @return array<string, string>
     */
    protected function casts(): array
    {
        return [
            'purchase_date' => 'date',
            'status' => AssetStatus::class,
        ];
    }

    /**
     * Mendapatkan assignment yang aktif
     */
    public function activeAssignment(): HasOne
    {
        return $this->hasOne(AssetAssignment::class)->whereNull('released_at');
    }

    /**
     * Mendapatkan holder yang sedang memegang asset ini
     */
    public function currentHolder(): BelongsTo
    {
        return $this->belongsTo(User::class, 'id', 'id')
            ->join(
                'asset_assignments',
                'asset_assignments.user_id',
                '=',
                'users.id',
            )
            ->where('asset_assignments.asset_id', $this->id)
            ->whereNull('asset_assignments.released_at');
    }

    /**
     * Mendapatkan semua assignment yang terkait dengan asset ini
     */
    public function assignments(): HasMany
    {
        return $this->hasMany(AssetAssignment::class);
    }

    /**
     * Mendapatkan history yang terkait dengan asset ini
     */
    public function histories(): HasMany
    {
        return $this->hasMany(AssetHistory::class);
    }
}
