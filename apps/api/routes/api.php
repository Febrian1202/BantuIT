<?php

use App\Http\Controllers\Auth\AuthController;
use App\Http\Controllers\HealthController;
use Illuminate\Support\Facades\Route;

Route::get('/health', HealthController::class);

Route::post('/login', [AuthController::class, 'login'])
    ->middleware('auth:sanctum')
    ->name('auth.login');

Route::post('/logout', [AuthController::class, 'logout'])
    ->middleware('auth:sanctum')
    ->name('auth.logout');
