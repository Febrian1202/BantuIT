<?php

namespace App\DTOs\Admin;

class CreateTicketCategoryData
{
    public function __construct(
        public readonly string $name,
        public readonly ?int $parentId,
        public readonly ?string $description,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self(
            name: $data['name'],
            parentId: $data['parent_id'] ?? null,
            description: $data['description'] ?? null,
        );
    }
}
