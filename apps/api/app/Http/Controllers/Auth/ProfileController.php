<?php

namespace App\Http\Controllers\Auth;

use App\DTOs\Auth\ChangePasswordData;
use App\DTOs\Auth\UpdateProfileData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Auth\ChangePasswordRequest;
use App\Http\Requests\Auth\UpdateProfileRequest;
use App\Http\Resources\Auth\UserResource;
use App\Services\Auth\ProfileService;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class ProfileController extends Controller
{
    public function __construct(
        private readonly ProfileService $profileService,
    ) {}

    /**
     * Mengambil profil pengguna
     */
    public function show(Request $request): JsonResponse
    {
        $profile = $this->profileService->show($request->user());

        $payload = new UserResource($profile['user'])->resolve();
        $payload['permissions'] = $profile['permissions'];

        return ApiResponse::success(
            $payload,
            'Profile retrieved successfully.',
        );
    }

    /**
     * Memperbarui profil pengguna
     */
    public function update(UpdateProfileRequest $request): JsonResponse
    {
        $user = $this->profileService->update(
            $request->user(),
            UpdateProfileData::fromArray($request->validated()),
        );

        return ApiResponse::success(
            new UserResource($user),
            'Profile updated successfully.',
        );
    }

    /**
     * Memperbarui password pengguna
     */
    public function updatePassword(ChangePasswordRequest $request): JsonResponse
    {
        $this->profileService->updatePassword(
            $request->user(),
            ChangePasswordData::fromArray($request->validated()),
        );

        return ApiResponse::success(null, 'Password updated successfully.');
    }
}
