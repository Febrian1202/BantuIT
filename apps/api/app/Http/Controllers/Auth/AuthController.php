<?php

namespace App\Http\Controllers\Auth;

use App\DTOs\Auth\LoginData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Auth\LoginRequest;
use App\Http\Resources\Auth\UserResource;
use App\Services\Auth\AuthServices;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class AuthController extends Controller
{
    public function __construct(private readonly AuthServices $authService) {}

    /**
     * Login user dan mengembalikan token dan data user
     */
    public function login(LoginRequest $request): JsonResponse
    {
        $result = $this->authService->login(
            LoginData::fromArray($request->validated()),
        );

        return ApiResponse::success(
            [
                "token" => $result["token"],
                "user" => new UserResource($result["user"]),
            ],
            "Login successful.",
        );
    }

    /**
     * Logout user dan menghapus token
     */
    public function logout(Request $request): JsonResponse
    {
        $this->authService->logout($request->user());

        return ApiResponse::success(null, "Logout successful.");
    }
}
