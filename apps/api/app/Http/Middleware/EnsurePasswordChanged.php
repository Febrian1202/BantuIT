<?php

namespace App\Http\Middleware;

use App\Support\ApiResponse;
use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

class EnsurePasswordChanged
{
    /**
     * Rute yang user bisa akses ketika must_change_password true.
     */
    private const ALLOWED_ROUTES = [
        'me.password.update',
        'me.show',
        'auth.logout',
    ];

    /**
     * Menolak request kecuali untuk password update, profile view, dan logout
     * ketika must_change_password true.
     */
    public function handle(Request $request, Closure $next): Response
    {
        $user = $request->user();

        if (
            $user &&
            $user->must_change_password &&
            ! in_array($request->route()?->getName(), self::ALLOWED_ROUTES, true)
        ) {
            return ApiResponse::error('Password harus diganti', status: 403);
        }

        return $next($request);
    }
}
