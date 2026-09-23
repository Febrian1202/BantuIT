<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;

class TicketStatus extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi (fillable)
     */
    protected $fillable = ["name", "description", "is_closed", "is_final"];

    /**
     * Konversi tipe data atribut (casting)
     */
    protected function casts(): array
    {
        return [
            "is_closed" => "boolean",
            "is_final" => "boolean",
        ];
    }

    /**
     * Relasi dengan model Ticket
     */
    public function tickets(): HasMany
    {
        return $this->hasMany(Ticket::class, "status_id");
    }
}
