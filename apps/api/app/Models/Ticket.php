<?php

namespace App\Models;

use App\Models\Concerns\SerializesDatesAsIso8601;
use App\Policies\TicketPolicy;
use Illuminate\Database\Eloquent\Attributes\UsePolicy;
use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\SoftDeletes;

#[UsePolicy(TicketPolicy::class)]
class Ticket extends Model
{
    use HasFactory, SerializesDatesAsIso8601, SoftDeletes;

    /**
     * Atribut yang dapat diisi secara massal (fillable)
     */
    protected $fillable = [
        "ticket_number",
        "title",
        "description",
        "category_id",
        "priority_id",
        "status_id",
        "reporter_id",
        "technician_id",
        "department_id",
        "asset_id",
        "sla_duration_minutes",
        "sla_deadline",
        "resolved_at",
        "closed_at",
        "sla_breached",
        "sla_breached_at",
    ];

    /**
     * Konversi tipe data untuk atribut yang di-cast
     */
    protected function casts(): array
    {
        return [
            "sla_duration_minutes" => "integer",
            "sla_deadline" => "datetime",
            "resolved_at" => "datetime",
            "closed_at" => "datetime",
            "sla_breached" => "boolean",
            "sla_breached_at" => "datetime",
        ];
    }

    /**
     * Relasi ke model TicketCategory
     */
    public function category(): BelongsTo
    {
        return $this->belongsTo(TicketCategory::class, "category_id");
    }

    /**
     * Relasi ke model TicketPriority
     */
    public function priority(): BelongsTo
    {
        return $this->belongsTo(TicketPriority::class, "priority_id");
    }

    /**
     * Relasi ke model TicketStatus
     */
    public function status(): BelongsTo
    {
        return $this->belongsTo(TicketStatus::class, "status_id");
    }

    /**
     * Relasi ke model User (reporter)
     */
    public function reporter(): BelongsTo
    {
        return $this->belongsTo(User::class, "reporter_id");
    }

    /**
     * Relasi ke model User (technician)
     */
    public function technician(): BelongsTo
    {
        return $this->belongsTo(User::class, "technician_id");
    }

    /**
     * Relasi ke model Department
     */
    public function department(): BelongsTo
    {
        return $this->belongsTo(Department::class, "department_id");
    }

    /**
     * Relasi ke model Asset
     */
    public function asset(): BelongsTo
    {
        return $this->belongsTo(Asset::class, "asset_id");
    }

    /**
     * Relasi ke model TicketComment
     */
    public function comments(): HasMany
    {
        return $this->hasMany(TicketComment::class);
    }

    /**
     * Relasi ke model TicketAttachment
     */
    public function attachments(): HasMany
    {
        return $this->hasMany(TicketAttachment::class);
    }

    /**
     * Relasi ke model TicketHistory
     */
    public function histories(): HasMany
    {
        return $this->hasMany(TicketHistory::class);
    }
}
