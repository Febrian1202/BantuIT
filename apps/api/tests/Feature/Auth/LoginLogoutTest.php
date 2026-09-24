<?php

use App\Models\User;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Illuminate\Support\Facades\Hash;

uses(RefreshDatabase::class);

test("User dapat login dengan kredensial yang valid", function () {
    User::factory()
        ->employee()
        ->create([
            "email" => "test@bantuit.test",
            "password" => Hash::make("Password123!"),
        ]);

    $response = $this->postJson("/api/login", [
        "email" => "test@bantuit.test",
        "password" => "Password123!",
    ]);

    $response
        ->assertStatus(200)
        ->assertJsonPath("success", true)
        ->assertJsonPath("message", "Login successful.")
        ->assertJsonStructure([
            "success",
            "message",
            "data" => [
                "token",
                "user" => [
                    "id",
                    "email",
                    "full_name",
                    "status",
                    "role",
                    "department",
                ],
            ],
        ]);

    expect($response->json("data.token"))->toBeString()->not->toBeEmpty();
    expect(str_contains($response->json("data.token"), "|"))->toBeTrue();
});

test("login gagal dengan password yang salah", function () {
    User::factory()
        ->employee()
        ->create([
            "email" => "test@bantuit.test",
            "password" => Hash::make("Password123!"),
        ]);

    $this->postJson("/api/login", [
        "email" => "test@bantuit.test",
        "password" => "wrong-password",
    ])
        ->assertStatus(422)
        ->assertJsonPath("errors.email", [
            "These credentials do not match our records.",
        ]);
});

test("login gagal untuk email yang tidak ada", function () {
    $this->postJson("/api/login", [
        "email" => "nonexistent@bantuit.test",
        "password" => "Password123!",
    ])
        ->assertStatus(422)
        ->assertJsonPath("errors.email", [
            "These credentials do not match our records.",
        ]);
});

test("login gagal untuk akun yang tidak aktif", function () {
    User::factory()
        ->employee()
        ->inactive()
        ->create([
            "email" => "inactive@bantuit.test",
            "password" => Hash::make("Password123!"),
        ]);

    $this->postJson("/api/login", [
        "email" => "inactive@bantuit.test",
        "password" => "Password123!",
    ])
        ->assertStatus(422)
        ->assertJsonPath("errors.email", ["This account is inactive."]);
});

test("login gagal untuk eror validasi ketika field kosong", function () {
    $this->postJson("/api/login", [
        "email" => "not-an-email",
    ])->assertStatus(422);
});

test("user yang sudah login dapat logout", function () {
    $user = User::factory()->employee()->create();
    $token = $user->createToken("auth_token", ["*"])->plainTextToken;

    $this->withHeaders(["Authorization" => "Bearer " . $token])
        ->postJson("/api/logout")
        ->assertStatus(200)
        ->assertJsonPath("success", true)
        ->assertJsonPath("data", null);

    expect($user->tokens()->count())->toBe(0);
});

test("user yang belum login tidak dapat logout", function () {
    $this->postJson("/api/logout")->assertStatus(401);
});

test("token yang sudah expired tidak dapat digunakan", function () {
    $user = User::factory()->employee()->create();
    $token = $user->createToken("auth_token", ["*"])->plainTextToken;

    $this->travelTo(now()->addHours(12)->addMinute());

    $this->withHeaders(["Authorization" => "Bearer " . $token])
        ->getJson("/api/me")
        ->assertStatus(401)
        ->assertJsonPath("message", "Unauthenticated.");
});
