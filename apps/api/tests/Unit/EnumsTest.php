<?php

use App\Enums\ArticleStatus;
use App\Enums\AssetHistoryAction;
use App\Enums\AssetStatus;
use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Enums\NotificationType;
use App\Enums\RoleName;
use App\Enums\TicketStatusName;
use App\Enums\UserStatus;
use App\Models\Concerns\SerializesDatesAsIso8601;
use Illuminate\Database\Eloquent\Model;

test('Enum memiliki nilai yang sesuai', function () {
    expect(RoleName::Admin->value)
        ->toBe('administrator')
        ->and(RoleName::Manager->value)
        ->toBe('manager')
        ->and(RoleName::Technician->value)
        ->toBe('technician')
        ->and(RoleName::Employee->value)
        ->toBe('employee')
        ->and(UserStatus::Active->value)
        ->toBe('active')
        ->and(UserStatus::Inactive->value)
        ->toBe('inactive')
        ->and(TicketStatusName::Open->value)
        ->toBe('OPEN')
        ->and(TicketStatusName::Assigned->value)
        ->toBe('ASSIGNED')
        ->and(TicketStatusName::InProgress->value)
        ->toBe('IN_PROGRESS')
        ->and(TicketStatusName::Resolved->value)
        ->toBe('RESOLVED')
        ->and(TicketStatusName::Closed->value)
        ->toBe('CLOSED')
        ->and(AssetStatus::Available->value)
        ->toBe('available')
        ->and(AssetStatus::Assigned->value)
        ->toBe('assigned')
        ->and(AssetStatus::Maintenance->value)
        ->toBe('maintenance')
        ->and(AssetStatus::Retired->value)
        ->toBe('retired')
        ->and(AssetStatus::Lost->value)
        ->toBe('lost');
});

test(
    'SerializesDatesAsIso8601 mengubah tanggal menjadi format UTC ISO 8601 dengan akhiran Z.',
    function () {
        $model = new class extends Model
        {
            use SerializesDatesAsIso8601;

            public function serializeDatePublic(DateTimeInterface $date): string
            {
                return $this->serializeDate($date);
            }
        };

        $dateTime = new DateTimeImmutable(
            '2026-09-01 17:00:00',
            new DateTimeZone('Asia/Jakarta'),
        );
        expect($model->serializeDatePublic($dateTime))->toBe(
            '2026-09-01T10:00:00Z',
        );
    },
);

test('AuditAction punya semua 22 case', function () {
    $cases = array_map(fn ($c) => $c->value, AuditAction::cases());
    expect($cases)->toContain(
        'create',
        'update',
        'delete',
        'assign',
        'reassign',
        'unassign',
        'self_assign',
        'status_change',
        'priority_change',
        'reopen',
        'resolve',
        'close',
        'cancel',
        'login',
        'logout',
        'password_reset',
        'sla_breach',
        'release',
        'publish',
        'unpublish',
        'activate',
        'deactivate',
    );
});

test('AuditModule punya semua 9 case', function () {
    $cases = array_map(fn ($c) => $c->value, AuditModule::cases());
    expect($cases)->toContain(
        'ticket',
        'asset',
        'article',
        'user',
        'role',
        'department',
        'ticket_category',
        'ticket_priority',
        'auth',
        'knowledge_category',
    );
});

test('NotificationType punya semua 11 case', function () {
    $cases = array_map(fn ($c) => $c->value, NotificationType::cases());
    expect($cases)->toContain(
        'TICKET_ASSIGNED',
        'TICKET_REASSIGNED',
        'TICKET_UNASSIGNED',
        'TICKET_STATUS_CHANGED',
        'TICKET_SELF_ASSIGNED',
        'TICKET_REOPENED',
        'TICKET_RESOLVED',
        'TICKET_CLOSED',
        'TICKET_CANCELLED',
        'TICKET_COMMENTED',
        'TICKET_SLA_BREACHED',
    );
});

test('ArticleStatus enum berisi draft dan published', function () {
    expect(ArticleStatus::Draft->value)
        ->toBe('draft')
        ->and(ArticleStatus::Published->value)
        ->toBe('published');
});

test('AssetHistoryAction enum berisi semua 4 case', function () {
    expect(AssetHistoryAction::Created->value)
        ->toBe('created')
        ->and(AssetHistoryAction::Assigned->value)
        ->toBe('assigned')
        ->and(AssetHistoryAction::Released->value)
        ->toBe('released')
        ->and(AssetHistoryAction::StatusChanged->value)
        ->toBe('status_changed');
});
