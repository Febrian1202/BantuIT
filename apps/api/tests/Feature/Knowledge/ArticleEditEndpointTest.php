<?php

use App\Models\KnowledgeArticle;
use App\Models\User;
use Laravel\Sanctum\Sanctum;

test(
    'edit endpoint mengembalikan article tanpa incrementing view count',
    function () {
        $tech = User::factory()->technician()->create();
        $article = KnowledgeArticle::factory()->create([
            'author_id' => $tech->id,
            'status' => 'published',
            'view_count' => 5,
        ]);

        Sanctum::actingAs($tech);
        $response = $this->getJson("/api/articles/{$article->id}/edit");

        $response
            ->assertStatus(200)
            ->assertJsonPath('data.id', $article->id)
            ->assertJsonPath('data.title', $article->title)
            ->assertJsonPath('data.content', $article->content);

        // view_count should NOT have changed
        $this->assertEquals(5, $article->fresh()->view_count);
    },
);

test(
    'edit endpoint mengembalikan 404 untuk artikel yang tidak ada',
    function () {
        $tech = User::factory()->technician()->create();
        Sanctum::actingAs($tech);
        $this->getJson('/api/articles/999999/edit')->assertStatus(404);
    },
);

test('employee cannot access edit endpoint', function () {
    $emp = User::factory()->employee()->create();
    $article = KnowledgeArticle::factory()->create(['status' => 'published']);
    Sanctum::actingAs($emp);
    $this->getJson("/api/articles/{$article->id}/edit")->assertStatus(403);
});
