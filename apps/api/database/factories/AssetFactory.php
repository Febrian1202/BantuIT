<?php

namespace Database\Factories;

use App\Enums\AssetStatus;
use App\Models\Asset;
use Illuminate\Database\Eloquent\Factories\Factory;

/**
 * @extends Factory<Asset>
 */
class AssetFactory extends Factory
{
    /**
     * Menentukan model yang digunakan oleh factory ini.
     */
    protected $model = Asset::class;

    public function definition(): array
    {
        return [
            "asset_tag" => "AST-" . fake()->unique()->numerify("#####"),
            "name" => fake()->word() . " Device",
            "category" => fake()->randomElement([
                "Laptop",
                "Monitor",
                "Printer",
                "Server",
                "Network",
            ]),
            "brand" => fake()->company(),
            "model" => fake()->word() . " Pro",
            "serial_number" => fake()->unique()->bothify("SN-####-????"),
            "purchase_date" => fake()->date(),
            "status" => AssetStatus::Available,
            "notes" => fake()->optional()->sentence(),
        ];
    }

    /**
     * Mengatur status asset menjadi Available.
     */
    public function available(): static
    {
        return $this->state(["status" => AssetStatus::Available]);
    }

    /**
     * Mengatur status asset menjadi Assigned.
     */
    public function assigned(): static
    {
        return $this->state(["status" => AssetStatus::Assigned]);
    }

    /**
     * Mengatur status asset menjadi Maintenance.
     */
    public function maintenance(): static
    {
        return $this->state(["status" => AssetStatus::Maintenance]);
    }

    /**
     * Mengatur status asset menjadi Retired.
     */
    public function retired(): static
    {
        return $this->state(["status" => AssetStatus::Retired]);
    }

    /**
     * Mengatur status asset menjadi Lost.
     */
    public function lost(): static
    {
        return $this->state(["status" => AssetStatus::Lost]);
    }
}
