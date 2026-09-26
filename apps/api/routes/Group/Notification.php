<?php

use App\Http\Controllers\Notification\NotificationController;
use Illuminate\Support\Facades\Route;

Route::get("/", [NotificationController::class, "index"])->name("index");
Route::get("/unread-count", [
    NotificationController::class,
    "unreadCount",
])->name("unread-count");
Route::post("/{notification}/read", [
    NotificationController::class,
    "read",
])->name("read");
Route::post("/read-all", [NotificationController::class, "readAll"])->name(
    "read-all",
);
