<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;

class Notification extends Model
{
    use HasFactory, SerializesDatesAsIso8601;

    /**
     * Atribut yang dapat diisi (fillable)
     */
    protected $fillable = ["user_id", "type", "data", "is_read", "read_at"];

    /**
     * Casting atribut ke tipe data yang sesuai
     */
    protected function casts(): array
    {
        return [
            "data" => "array",
            "is_read" => "boolean",
            "read_at" => "datetime",
        ];
    }

    /**
     * Relasi ke model User
     */
    public function user(): BelongsTo
    {
        return $this->belongsTo(User::class);
    }
}
