<?php

namespace App\Services\Admin;

use App\DTOs\Department\CreateDepartmentData;
use App\DTOs\Department\UpdateDepartmentData;
use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Models\Department;
use App\Models\User;
use App\Services\Audit\AuditLogger;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Support\Facades\DB;

class DepartmentService
{
    public function __construct(
        private AuditLogger $auditLogger,
        private ReferentialIntegrityGuard $guard,
    ) {}

    /**
     * Mengambil daftar semua departemen.
     */
    public function index(): Collection
    {
        return Department::orderBy('name')->get();
    }

    /**
     * Membuat departemen baru.
     */
    public function store(CreateDepartmentData $data, User $actor): Department
    {
        return DB::transaction(function () use ($data, $actor): Department {
            $department = Department::create($data->toArray());

            $this->auditLogger->log(
                actor: $actor,
                action: AuditAction::Create,
                module: AuditModule::Department,
                moduleId: $department->id,
                description: "Departemen {$department->name} dibuat.",
                oldData: null,
                newData: $department->only(['name', 'description']),
            );

            return $department;
        });
    }

    /**
     * Memperbarui data departemen.
     */
    public function update(
        UpdateDepartmentData $data,
        Department $department,
        User $actor,
    ): Department {
        $oldData = $department->only(['name', 'description']);

        return DB::transaction(function () use (
            $data,
            $department,
            $actor,
            $oldData,
        ): Department {
            $department->update($data->toArray());

            $this->auditLogger->log(
                actor: $actor,
                action: AuditAction::Update,
                module: AuditModule::Department,
                moduleId: $department->id,
                description: "Departemen {$department->name} diperbarui.",
                oldData: $oldData,
                newData: $department->only(['name', 'description']),
            );

            return $department;
        });
    }

    /**
     * Hapus departemen dan log aksi.
     */
    public function destroy(Department $department, User $actor): void
    {
        $this->guard->assertUnreferenced([
            'Departemen' => User::where('department_id', $department->id),
        ]);

        DB::transaction(function () use ($department, $actor): void {
            $deptName = $department->name;
            $deptId = $department->id;

            $department->delete();

            $this->auditLogger->log(
                actor: $actor,
                action: AuditAction::Delete,
                module: AuditModule::Department,
                moduleId: $deptId,
                description: "Departemen {$deptName} dihapus.",
            );
        });
    }
}
