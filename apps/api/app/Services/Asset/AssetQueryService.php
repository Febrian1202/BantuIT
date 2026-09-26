<?php

namespace App\Services\Asset;

use App\Enums\AssetStatus;
use App\Models\Asset;
use App\Models\AssetAssignment;
use App\Models\User;
use App\Support\HandlesPagination;
use Illuminate\Http\Request;
use Illuminate\Pagination\LengthAwarePaginator;
use Illuminate\Support\Collection;

class AssetQueryService
{
    use HandlesPagination;

    /**
     * Mengambil daftar aset dengan pagination.
     */
    public function paginate(Request $request): LengthAwarePaginator
    {
        $query = Asset::query()->with(['activeAssignment.user']);

        if ($search = $request->search) {
            $term = str_replace(['%', '_'], ['\\%', '\\_'], (string) $search);
            $query->where(function ($q) use ($term) {
                $q->where('asset_tag', 'LIKE', "%{$term}%")
                    ->orWhere('serial_number', 'LIKE', "%{$term}%")
                    ->orWhere('name', 'LIKE', "%{$term}%");
            });
        }

        if ($status = $request->status) {
            $query->where('status', $status);
        }

        if ($category = $request->category) {
            $query->where('category', $category);
        }

        if ($assignedUserId = $request->assigned_user_id) {
            $query->whereHas('activeAssignment', function ($q) use (
                $assignedUserId,
            ) {
                $q->where('user_id', $assignedUserId);
            });
        }

        return $this->applySorting($query, $request, [
            'asset_tag',
            'name',
            'status',
            'purchase_date',
            'created_at',
        ])->paginate($this->getPerPage($request));
    }

    /**
     * Mengambil daftar kategori aset.
     */
    public function categories(): Collection
    {
        return Asset::query()
            ->whereNotNull('category')
            ->distinct()
            ->orderBy('category')
            ->pluck('category');
    }

    /**
     * Mengambil detail aset berdasarkan ID.
     */
    public function show(Asset $asset): Asset
    {
        return $asset->load(['activeAssignment.user']);
    }

    /**
     * Mengambil daftar aset yang dimiliki oleh pengguna yang sedang login.
     */
    public function myAssets(User $user, Request $request): LengthAwarePaginator
    {
        return Asset::query()
            ->with(['activeAssignment.user'])
            ->whereHas(
                'activeAssignment',
                fn ($q) => $q->where('user_id', $user->id),
            )
            ->orderBy('created_at')
            ->paginate($this->getPerPage($request));
    }

    /**
     * Mengambil daftar aset yang dapat ditugaskan.
     */
    public function assignable(User $user): Collection
    {
        return Asset::whereIn(
            'id',
            AssetAssignment::where('user_id', $user->id)
                ->whereNull('released_at')
                ->pluck('asset_id'),
        )
            ->whereIn('status', [AssetStatus::Available, AssetStatus::Assigned])
            ->get();
    }
}
