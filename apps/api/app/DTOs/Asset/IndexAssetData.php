<?php

namespace App\DTOs\Asset;

use App\Http\Requests\Asset\IndexAssetRequest;

class IndexAssetData
{
    public function __construct(
        public readonly ?string $search = null,
        public readonly ?string $status = null,
        public readonly ?string $category = null,
        public readonly ?string $assignedUserId = null,
        public readonly ?string $sortBy = null,
        public readonly ?string $sortDir = null,
        public readonly ?int $perPage = null,
    ) {}

    public function fromRequest(IndexAssetRequest $data): self
    {
        return new self(
            $data->search ?? null,
            $data->status ?? null,
            $data->category ?? null,
            $data->assignedUserId ?? null,
            $data->sortBy ?? null,
            $data->sortDir ?? null,
            $data->perPage ?? null,
        );
    }

    public function toArray(): array
    {
        return [
            'search' => $this->search,
            'status' => $this->status,
            'category' => $this->category,
            'assignedUserId' => $this->assignedUserId,
            'sortBy' => $this->sortBy,
            'sortDir' => $this->sortDir,
            'perPage' => $this->perPage,
        ];
    }
}
