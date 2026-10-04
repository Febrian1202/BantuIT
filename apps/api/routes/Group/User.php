<?php

use App\Http\Controllers\Admin\UserController;
use Illuminate\Support\Facades\Route;

Route::get('/assignable', [UserController::class, 'assignable'])->name(
    'assignable',
);
Route::post('/{user}/activate', [UserController::class, 'activate'])->name(
    'activate',
);
Route::post('/{user}/deactivate', [UserController::class, 'deactivate'])->name(
    'deactivate',
);
Route::post('/{user}/reset-password', [
    UserController::class,
    'resetPassword',
])->name('reset-password');

Route::get('/', [UserController::class, 'index'])->name('index');
Route::post('/', [UserController::class, 'store'])->name('store');
Route::get('/{user}', [UserController::class, 'show'])->name('show');
Route::put('/{user}', [UserController::class, 'update'])->name('update');
Route::delete('/{user}', [UserController::class, 'destroy'])->name('destroy');
