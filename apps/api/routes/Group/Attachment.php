<?php

use App\Http\Controllers\Ticket\TicketAttachmentController;
use Illuminate\Support\Facades\Route;

Route::get('{attachment}/download', [
    TicketAttachmentController::class,
    'download',
])->name('donwload');
Route::delete('{attachment}', [
    TicketAttachmentController::class,
    'destroy',
])->name('destroy');
