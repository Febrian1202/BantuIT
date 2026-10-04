<?php

namespace App\Services\Audit;

use App\Enums\AuditModule;
use App\Enums\RoleName;
use App\Models\AuditLog;
use App\Models\User;
use Carbon\Carbon;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Pagination\LengthAwarePaginator;

class AuditLogQueryService
{
    public const MANAGER_ALLOWED_MODULES = [
        AuditModule::Ticket->value,
        AuditModule::Asset->value,
        AuditModule::Article->value,
    ];

    /**
     * Mengambil data log audit dengan filter dan pagination.
     */
    public function paginate(
        User $actor,
        array $filters = [],
        ?Carbon $dateFrom = null,
        ?Carbon $dateTo = null,
        int $perPage = 10,
        string $sortBy = 'created_at',
        string $sortDir = 'desc',
    ): LengthAwarePaginator {
        $query = AuditLog::query()->with('user');

        $this->applyRoleScope($query, $actor);

        if (! empty($filters['user_id'])) {
            $query->where('user_id', $filters['user_id']);
        }

        if (! empty($filters['module'])) {
            $query->where('module', $filters['module']);
        }

        if (! empty($filters['action'])) {
            $query->where('action', $filters['action']);
        }

        if (! empty($filters['module_id'])) {
            $query->where('module_id', $filters['module_id']);
        }

        if ($dateFrom !== null) {
            $query->where('created_at', '>=', $dateFrom);
        }

        if ($dateTo !== null) {
            $query->where('created_at', '<=', $dateTo);
        }

        return $query->orderBy($sortBy, $sortDir)->paginate($perPage);
    }

    /**
     * Menampilkan log audit berdasarkan ID dan memastikan pengguna memiliki izin untuk melihatnya.
     */
    public function show(AuditLog $auditLog, User $actor): AuditLog
    {
        if (! $this->isVisibleTo($auditLog, $actor)) {
            abort(404, 'Resource not found.');
        }

        return $auditLog->load('user');
    }

    /**
     * Memeriksa apakah log audit dapat dilihat oleh pengguna.
     */
    public function isVisibleTo(AuditLog $auditLog, User $actor): bool
    {
        if ($actor->isAdmin()) {
            return true;
        }

        if ($actor->hasRole(RoleName::Manager)) {
            return in_array(
                $auditLog->module,
                self::MANAGER_ALLOWED_MODULES,
                true,
            );
        }

        return false;
    }

    /**
     * Menambahkan scope berdasarkan peran pengguna.
     */
    protected function applyRoleScope(Builder $query, User $actor): void
    {
        if ($actor->isAdmin()) {
            return; // Full akses
        }

        if ($actor->hasRole(RoleName::Manager)) {
            $query->whereIn('module', self::MANAGER_ALLOWED_MODULES);

            return;
        }

        // Role selain admin dan manager tidak bisa melihat log
        $query->whereRaw('1 = 0');
    }
}
