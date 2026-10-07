<?php

use App\Http\Controllers\Admin\DepartmentController;
use App\Http\Controllers\Admin\TicketCategoryController;
use App\Http\Controllers\Admin\TicketPriorityController;
use App\Http\Controllers\Admin\UserController;
use App\Http\Controllers\Article\KnowledgeCategoryController;
use App\Http\Controllers\Asset\AssetController;
use App\Http\Controllers\Auth\AuthController;
use App\Http\Controllers\Auth\ProfileController;
use App\Http\Controllers\HealthController;
use App\Http\Controllers\ReferenceController;
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

        // User module
        Route::get("/roles", [UserController::class, "roles"])->name(
            "roles.index",
        );
        Route::prefix("/users")
            ->name("users.")
            ->group(base_path("routes/Group/User.php"));

        // Attachment endpoint
        Route::prefix("/attachments")
            ->name("attachments.")
            ->group(base_path("routes/Group/Attachment.php"));

        // Department module
        Route::apiResource("departments", DepartmentController::class);

        // Ticket module
        Route::prefix("/tickets")
            ->name("tickets.")
            ->group(base_path("routes/Group/Ticket.php"));

        // Asset module
        Route::prefix("/assets")
            ->name("assets.")
            ->group(base_path("routes/Group/Asset.php"));

        Route::get("/my-assets", [AssetController::class, "myAssets"])->name(
            "assets.my-assets",
        );

        // Dashboard module
        Route::prefix("/dashboard")
            ->name("dashboard.")
            ->group(base_path("routes/Group/Dashboard.php"));

        // Notification module
        Route::prefix("/notifications")
            ->name("notifications.")
            ->group(base_path("routes/Group/Notification.php"));

        // Ticket category, priority, statuses, dan technicians
        Route::apiResource(
            "/ticket-categories",
            TicketCategoryController::class,
        );
        Route::apiResource(
            "/ticket-priorities",
            TicketPriorityController::class,
        );

        Route::get("/ticket-statuses", [
            ReferenceController::class,
            "statuses",
        ])->name("ticket-statuses.index");

        Route::get("/technicians", [
            ReferenceController::class,
            "technicians",
        ])->name("technicians.index");

        // Artikel module
        Route::prefix("/articles")
            ->name("articles.")
            ->group(base_path("routes/Group/Knowledge.php"));

        Route::apiResource(
            "/knowledge-categories",
            KnowledgeCategoryController::class,
        );

        // Export module
        Route::prefix("/export")
            ->name("export.")
            ->middleware("throttle:export")
            ->group(base_path("routes/Group/Export.php"));

        // Audit Log module
        Route::prefix("/audit-logs")
            ->name("audit-logs.")
            ->group(base_path("routes/Group/Audit.php"));
    });
});
