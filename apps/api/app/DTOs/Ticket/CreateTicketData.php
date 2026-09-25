<?php

namespace App\DTOs\Ticket;

class CreateTicketData
{
    public function __construct(
        public readonly string $title,
        public readonly string $description,
        public readonly int $categoryId,
        public readonly int $priorityId,
        public readonly ?int $assetid = null,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self(
            title: $data['title'],
            description: $data['description'],
            categoryId: $data['category_id'],
            priorityId: $data['priority_id'],
            assetid: isset($data['asset_id']) ? $data['asset_id'] : null,
        );
    }
}
