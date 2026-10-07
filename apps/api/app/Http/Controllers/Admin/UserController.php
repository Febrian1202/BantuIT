<?php

namespace App\Http\Controllers\Admin;

use App\DTOs\User\CreateUserData;
use App\DTOs\User\UpdateUserData;
use App\Http\Controllers\Controller;
use App\Http\Requests\User\IndexUserRequest;
use App\Http\Requests\User\StoreUserRequest;
use App\Http\Requests\User\UpdateUserRequest;
use App\Http\Resources\User\UserAdminResource;
use App\Http\Resources\User\UserListResource;
use App\Models\User;
use App\Services\Admin\UserService;
use App\Support\ApiResponse;
use App\Support\HandlesPagination;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class UserController extends Controller
{
    use HandlesPagination;

    public function __construct(protected UserService $userService) {}

    /**
     * Menampilkan daftar user
     */
    public function index(IndexUserRequest $request): JsonResponse
    {
        $this->authorize('user.viewAny');

        $paginator = $this->userService->index($request);

        return ApiResponse::paginated(
            $paginator,
            'Users retrieved successfully',
            UserListResource::class,
        );
    }

    /**
     * Menampilkan daftar user yang dapat ditugaskan
     */
    public function assignable(Request $request): JsonResponse
    {
        $this->authorize('user.lookup');

        $users = $this->userService->assignable($request);

        return ApiResponse::success($users, 'Assignable users retrieved.');
    }

    /**
     * Menampilkan detail user
     */
    public function show(User $user): JsonResponse
    {
        $this->authorize('user.view', $user);

        $user = $this->userService->show($user);

        return ApiResponse::success(
            new UserAdminResource($user),
            'User retrieved.',
        );
    }

    /**
     * Membuat user baru
     */
    public function store(StoreUserRequest $request): JsonResponse
    {
        $this->authorize('user.create');

        $data = CreateUserData::fromArray($request->validated());
        $user = $this->userService->create($data, $request->user());

        return ApiResponse::created(
            new UserAdminResource($user),
            'User created successfully.',
        );
    }

    /**
     * Mengupdate data user
     */
    public function update(UpdateUserRequest $request, User $user): JsonResponse
    {
        $this->authorize('user.update', $user);

        $data = UpdateUserData::fromArray($request->validated());
        $updatedUser = $this->userService->update(
            $user,
            $data,
            $request->user(),
        );

        return ApiResponse::success(
            new UserAdminResource($updatedUser),
            'User updated successfully.',
        );
    }

    /**
     * Menghapus user
     */
    public function destroy(User $user, Request $request): JsonResponse
    {
        $this->authorize('user.delete', $user);

        $this->userService->delete($user, $request->user());

        return ApiResponse::success(null, 'User deleted successfully.');
    }

    /**
     * Mengaktifkan user
     */
    public function activate(User $user, Request $request): JsonResponse
    {
        $this->authorize('user.activate', $user);

        $activeUser = $this->userService->activate($user, $request->user());

        return ApiResponse::success(
            new UserAdminResource($activeUser),
            'User activated successfully.',
        );
    }

    /**
     * Mengubah status user menjadi tidak aktif
     */
    public function deactivate(User $user, Request $request): JsonResponse
    {
        $this->authorize('user.deactivate', $user);

        $deactivatedUser = $this->userService->deactivate(
            $user,
            $request->user(),
        );

        return ApiResponse::success(
            new UserAdminResource($deactivatedUser),
            'User deactivated successfully.',
        );
    }

    /**
     * Reset kata sandi user
     */
    public function resetPassword(User $user, Request $request): JsonResponse
    {
        $this->authorize('user.reset-password', $user);

        $password = $this->userService->resetPassword($user, $request->user());

        return ApiResponse::success(
            ['temporary_password' => $password],
            'Password reset successfully.',
        );
    }

    /**
     * Mengambil daftar peran (roles)
     */
    public function roles(): JsonResponse
    {
        $this->authorize('user.viewAny');

        $roles = $this->userService->roles();

        return ApiResponse::success($roles, 'Roles retrieved.');
    }
}
