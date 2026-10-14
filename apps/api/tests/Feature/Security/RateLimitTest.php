<?php

use App\Models\User;
use Illuminate\Cache\RateLimiting\Limit;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\RateLimiter;
use Laravel\Sanctum\Sanctum;

test('ticket list endpoint enforces search rate limiter at 60 requests per minute', function () {
    $user = User::factory()->employee()->create();
    Sanctum::actingAs($user);

    for ($i = 0; $i < 60; $i++) {
        $response = $this->getJson('/api/tickets?search=printer');
        $response->assertStatus(200);
    }

    $response = $this->getJson('/api/tickets?search=printer');
    $response->assertStatus(429);
});

test('article list endpoint enforces search rate limiter', function () {
    $user = User::factory()->employee()->create();
    Sanctum::actingAs($user);

    for ($i = 0; $i < 60; $i++) {
        $response = $this->getJson('/api/articles?search=guide');
        $response->assertStatus(200);
    }

    $response = $this->getJson('/api/articles?search=guide');
    $response->assertStatus(429);
});

test('asset list endpoint enforces search rate limiter', function () {
    $user = User::factory()->technician()->create();
    Sanctum::actingAs($user);

    for ($i = 0; $i < 60; $i++) {
        $response = $this->getJson('/api/assets?search=laptop');
        $response->assertStatus(200);
    }

    $response = $this->getJson('/api/assets?search=laptop');
    $response->assertStatus(429);
});

test('login rate limiter configures 5 requests per minute limit in production', function () {
    $request = Request::create('/api/login', 'POST', server: ['REMOTE_ADDR' => '127.0.0.1']);

    app()->detectEnvironment(fn () => 'production');

    $limiter = RateLimiter::limiter('login');
    expect($limiter)->not->toBeNull();

    $limit = $limiter($request);
    expect($limit)->toBeInstanceOf(Limit::class);
    expect($limit->maxAttempts)->toBe(5);
    expect($limit->decaySeconds)->toBe(60);

    app()->detectEnvironment(fn () => 'testing');
});
