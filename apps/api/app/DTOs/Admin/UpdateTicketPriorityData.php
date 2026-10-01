<?php

namespace App\DTOs\Admin;

class UpdateTicketPriorityData
{
    public function __construct(
        public readonly ?string $name,
        public readonly ?int $level,
        public readonly ?int $slaMinutes,
        public readonly ?string $description,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self(
            name: $data['name'] ?? null,
            level: $data['level'] ?? null,
            slaMinutes: $data['sla_minutes'] ?? null,
            description: $data['description'] ?? null,
        );
    }

    public function toArray(): array
    {
        return [
            'name' => $this->name,
            'level' => $this->level,
            'sla_minutes' => $this->slaMinutes,
            'description' => $this->description,
        ];
    }
}
