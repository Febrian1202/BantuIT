<?php

namespace App\Services\Auth;

use App\Authorization\AbilityMatrix;
use App\DTOs\Auth\ChangePasswordData;
use App\DTOs\Auth\UpdateProfileData;
use App\Enums\RoleName;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Illuminate\Support\Facades\Hash;
use Illuminate\Validation\ValidationException;

class ProfileService
{
    public function show(User $user): array
    {
        $user->load(["role", "department", "employeeProfile"]);

        $role = RoleName::tryFrom($user->role?->name ?? "");
        $permissions = [];

        if ($role) {
            $permissions = AbilityMatrix::permissionsFor($role);
            $policyAbilities = AbilityMatrix::getPolicyAbilities();

            foreach ($policyAbilities as $ability) {
                // Untuk admin, semua abilities policy diizinkan
                if ($role === RoleName::Admin) {
                    $permissions[] = $ability;

                    continue;
                }

                // Untuk role lainnya, cek apakah ability policy diizinkan
                if ($this->roleCanPerformPolicyAbility($role, $ability)) {
                    $permissions[] = $ability;
                }
            }
        }

        return [
            "user" => $user,
            "permissions" => array_values(array_unique($permissions)),
        ];
    }

    /**
     * Mappipng policy abilities ke masing-masing role.
     */
    private function roleCanPerformPolicyAbility(
        RoleName $role,
        string $ability,
    ): bool {
        return match ($role) {
            RoleName::Admin => true,
            RoleName::Manager => in_array(
                $ability,
                [
                    // Ticket (12 of 13, kecuali selfAssign)
                    "ticket.viewAny",
                    "ticket.view",
                    "ticket.create",
                    "ticket.update",
                    "ticket.assign",
                    "ticket.unassign",
                    "ticket.changeStatus",
                    "ticket.changePriority",
                    "ticket.comment",
                    "ticket.viewHistory",
                    "ticket.attach",
                    // Attachment (4)
                    "attachment.view",
                    "attachment.download",
                    "attachment.create",
                    "attachment.delete",
                    // Asset (9 of 10)
                    "asset.viewAny",
                    "asset.view",
                    "asset.viewOwn",
                    "asset.viewAssignable",
                    "asset.create",
                    "asset.update",
                    "asset.delete",
                    "asset.assign",
                    "asset.release",
                    "asset.viewHistory",
                    // Article (7)
                    "article.viewAny",
                    "article.view",
                    "article.create",
                    "article.update",
                    "article.publish",
                    "article.unpublish",
                    "article.delete",
                    // Notification (3)
                    "notification.viewAny",
                    "notification.markAsRead",
                    "notification.markAllAsRead",
                ],
                true,
            ),
            RoleName::Technician => in_array(
                $ability,
                [
                    // Ticket (10 of 13)
                    "ticket.viewAny",
                    "ticket.view",
                    "ticket.create",
                    "ticket.update",
                    "ticket.changeStatus",
                    "ticket.selfAssign",
                    "ticket.changePriority",
                    "ticket.comment",
                    "ticket.viewHistory",
                    "ticket.attach",
                    // Attachment (4)
                    "attachment.view",
                    "attachment.download",
                    "attachment.create",
                    "attachment.delete",
                    // Asset (8 of 10)
                    "asset.viewAny",
                    "asset.view",
                    "asset.viewOwn",
                    "asset.viewAssignable",
                    "asset.create",
                    "asset.update",
                    "asset.assign",
                    "asset.release",
                    "asset.viewHistory",
                    // Article (7)
                    "article.viewAny",
                    "article.view",
                    "article.create",
                    "article.update",
                    "article.publish",
                    "article.unpublish",
                    "article.delete",
                    // Notification (3)
                    "notification.viewAny",
                    "notification.markAsRead",
                    "notification.markAllAsRead",
                ],
                true,
            ),
            RoleName::Employee => in_array(
                $ability,
                [
                    // Ticket (8 of 13)
                    "ticket.viewAny",
                    "ticket.view",
                    "ticket.create",
                    "ticket.update",
                    "ticket.changeStatus",
                    "ticket.comment",
                    "ticket.viewHistory",
                    "ticket.attach",
                    // Attachment (3)
                    "attachment.view",
                    "attachment.download",
                    "attachment.create",
                    // Asset (2)
                    "asset.viewOwn",
                    "asset.viewAssignable",
                    // Article (2)
                    "article.viewAny",
                    "article.view",
                    // Notification (3)
                    "notification.viewAny",
                    "notification.markAsRead",
                    "notification.markAllAsRead",
                ],
                true,
            ),
        };
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
                "current_password" => ["The current password is incorrect."],
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
