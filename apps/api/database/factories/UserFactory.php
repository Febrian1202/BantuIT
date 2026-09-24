<?php

namespace Database\Factories;

use App\Enums\RoleName;
use App\Models\Department;
use App\Models\Role;
use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;
use Illuminate\Support\Facades\Hash;

class UserFactory extends Factory
{
    protected $model = User::class;

    protected static ?string $password;

    public function definition(): array
    {
        return [
            "role_id" => Role::factory(),
            "department_id" => Department::factory(),
            "full_name" => fake()->name(),
            "email" => fake()->unique()->safeEmail(),
            "password" => (static::$password ??= Hash::make("Password123!")),
            "status" => "active",
            "must_change_password" => false,
            "last_login_at" => null,
        ];
    }

    public function admin(): static
    {
        return $this->state([
            "role_id" => Role::firstOrCreate(
                ["name" => RoleName::Admin->value],
                ["description" => "Administrator"],
            )->id,
        ]);
    }

    public function manager(): static
    {
        return $this->state([
            "role_id" => Role::firstOrCreate(
                ["name" => RoleName::Manager->value],
                ["description" => "Manager"],
            )->id,
        ]);
    }

    public function technician(): static
    {
        return $this->state([
            "role_id" => Role::firstOrCreate(
                ["name" => RoleName::Technician->value],
                ["description" => "Technician"],
            )->id,
        ]);
    }

    public function employee(): static
    {
        return $this->state([
            "role_id" => Role::firstOrCreate(
                ["name" => RoleName::Employee->value],
                ["description" => "Employee"],
            )->id,
        ]);
    }

    public function inactive(): static
    {
        return $this->state([
            "status" => "inactive",
        ]);
    }
}
