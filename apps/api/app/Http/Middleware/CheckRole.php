<?php

namespace App\Http\Middleware;

use App\Support\ApiResponse;
use Closure;
use Illuminate\Http\Request;
use Symfony\Component\HttpFoundation\Response;

class CheckRole
{
    /**
     * Menolak request ketika pengguna tidak memiliki salah satu dari role yang diizinkan.
     *
     * @param  string  ...$roles  Satu atau lebih role (e.g. `role:manager,admin`)
     */
    public function handle(
        Request $request,
        Closure $next,
        string ...$roles,
    ): Response {
        $user = $request->user();

        if ($user && $user->hasRole(...$roles)) {
            return $next($request);
        }

        return ApiResponse::error("Forbidden.", status: 403);
    }
}
