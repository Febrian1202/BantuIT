<?php

namespace App\Services\Admin;

use App\Exceptions\StateConflictException;
use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Relations\Relation;

class ReferentialIntegrityGuard
{
    /**
     * Memastikan tidak ada data terkait yang masih digunakan.
     *
     * @param  array<string, Builder|Relation>  $references  label => query/relation
     *
     * @throws StateConflictException jika ada data terkait yang masih digunakan
     */
    public function assertUnreferenced(array $references): void
    {
        foreach ($references as $label => $query) {
            $count = $query->count();
            if ($count > 0) {
                throw new StateConflictException(
                    "{$label} masih digunakan oleh {$count} data terkait dan tidak dapat dihapus.",
                );
            }
        }
    }
}
