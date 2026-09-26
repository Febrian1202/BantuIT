<?php

namespace App\Services\Asset;

use App\Models\Asset;

class AssetHistoryService
{
    public function timeline(Asset $asset): array
    {
        $assignments = $asset
            ->assignments()
            ->with('user')
            ->get()
            ->flatMap(function ($a) {
                $events = [];

                $events[] = [
                    'type' => 'assignment',
                    'action' => 'assigned',
                    'user' => $a->user
                        ? [
                            'id' => $a->user->id,
                            'full_name' => $a->user->full_name,
                        ]
                        : null,
                    'notes' => $a->notes,
                    'occurred_at' => $a->assigned_at?->toISOString(),
                ];

                if ($a->released_at) {
                    $events[] = [
                        'type' => 'assignment',
                        'action' => 'released',
                        'user' => $a->user
                            ? [
                                'id' => $a->user->id,
                                'full_name' => $a->user->full_name,
                            ]
                            : null,
                        'notes' => $a->notes,
                        'occurred_at' => $a->released_at?->toISOString(),
                    ];
                }

                return $events;
            });

        $histories = $asset->histories()->get()->map(
            fn ($h) => [
                'type' => 'history',
                'action' => $h->action,
                'description' => $h->description,
                'user' => null,
                'notes' => null,
                'occurred_at' => (
                    $h->action_at ?? $h->created_at
                )?->toISOString(),
            ],
        );

        return $assignments
            ->concat($histories)
            ->sortBy('occurred_at')
            ->values()
            ->all();
    }
}
