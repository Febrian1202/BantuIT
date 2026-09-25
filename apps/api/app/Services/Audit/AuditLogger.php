<?php

namespace App\Services\Audit;

use App\Enums\AuditAction;
use App\Enums\AuditModule;
use App\Models\AuditLog;
use App\Models\User;
use Illuminate\Http\Request;

class AuditLogger
{
    public function log(
        ?User $actor,
        AuditAction $action,
        AuditModule $module,
        ?int $moduleId = null,
        ?string $description = null,
        ?array $oldData = null,
        ?array $newData = null,
        ?Request $request = null,
    ): AuditLog {
        // TODO: Implement log method
        return new AuditLog();
    }

    public function redact(array $data): array
    {
        // TODO: Implement redact method
        return $data;
    }
}
