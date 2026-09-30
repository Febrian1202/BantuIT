<?php

use App\Http\Controllers\Article\ArticleController;
use Illuminate\Support\Facades\Route;

Route::get("/", [ArticleController::class, "index"])
    ->middleware("throttle:search")
    ->name("index");

Route::post("/", [ArticleController::class, "store"])->name("store");

Route::get("/{article}/edit", [ArticleController::class, "edit"])->name("edit");

Route::get("/{article:slug}", [ArticleController::class, "show"])->name("show");

Route::put("/{article}", [ArticleController::class, "update"])->name("update");

Route::delete("/{article}", [ArticleController::class, "destroy"])->name(
    "destroy",
);

Route::post("/{article}/publish", [ArticleController::class, "publish"])->name(
    "publish",
);
Route::post("/{article}/unpublish", [
    ArticleController::class,
    "unpublish",
])->name("unpublish");
