<?php

use App\Http\Controllers\Ticket\TicketController;
use Illuminate\Support\Facades\Route;

// Ticket endpoint
Route::get("/", [TicketController::class, "index"])
    ->middleware("throttle:search")
    ->name("index");

Route::apiResource("", TicketController::class)->except(["index"]);

Route::prefix("{ticket}")->group(function () {
    // Ticket transition
    Route::post("/transition", [TicketController::class, "transition"])->name(
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
