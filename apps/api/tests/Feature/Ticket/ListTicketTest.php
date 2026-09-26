<?php

use App\Models\Asset;
use App\Models\Department;
use App\Models\Ticket;
use App\Models\User;
use Illuminate\Support\Facades\DB;
use Laravel\Sanctum\Sanctum;

uses()->group('ticket');

test('employee hanya bisa melihat tiketnya sendiri', function () {
    $employee = User::factory()->employee()->create();
    Ticket::factory()
        ->open()
        ->count(3)
        ->create(['reporter_id' => $employee->id]);
    Ticket::factory()->open()->count(5)->create();
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets')->assertJsonPath('meta.total', 3);
});

test('filter by reporter_id tidak berpengaruh', function () {
    $employee = User::factory()->employee()->create();
    $other = User::factory()->employee()->create();
    Ticket::factory()
        ->open()
        ->create(['reporter_id' => $other->id]);
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets?reporter_id='.$other->id)->assertJsonPath(
        'meta.total',
        0,
    );
});

test('filter by sla_status breached dan on_track', function () {
    $employee = User::factory()->employee()->create();
    Ticket::factory()
        ->breached()
        ->count(2)
        ->create(['reporter_id' => $employee->id]);
    Ticket::factory()
        ->open()
        ->count(3)
        ->create(['reporter_id' => $employee->id]);
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets?sla_status=breached')->assertJsonPath(
        'meta.total',
        2,
    );
    $this->getJson('/api/tickets?sla_status=on_track')->assertJsonPath(
        'meta.total',
        3,
    );
});

test('mencari ticket_number dan title yang cocok', function () {
    $employee = User::factory()->employee()->create();
    Ticket::factory()
        ->open()
        ->create([
            'ticket_number' => 'TCK-9999',
            'reporter_id' => $employee->id,
        ]);
    Ticket::factory()
        ->open()
        ->create(['title' => 'Laptop rusak', 'reporter_id' => $employee->id]);
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets?search=9999')->assertJsonPath('meta.total', 1);
    $this->getJson('/api/tickets?search=Laptop')->assertJsonPath(
        'meta.total',
        1,
    );
});

test('mencari title yang mengandung wildcard', function () {
    $employee = User::factory()->employee()->create();
    Ticket::factory()
        ->open()
        ->create([
            'title' => 'Laptop 50% rusak',
            'reporter_id' => $employee->id,
        ]);
    Ticket::factory()
        ->open()
        ->create([
            'title' => 'Monitor_Screen mati',
            'reporter_id' => $employee->id,
        ]);
    Ticket::factory()
        ->open()
        ->count(2)
        ->create(['reporter_id' => $employee->id]);
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets?search=%')->assertJsonPath('meta.total', 1);
    $this->getJson('/api/tickets?search=_')->assertJsonPath('meta.total', 1);
});

test('filter berdasarkan status_id', function () {
    $manager = User::factory()->manager()->create();
    Ticket::factory()
        ->open()
        ->create(['status_id' => 1]);
    Ticket::factory()
        ->resolved()
        ->create(['status_id' => 4]);
    Sanctum::actingAs($manager);

    $this->getJson('/api/tickets?status_id=1')->assertJsonPath('meta.total', 1);
    $this->getJson('/api/tickets?status_id=4')->assertJsonPath('meta.total', 1);
    $this->getJson('/api/tickets?status_id=1,4')->assertJsonPath(
        'meta.total',
        2,
    );
});

test('filter berdasarkan priority_id', function () {
    $manager = User::factory()->manager()->create();
    Ticket::factory()
        ->open()
        ->create(['priority_id' => 1]);
    Ticket::factory()
        ->open()
        ->create(['priority_id' => 2]);
    Sanctum::actingAs($manager);

    $this->getJson('/api/tickets?priority_id=1')->assertJsonPath(
        'meta.total',
        1,
    );
    $this->getJson('/api/tickets?priority_id=1,2')->assertJsonPath(
        'meta.total',
        2,
    );
});

test('filter berdasarkan category_id', function () {
    $manager = User::factory()->manager()->create();
    Ticket::factory()
        ->open()
        ->create(['category_id' => 1]);
    Ticket::factory()
        ->open()
        ->create(['category_id' => 2]);
    Sanctum::actingAs($manager);

    $this->getJson('/api/tickets?category_id=1')->assertJsonPath(
        'meta.total',
        1,
    );
    $this->getJson('/api/tickets?category_id=1,2')->assertJsonPath(
        'meta.total',
        2,
    );
});

