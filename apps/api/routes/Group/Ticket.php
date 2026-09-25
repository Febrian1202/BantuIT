<?php

use App\Http\Controllers\Ticket\TicketController;
use Illuminate\Support\Facades\Route;

// Ticket endpoint
Route::get("/", [TicketController::class, "index"])
    ->middleware("throttle:search")
    ->name("index");
Route::post("/", [TicketController::class, "store"])->name("store");
Route::get("/{ticket}", [TicketController::class, "show"])->name("show");
Route::put("/{ticket}", [TicketController::class, "update"])->name("update");
Route::delete("/{ticket}", [TicketController::class, "destroy"])->name(
    "destroy",
);

Route::prefix("{ticket}")->group(function () {
    // Ticket transition
    Route::post("/status", [TicketController::class, "transition"])->name(
        "status",
    );
    Route::post("/assign", [TicketController::class, "assign"])->name("assign");
    Route::post("/unassign", [TicketController::class, "unassign"])->name(
        "unassign",
    );
    Route::post("/priority", [TicketController::class, "changePriority"])->name(
        "priority",
    );
});
