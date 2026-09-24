<?php

namespace App\Services\Auth;

use App\DTOs\Auth\ChangePasswordData;
use App\DTOs\Auth\UpdateProfileData;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Hash;
use Illuminate\Validation\ValidationException;

class ProfileService
{
    public function show(User $user): array
    {
        return [
            "user" => $user,
        ];
    }

    /**
     * Update profil user (hanya nama dan nomor telepon)
     */
    public function update(User $user, UpdateProfileData $data): User
    {
        $user->update(["full_name" => $data->fullName]);

        if ($data->phone !== null) {
            $user->employeeProfile()->update(["phone" => $data->phone]);
        }

        return $user->load(["role", "department", "employeeProfile"]);
    }

    /**
     * Mengganti password user dan revoke semua token selain
     * token dari request saat ini (log out other devices).
     */
    public function updatePassword(User $user, ChangePasswordData $data): void
    {
        if (!Hash::check($data->currentPassword, $user->password)) {
            throw ValidationException::withMessages([
                "current_password" => ["Password yang Anda masukkan salah"],
            ]);
        }

        DB::transaction(function () use ($user, $data): void {
            $user
                ->forceFill([
                    "password" => $data->password,
                    "must_change_password" => false,
                ])
                ->save();

            $user
                ->tokens()
                ->where("id", "!=", $user->currentAccessToken()->id)
                ->delete();
        });
    }
}
