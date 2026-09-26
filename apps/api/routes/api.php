<?php

use App\Http\Controllers\Auth\AuthController;
use App\Http\Controllers\Auth\ProfileController;
use App\Http\Controllers\HealthController;
use Illuminate\Support\Facades\Route;

Route::get("/health", HealthController::class);

Route::post("/login", [AuthController::class, "login"])
    ->middleware("throttle:login")
    ->name("auth.login");

// Auth Routes sanctum
Route::middleware("auth:sanctum")->group(function () {
    Route::post("/logout", [AuthController::class, "logout"])->name(
        "auth.logout",
    );
    Route::put("/me/password", [ProfileController::class, "updatePassword"])
        ->middleware("auth:sanctum")
        ->name("me.password.update");

    // Terauthentikasi dan flag must_change_password false
    Route::middleware("password.changed")->group(function () {
        // Profile endpoints
        Route::get("/me", [ProfileController::class, "show"])->name("me.show");
        Route::put("/me", [ProfileController::class, "update"])->name(
            "me.update",
        );

        // Ticket module
        Route::prefix("/tickets")
            ->name("tickets.")
            ->group(base_path("routes/Group/Ticket.php"));

        // Asset module
        Route::prefix("/assets")
            ->name("assets.")
            ->group(base_path("routes/Group/Asset.php"));
    });
});
