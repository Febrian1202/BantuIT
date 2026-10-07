<?php

use App\Http\Controllers\Export\ExportController;
use Illuminate\Support\Facades\Route;

Route::get("/tickets", [ExportController::class, "tickets"])->name("tickets");
Route::get("/assets", [ExportController::class, "assets"])->name("assets");
Route::get("/audit-logs", [ExportController::class, "auditLogs"])->name(
    "audit-logs",
);
