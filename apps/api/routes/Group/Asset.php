<?php

use App\Http\Controllers\Asset\AssetController;
use Illuminate\Support\Facades\Route;

Route::get('/assignable', [AssetController::class, 'assignable'])->name(
    '.assignable',
);
