<?php

namespace App\Http\Controllers\Admin;

use App\DTOs\Department\CreateDepartmentData;
use App\DTOs\Department\UpdateDepartmentData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Department\StoreDepartmentRequest;
use App\Http\Requests\Department\UpdateDepartmentRequest;
use App\Models\Department;
use App\Services\Admin\DepartmentService;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class DepartmentController extends Controller
{
    public function __construct(private DepartmentService $departmentService) {}

    /**
     * Menampilkan daftar departemen
     */
    public function index(): JsonResponse
    {
        $this->authorize('department.viewAny');

        $departments = $this->departmentService->index();

        return ApiResponse::success($departments, 'Departments retrieved.');
    }

    /**
     * Menampilkan detail departemen
     */
    public function show(Department $department): JsonResponse
    {
        $this->authorize('department.viewAny');

        return ApiResponse::success($department, 'Department retrieved.');
    }

    /**
     * Membuat departemen baru
     */
    public function store(StoreDepartmentRequest $request): JsonResponse
    {
        $this->authorize('department.manage');

        $dto = CreateDepartmentData::fromArray($request->validated());
        $department = $this->departmentService->store($dto, $request->user());

        return ApiResponse::created(
            $department,
            'Department created successfully.',
        );
    }

    /**
     * Mengupdate departemen yang sudah ada
     */
    public function update(
        UpdateDepartmentRequest $request,
        Department $department,
    ): JsonResponse {
        $this->authorize('department.manage');

        $dto = UpdateDepartmentData::fromArray($request->validated());
        $department = $this->departmentService->update(
            $dto,
            $department,
            $request->user(),
        );

        return ApiResponse::success(
            $department,
            'Department updated successfully.',
        );
    }

    public function destroy(
        Department $department,
        Request $request,
    ): JsonResponse {
        $this->authorize('department.manage');

        $this->departmentService->destroy($department, $request->user());

        return ApiResponse::success(null, 'Department deleted successfully.');
    }
}
