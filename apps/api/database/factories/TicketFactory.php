<?php

namespace Database\Factories;

use App\Models\Department;
use App\Models\Ticket;
use App\Models\TicketCategory;
use App\Models\User;
use Illuminate\Database\Eloquent\Factories\Factory;

class TicketFactory extends Factory
{
    protected $model = Ticket::class;

    public function definition(): array
    {
        $priorityId = fake()->randomElement([1, 2, 3, 4]);

        $durations = [
            1 => 120,
            2 => 240,
            3 => 480,
            4 => 1440,
        ];
        $duration = $durations[$priorityId];

        return [
            "ticket_number" => "TCK-" . fake()->unique()->numerify("#####"),
            "title" => fake()->sentence(),
            "description" => fake()->paragraph(),
            "category_id" =>
                TicketCategory::query()->inRandomOrder()->value("id") ??
                TicketCategory::factory(),
            "priority_id" => $priorityId,
            "status_id" => 1,
            "reporter_id" => User::factory(),
            "technician_id" => null,
            "department_id" => Department::factory(),
            "asset_id" => null,
            "sla_duration_minutes" => $duration,
            "sla_deadline" => now()->addMinutes($duration),
            "resolved_at" => null,
            "closed_at" => null,
            "sla_breached" => false,
            "sla_breached_at" => null,
        ];
    }

    /**
     * Mendefinisikan state untuk ticket yang masih terbuka.
     */
    public function open(): static
    {
        return $this->state([
            "status_id" => 1,
            "technician_id" => null,
            "resolved_at" => null,
            "closed_at" => null,
        ]);
    }

    /**
     * Mendefinisikan state untuk ticket yang sudah ditugaskan.
     */
    public function assigned(): static
    {
        return $this->state([
            "status_id" => 2,
            "technician_id" => User::factory()->technician(),
        ]);
    }

    /**
     * Mendefinisikan state untuk ticket yang sedang dalam proses.
     */
    public function inProgress(): static
    {
        return $this->state([
            "status_id" => 3,
            "technician_id" => User::factory()->technician(),
        ]);
    }

    /**
     * Mendefinisikan state untuk ticket yang sudah selesai.
     */
    public function resolved(): static
    {
        return $this->state([
            "status_id" => 4,
            "resolved_at" => now(),
        ]);
    }

    /**
     * Mendefinisikan state untuk ticket yang sudah ditutup.
     */
    public function closed(): static
    {
        return $this->state([
            "status_id" => 5,
            "closed_at" => now(),
        ]);
    }

    /**
     * Mendefinisikan state untuk ticket yang sudah melewati batas waktu SLA.
     */
    public function breached(): static
    {
        return $this->state([
            "sla_breached" => true,
            "sla_breached_at" => now(),
            "sla_deadline" => now()->subHour(),
        ]);
    }

    /**
     * Mendefinisikan state untuk ticket yang sudah ditugaskan ke teknisi.
     */
    public function withTechnician(): static
    {
        return $this->state([
            "technician_id" => User::factory()->technician(),
        ]);
    }
}
