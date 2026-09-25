<?php

namespace App\Support;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Http\Request;
use Illuminate\Validation\ValidationException;

trait HandlesPagination
{
    /**
     * Mengambil page size dari request, defaultnya 10 dan maksimal dibatasi 100.
     */
    public function getPerPage(Request $request): int
    {
        $perPage = (int) $request->query('per_page', 10);

        return max(1, min($perPage, 100));
    }

    /**
     * Menerapkan pengurutan (sorting) yang diizinkan (whitelist) pada kueri. Kolom `sort_by`
     * yang tidak dikenal akan ditolak dengan status 422 untuk mencegah kebocoran informasi.
     *
     * @param  array<int, string>  $allowedColumns
     */
    public function applySorting(
        Builder $query,
        Request $request,
        array $allowedColumns,
    ): Builder {
        $sortBy = $request->query('sort_by', 'created_at');
        $sortDir = strtolower((string) $request->query('sort_dir', 'desc'));

        if (! in_array($sortBy, $allowedColumns, true)) {
            throw ValidationException::withMessages([
                'sort_by' => ['Kolom sort_by tidak valid.'],
            ]);
        }

        if (! in_array($sortDir, ['asc', 'desc'], true)) {
            throw ValidationException::withMessages([
                'sort_dir' => ['Arah pengurutan tidak valid.'],
            ]);
        }

        return $query->orderBy($sortBy, $sortDir);
    }
}
