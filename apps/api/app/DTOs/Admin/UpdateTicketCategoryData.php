<?php

namespace App\DTOs\Admin;

class UpdateTicketCategoryData
{
    public function __construct(
        public readonly ?string $name,
        public readonly ?int $parentId,
        public readonly ?string $description,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self(
            name: $data["name"] ?? null,
            parentId: $data["parent_id"] ?? null,
            description: $data["description"] ?? null,
        );
    }

    public function toArray(): array
    {
        return [
            "name" => $this->name,
            "parent_id" => $this->parentId,
            "description" => $this->description,
        ];
    }
}
