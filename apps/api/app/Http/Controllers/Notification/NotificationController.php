<?php

namespace App\Http\Controllers\Notification;

use App\Http\Controllers\Controller;
use App\Http\Requests\Notification\IndexNotificationRequest;
use App\Http\Resources\Notification\NotificationResource;
use App\Models\Notification;
use App\Services\Notification\NotificationQueryService;
use App\Support\ApiResponse;
use App\Support\HandlesPagination;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class NotificationController extends Controller
{
    use HandlesPagination;

    public function __construct(
        protected NotificationQueryService $queryService,
    ) {}

    /**
     * Mendapatkan daftar notifikasi
     */
    public function index(IndexNotificationRequest $request): JsonResponse
    {
        $this->authorize("viewAny", Notification::class);

        $perPage = $this->getPerPage($request);
        $sortBy = $request->query("sort_by", "created_at");
        $sortDir = $request->query("sort_dir", "desc");

        $paginator = $this->queryService->paginate(
            user: $request->user(),
            filters: $request->validated(),
            perPage: $perPage,
            sortBy: $sortBy,
            sortDirection: $sortDir,
        );

        return ApiResponse::paginated(
            $paginator,
            "Notifications retrieved successfully.",
            NotificationResource::class,
        );
    }

    /**
     * Mendapatkan jumlah notifikasi yang belum dibaca
     */
    public function unreadCount(Request $request): JsonResponse
    {
        $this->authorize("viewAny", Notification::class);

        $count = $this->queryService->getUnreadCount($request->user());

        return ApiResponse::success(
            data: [
                "unread_count" => $count,
            ],
            message: "Unread notifications count retrieved successfully.",
        );
    }

    /**
     * Menandai notifikasi sebagai sudah dibaca
     */
    public function read(
        Request $request,
        Notification $notification,
    ): JsonResponse {
        if ($notification->user_id !== $request->user()->id) {
            abort(404, "Resource not found.");
        }

        $this->authorize("markAsRead", $notification);

        $this->queryService->markAsRead($notification);

        return ApiResponse::success(null, "Notification marked as read.");
    }

    public function readAll(Request $request): JsonResponse
    {
        $this->authorize("markAllAsRead", Notification::class);

        $this->queryService->markAllAsRead($request->user());

        return ApiResponse::success(null, "All notifications marked as read.");
    }
}
