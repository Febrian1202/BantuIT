<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;

class TicketCategory extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi (fillable) untuk model ini.
     */
    protected $fillable = ["name", "description", "parent_id"];

    /**
     * Relasi ke kategori induk (parent).
     */
    public function parent(): BelongsTo
    {
        return $this->belongsTo(self::class, "parent_id");
    }

    /**
     * Relasi ke kategori anak (children).
     */
    public function children(): HasMany
    {
        return $this->hasMany(self::class, "parent_id");
    }

    /**
     * Relasi ke tiket (tickets).
     */
    public function tickets(): HasMany
    {
        return $this->hasMany(Ticket::class, "category_id");
    }
}
