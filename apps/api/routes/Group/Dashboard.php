<?php

use App\Http\Controllers\Dashboard\DashboardController;
use Illuminate\Support\Facades\Route;

Route::get("/employee", [DashboardController::class, "employee"])->name(
    "employee",
);
Route::get("/technician", [DashboardController::class, "technician"])->name(
    "technician",
);
Route::get("/manager", [DashboardController::class, "manager"])->name(
    "manager",
);
Route::get("/admin", [DashboardController::class, "admin"])->name("admin");
