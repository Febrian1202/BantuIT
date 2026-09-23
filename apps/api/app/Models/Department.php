<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;

class Department extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi secara massal
     *
     * @var list<string>
     */
    protected $fillable = ["name", "description"];

    /**
     * Relasi model Department ke model User
     */
    public function users(): HasMany
    {
        return $this->hasMany(User::class);
    }
}
