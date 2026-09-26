<?php

namespace App\Http\Controllers\Asset;

use App\DTOs\Asset\AssignAssetData;
use App\DTOs\Asset\CreateAssetData;
use App\DTOs\Asset\ReleaseAssetData;
use App\DTOs\Asset\UpdateAssetData;
use App\Http\Controllers\Controller;
use App\Http\Requests\Asset\AssignAssetRequest;
use App\Http\Requests\Asset\IndexAssetRequest;
use App\Http\Requests\Asset\ReleaseAssetRequest;
use App\Http\Requests\Asset\StoreAssetRequest;
use App\Http\Requests\Asset\UpdateAssetRequest;
use App\Http\Resources\Asset\AssetListResource;
use App\Http\Resources\Asset\AssetResource;
use App\Http\Resources\Asset\AssignableAssetResource;
use App\Models\Asset;
use App\Services\Asset\AssetAssignmentService;
use App\Services\Asset\AssetQueryService;
use App\Services\Asset\AssetService;
use App\Support\ApiResponse;
use App\Support\HandlesPagination;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class AssetController extends Controller
{
    use HandlesPagination;

    public function __construct(
        private AssetQueryService $queryService,
        private AssetService $assetService,
        private AssetAssignmentService $assignmentService,
    ) {}

    /**
     * Handler untuk mendapatkan daftar asset dengan pagination.
     */
    public function index(IndexAssetRequest $request): JsonResponse
    {
        $this->authorize('viewAny', Asset::class);

        $paginator = $this->queryService->paginate($request);

        return ApiResponse::success(
            $paginator,
            'Assets retrieved successfully.',
            AssetListResource::class,
        );
    }

    /**
     * Handler untuk mendapatkan daftar kategori asset.
     */
    public function categories(): JsonResponse
    {
        $this->authorize('viewAny', Asset::class);

        $categories = $this->queryService->categories();

        return ApiResponse::success($categories, 'Asset categories retrieved.');
    }

    /**
     * Handler untuk mendapatkan detail asset.
     */
    public function show(Asset $asset): JsonResponse
    {
        $this->authorize('view', $asset);

        $data = $this->queryService->show($asset);

        return ApiResponse::success(
            new AssetResource($data),
            'Asset details retrieved successfully.',
        );
    }

    /**
     * Handler untuk membuat asset baru.
     */
    public function store(StoreAssetRequest $request): JsonResponse
    {
        $this->authorize('create', Asset::class);

        $data = $this->assetService->create(
            CreateAssetData::fromArray($request->validated()),
            $request->user(),
        );

        return ApiResponse::success(
            new AssetListResource($data),
            'Asset created successfully.',
            201,
        );
    }

    /**
     * Handler untuk memperbarui asset.
     */
    public function update(
        UpdateAssetRequest $request,
        Asset $asset,
    ): JsonResponse {
        $this->authorize('update', $asset);

        $oldData = $asset->only([
            'asset_tag',
            'name',
            'category',
            'brand',
            'model',
            'serial_number',
            'purchase_date',
            'status',
            'notes',
        ]);
        $dto = UpdateAssetData::fromArray($request->validated(), $oldData);
        $updated = $this->assetService->update($asset, $dto, $request->user());

        return ApiResponse::success(
            new AssetListResource($updated),
            'Asset updated successfully.',
        );
    }

    /**
     * Handler untuk menghapus asset.
     */
    public function destroy(Request $request, Asset $asset): JsonResponse
    {
        $this->authorize('delete', $asset);

        $this->assetService->delete($asset, $request->user());

        return ApiResponse::success(null, 'Asset deleted successfully.');
    }

    /**
     * Handler untuk meng-assign asset ke pengguna.
     */
    public function assign(
        Asset $asset,
        AssignAssetRequest $request,
    ): JsonResponse {
        $this->authorize('assign', $asset);

        $dto = AssignAssetData::fromArray($request->validated());
        $assigned = $this->assignmentService->assign(
            $asset,
            $dto,
            $request->user(),
        );

        return ApiResponse::success(
            new AssetListResource($assigned),
            'Asset assigned successfully.',
        );
    }

    /**
     * Handler untuk mengcabut asset dari pengguna.
     */
    public function release(
        ReleaseAssetRequest $request,
        Asset $asset,
    ): JsonResponse {
        $this->authorize('release', $asset);

        $dto = ReleaseAssetData::fromArray($request->validated());
        $released = $this->assignmentService->release(
            $asset,
            $dto,
            $request->user(),
        );

        return ApiResponse::success(
            new AssetListResource($released),
            'Asset released successfully.',
        );
    }

    /**
     * Handler untuk mengambil daftar aset yang dimiliki oleh pengguna yang sedang login.
     */
    public function myAssets(Request $request): JsonResponse
    {
        $this->authorize('viewOwn', Asset::class);

        $paginator = $this->queryService->myAssets($request->user(), $request);

        return ApiResponse::paginated(
            $paginator,
            'My assets retrieved successfully.',
            AssetListResource::class,
        );
    }

    public function assignable(Request $request): JsonResponse
    {
        $this->authorize('viewAssignable', Asset::class);

        $assets = $this->queryService->assignable($request->user());

        return ApiResponse::success(
            AssignableAssetResource::collection($assets),
            'Assignable assets retrieved.',
        );
    }
}
