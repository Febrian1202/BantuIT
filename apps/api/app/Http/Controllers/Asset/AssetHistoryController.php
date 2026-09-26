<?php

namespace App\Http\Controllers\Asset;

use App\Http\Controllers\Controller;
use App\Models\Asset;
use App\Services\Asset\AssetHistoryService;
use App\Support\ApiResponse;
use Illuminate\Http\JsonResponse;

class AssetHistoryController extends Controller
{
    public function __construct(private AssetHistoryService $historyService) {}

    public function __invoke(Asset $asset): JsonResponse
    {
        $this->authorize('viewHistory', $asset);

        $timeline = $this->historyService->timeline($asset);

        return ApiResponse::success(
            $timeline,
            'Asset history retrieved successfully.',
        );
    }
}
