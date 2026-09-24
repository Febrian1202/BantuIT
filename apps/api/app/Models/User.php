<?php

namespace App\Models;

use App\Enums\RoleName;
use App\Models\Concerns\SerializesDatesAsIso8601;
use App\Observers\UserObserver;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\SoftDeletes;
use Illuminate\Foundation\Auth\User as Authenticatable;
use Laravel\Sanctum\HasApiTokens;
use Laravel\Sanctum\PersonalAccessToken;

/**
 * @method PersonalAccessToken|null currentAccessToken()
 */
#[ObservedBy([UserObserver::class])]
class User extends Authenticatable
{
    use HasApiTokens, HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi secara massal
     */
    protected $fillable = [
        'role_id',
        'department_id',
        'email',
        'password',
        'full_name',
        'status',
        'must_change_password',
        'last_login_at',
    ];

    /**
     * Atribut yang tersembunyi (tidak akan dikembalikan dalam respons API)
     */
    protected $hidden = ['password', 'role_id', 'department_id'];

    /**
     * Atribut yang harus dikonversi ke tipe data tertentu
     */
    protected function casts(): array
    {
        return [
            'password' => 'hashed',
            'must_change_password' => 'boolean',
            'last_login_at' => 'datetime',
        ];
    }

    /**
     * Relasi ke model Role
     */
    public function role(): BelongsTo
    {
        return $this->belongsTo(Role::class);
    }

    /**
     * Relasi ke model Department
     */
    public function department(): BelongsTo
    {
        return $this->belongsTo(Department::class);
    }

    /**
     * Relasi ke model EmployeeProfile
     */
    public function employeeProfile(): HasOne
    {
        return $this->hasOne(EmployeeProfile::class);
    }

    /**
     * Relasi ke model AssetAssignment
     */
    public function assetAssignments(): HasMany
    {
        return $this->hasMany(AssetAssignment::class);
    }

    /**
     * Relasi ke model AssetAssignment yang aktif (belum dilepaskan)
     */
    public function activeAssignments(): HasMany
    {
        return $this->hasMany(AssetAssignment::class)->whereNull('released_at');
    }

    /**
     * Memeriksa apakah pengguna memiliki role tertentu
     */
    public function hasRole(RoleName|string ...$roles): bool
    {
        $roleName = $this->role?->name;

        foreach ($roles as $role) {
            $expected = $role instanceof RoleName ? $role->value : $role;
            if ($roleName === $expected) {
                return true;
            }
        }

        return false;
    }

    /**
     * Memeriksa apakah pengguna adalah admin
     */
    public function isAdmin(): bool
    {
        return $this->hasRole(RoleName::Admin);
    }
}
