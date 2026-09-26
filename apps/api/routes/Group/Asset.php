<?php

use App\Http\Controllers\Asset\AssetController;
use App\Http\Controllers\Asset\AssetHistoryController;
use Illuminate\Support\Facades\Route;

Route::get('/categories', [AssetController::class, 'categories'])->name(
    'categories',
);
Route::get('/assignable', [AssetController::class, 'assignable'])->name(
    'assignable',
);

// CRUD
Route::get('/', [AssetController::class, 'index'])
    ->middleware('throttle:search')
    ->name('index');
Route::post('/', [AssetController::class, 'store'])->name('store');
Route::get('/{asset}', [AssetController::class, 'show'])->name('show');
Route::put('/{asset}', [AssetController::class, 'update'])->name('update');
Route::delete('/{asset}', [AssetController::class, 'destroy'])->name('destroy');

Route::post('/{asset}/assign', [AssetController::class, 'assign'])->name(
    'assign',
);
Route::post('/{asset}/release', [AssetController::class, 'release'])->name(
    'release',
);

Route::get('/{asset}/history', AssetHistoryController::class)->name('history');
