<?php

use App\Http\Controllers\Audit\AuditLogController;
use Illuminate\Support\Facades\Route;

Route::get('/', [AuditLogController::class, 'index'])
    ->middleware('throttle:search')
    ->name('index');
Route::get('/{auditLog', [AuditLogController::class, 'show'])->name('show');
