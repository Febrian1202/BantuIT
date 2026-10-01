<?php

namespace App\DTOs\Admin;

class CreateTicketPriorityData
{
    public function __construct(
        public readonly string $name,
        public readonly int $level,
        public readonly int $slaMinutes,
        public readonly ?string $description,
    ) {}

    public static function fromArray(array $data): self
    {
        return new self(
            name: $data['name'],
            level: $data['level'],
            slaMinutes: $data['sla_minutes'],
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
