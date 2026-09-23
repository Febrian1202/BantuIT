<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\SoftDeletes;

class EmployeeProfile extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi secara massal (fillable)
     */
    protected $fillable = [
        "user_id",
        "employee_code",
        "phone",
        "position",
        "hire_date",
    ];

    /**
     * Konversi tipe data atribut (casts)
     */
    protected function casts(): array
    {
        return [
            "hire_date" => "date",
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
