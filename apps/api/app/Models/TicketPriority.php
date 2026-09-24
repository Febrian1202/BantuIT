<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;

class TicketPriority extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi (fillable)
     */
    protected $fillable = ['name', 'level', 'sla_minutes', 'description'];

    /**
     * Casting atribut ke tipe data yang sesuai
     */
    protected function casts(): array
    {
        return [
            'level' => 'integer',
            'sla_minutes' => 'integer',
        ];
    }

    /**
     * Relasi ke model Ticket
     */
    public function tickets(): HasMany
    {
        return $this->hasMany(Ticket::class, 'priority_id');
    }
}
