<?php

use App\Enums\AssetStatus;
use App\Models\Asset;
use App\Models\AssetAssignment;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

uses()->group('ticket', 'route');

beforeEach(function () {
    $this->employee = User::factory()->employee()->create();
    Sanctum::actingAs($this->employee);
});

test('route index ticket terdaftar', function () {
    $this->getJson('/api/tickets')->assertStatus(200);
});

test(
    'assignable hanya mengembalikan asset yang ditugaskan ke pengguna saat ini',
    function () {
        $otherUser = User::factory()->employee()->create();

        $myAsset = Asset::factory()->create([
            'status' => AssetStatus::Assigned,
        ]);
        AssetAssignment::factory()->create([
            'asset_id' => $myAsset->id,
            'user_id' => $this->employee->id,
            'released_at' => null,
        ]);

        $otherAsset = Asset::factory()->create([
            'status' => AssetStatus::Assigned,
        ]);
        AssetAssignment::factory()->create([
            'asset_id' => $otherAsset->id,
            'user_id' => $otherUser->id,
            'released_at' => null,
        ]);

        $this->getJson('/api/assets/assignable')
            ->assertStatus(200)
            ->assertJsonCount(1, 'data')
            ->assertJsonPath('data.0.id', $myAsset->id);
    },
);

test(
    'assignable tidak mengembalikan asset yang hilang dan sudah dihapus',
    function () {
        $retiredAsset = Asset::factory()->create([
            'status' => AssetStatus::Retired,
        ]);
        AssetAssignment::factory()->create([
            'asset_id' => $retiredAsset->id,
            'user_id' => $this->employee->id,
            'released_at' => null,
        ]);

        $this->getJson('/api/assets/assignable')
            ->assertStatus(200)
            ->assertJsonCount(0, 'data');
    },
);
