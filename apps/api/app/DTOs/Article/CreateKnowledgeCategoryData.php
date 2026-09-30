<?php
namespace App\DTOs\Article;

class CreateKnowledgeCategoryData
{
    public function __construct(
        public string $name,
        public ?string $description,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self(
            name: $data["name"],
            description: $data["description"] ?? null,
        );
    }

    public function toArray(): array
    {
        return [
            "name" => $this->name,
            "description" => $this->description,
        ];
    }
}
