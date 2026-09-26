<?php

use App\Http\Controllers\Ticket\TicketCommentController;
use App\Http\Controllers\Ticket\TicketController;
use App\Http\Controllers\Ticket\TicketHistoryController;
use Illuminate\Support\Facades\Route;

// Ticket endpoint
Route::get('/', [TicketController::class, 'index'])
    ->middleware('throttle:search')
    ->name('index');
Route::post('/', [TicketController::class, 'store'])->name('store');
Route::get('/{ticket}', [TicketController::class, 'show'])->name('show');
Route::put('/{ticket}', [TicketController::class, 'update'])->name('update');
Route::delete('/{ticket}', [TicketController::class, 'destroy'])->name(
    'destroy',
);

Route::prefix('{ticket}')->group(function () {
    // Ticket transition
    Route::post('/status', [TicketController::class, 'transition'])->name(
        'status',
    );
    Route::post('/assign', [TicketController::class, 'assign'])->name('assign');
    Route::post('/unassign', [TicketController::class, 'unassign'])->name(
        'unassign',
    );
    Route::post('/priority', [TicketController::class, 'changePriority'])->name(
        'priority',
    );

    // Ticket comments
    Route::get('/comments', [TicketCommentController::class, 'index'])->name(
        'comments.index',
    );
    Route::post('/comments', [TicketCommentController::class, 'store'])->name(
        'comments.store',
    );
    Route::put('/comments/{comment}', [
        TicketCommentController::class,
        'update',
    ])->name('comments.update');
    Route::delete('/comments/{comment}', [
        TicketCommentController::class,
        'destroy',
    ])->name('comments.destroy');

    // Ticket History
    Route::get('/histories', [TicketHistoryController::class, 'index'])->name(
        'histories',
    );
});