test('filter berdasarkan technician_id', function () {
    $manager = User::factory()->manager()->create();
    $tech = User::factory()->technician()->create();
    Ticket::factory()
        ->open()
        ->create(['technician_id' => $tech->id]);
    Ticket::factory()
        ->open()
        ->create(['technician_id' => User::factory()->technician()]);
    Sanctum::actingAs($manager);

    $this->getJson('/api/tickets?technician_id='.$tech->id)->assertJsonPath(
        'meta.total',
        1,
    );
});

test('filter berdasarkan technician_id unassigned', function () {
    $manager = User::factory()->manager()->create();
    Ticket::factory()
        ->open()
        ->create(['technician_id' => null]);
    Ticket::factory()
        ->open()
        ->create(['technician_id' => User::factory()->technician()]);
    Sanctum::actingAs($manager);

    $this->getJson('/api/tickets?technician_id=unassigned')->assertJsonPath(
        'meta.total',
        1,
    );
});

test('filter berdasarkan department_id', function () {
    $manager = User::factory()->manager()->create();
    $department = Department::factory()->create();
    Ticket::factory()
        ->open()
        ->create(['department_id' => $department->id]);
    Ticket::factory()->open()->create();
    Sanctum::actingAs($manager);

    $this->getJson(
        '/api/tickets?department_id='.$department->id,
    )->assertJsonPath('meta.total', 1);
});

test('filter berdasarkan asset_id', function () {
    $manager = User::factory()->manager()->create();
    $asset = Asset::factory()->create();
    Ticket::factory()
        ->open()
        ->create(['asset_id' => $asset->id]);
    Ticket::factory()->open()->create();
    Sanctum::actingAs($manager);

    $this->getJson('/api/tickets?asset_id='.$asset->id)->assertJsonPath(
        'meta.total',
        1,
    );
});

test('filter berdasarkan created_from dan created_to', function () {
    $manager = User::factory()->manager()->create();
    $ticket = Ticket::factory()->open()->create();
    $ticket->forceFill(['created_at' => '2024-01-15 10:00:00'])->save();
    $other = Ticket::factory()->open()->create();
    $other->forceFill(['created_at' => '2024-02-20 10:00:00'])->save();
    Sanctum::actingAs($manager);

    $this->getJson(
        '/api/tickets?created_from=2024-01-01&created_to=2024-01-31',
    )->assertJsonPath('meta.total', 1);
    $this->getJson('/api/tickets?created_from=2024-02-01')->assertJsonPath(
        'meta.total',
        1,
    );
    $this->getJson('/api/tickets?created_to=2024-01-01')->assertJsonPath(
        'meta.total',
        0,
    );
});

test('invalid sort_by mengembalikan 422', function () {
    $employee = User::factory()->employee()->create();
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets?sort_by=password')
        ->assertStatus(422)
        ->assertJsonValidationErrors('sort_by');
});

test('meta memiliki tepat enam kunci dan tidak ada link', function () {
    $employee = User::factory()->employee()->create();
    Ticket::factory()
        ->open()
        ->count(5)
        ->create(['reporter_id' => $employee->id]);
    Sanctum::actingAs($employee);

    $response = $this->getJson('/api/tickets?per_page=2');
    $response->assertJsonStructure([
        'meta' => [
            'current_page',
            'per_page',
            'total',
            'last_page',
            'from',
            'to',
        ],
    ]);
    expect($response->json('meta'))->toHaveCount(6);
    expect($response->json('meta'))->not->toHaveKey('links');
});

test('per_page di batasi di 100', function () {
    $employee = User::factory()->employee()->create();
    Sanctum::actingAs($employee);

    $this->getJson('/api/tickets?per_page=500')->assertJsonPath(
        'meta.per_page',
        100,
    );
});

test(
    'jumlah query tidak bertambah dengan ticket yang lebih banyak',
    function () {
        $queries = [];
        DB::listen(function ($query) use (&$queries) {
            $queries[] = $query;
        });
        $admin = User::factory()->admin()->create();
        Sanctum::actingAs($admin);
        $admin->role;

        Ticket::factory()->open()->create();
        $queries = [];
        $this->getJson('/api/tickets');
        $count1 = count($queries);

        Ticket::factory()->open()->count(9)->create();
        $queries = [];
        $this->getJson('/api/tickets');
        $count2 = count($queries);

        expect($count2)->toBe($count1);
    },
);
