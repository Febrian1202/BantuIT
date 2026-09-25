<?php

// @formatter:off
// phpcs:ignoreFile
/**
 * A helper file for your Eloquent Models
 * Copy the phpDocs from this file to the correct Model,
 * And remove them from this file, to prevent double declarations.
 *
 * @author Barry vd. Heuvel <barryvdh@gmail.com>
 */


namespace App\Models{
/**
 * @property int $id
 * @property string $asset_tag
 * @property string $name
 * @property string $category
 * @property string|null $brand
 * @property string|null $model
 * @property string|null $serial_number
 * @property \Illuminate\Support\Carbon|null $purchase_date
 * @property \App\Enums\AssetStatus $status
 * @property string|null $notes
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \App\Models\AssetAssignment|null $activeAssignment
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\AssetAssignment> $assignments
 * @property-read int|null $assignments_count
 * @property-read \App\Models\User|null $currentHolder
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\AssetHistory> $histories
 * @property-read int|null $histories_count
 * @method static \Database\Factories\AssetFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereAssetTag($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereBrand($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereCategory($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereModel($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereNotes($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset wherePurchaseDate($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereSerialNumber($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereStatus($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Asset withoutTrashed()
 */
	class Asset extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $asset_id
 * @property int $user_id
 * @property \Illuminate\Support\Carbon $assigned_at
 * @property \Illuminate\Support\Carbon|null $released_at
 * @property string|null $notes
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \App\Models\Asset|null $asset
 * @property-read \App\Models\User|null $user
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment active()
 * @method static \Database\Factories\AssetAssignmentFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereAssetId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereAssignedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereNotes($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereReleasedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment whereUserId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetAssignment withoutTrashed()
 */
	class AssetAssignment extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $asset_id
 * @property string $action
 * @property string|null $description
 * @property \Illuminate\Support\Carbon $action_at
 * @property \Illuminate\Support\Carbon $created_at
 * @property-read \App\Models\Asset|null $asset
 * @method static \Database\Factories\AssetHistoryFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory whereAction($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory whereActionAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory whereAssetId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AssetHistory whereId($value)
 */
	class AssetHistory extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int|null $user_id
 * @property string $action
 * @property string $module
 * @property int|null $module_id
 * @property string|null $description
 * @property array<array-key, mixed>|null $old_data
 * @property array<array-key, mixed>|null $new_data
 * @property string|null $ip_address
 * @property string|null $user_agent
 * @property \Illuminate\Support\Carbon $created_at
 * @property-read \App\Models\User|null $user
 * @method static \Database\Factories\AuditLogFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereAction($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereIpAddress($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereModule($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereModuleId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereNewData($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereOldData($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereUserAgent($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|AuditLog whereUserId($value)
 */
	class AuditLog extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $name
 * @property string|null $description
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\User> $users
 * @property-read int|null $users_count
 * @method static \Database\Factories\DepartmentFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Department withoutTrashed()
 */
	class Department extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $user_id
 * @property string $employee_code
 * @property string|null $phone
 * @property string|null $position
 * @property \Illuminate\Support\Carbon|null $hire_date
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \App\Models\User|null $user
 * @method static \Database\Factories\EmployeeProfileFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereEmployeeCode($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereHireDate($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile wherePhone($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile wherePosition($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile whereUserId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|EmployeeProfile withoutTrashed()
 */
	class EmployeeProfile extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $category_id
 * @property int $author_id
 * @property string $title
 * @property string $slug
 * @property string $content
 * @property \App\Enums\ArticleStatus $status
 * @property int $view_count
 * @property \Illuminate\Support\Carbon|null $published_at
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \App\Models\User|null $author
 * @property-read \App\Models\KnowledgeCategory|null $category
 * @method static \Database\Factories\KnowledgeArticleFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereAuthorId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereCategoryId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereContent($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle wherePublishedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereSlug($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereStatus($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereTitle($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle whereViewCount($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeArticle withoutTrashed()
 */
	class KnowledgeArticle extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $name
 * @property string|null $description
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\KnowledgeArticle> $articles
 * @property-read int|null $articles_count
 * @method static \Database\Factories\KnowledgeCategoryFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|KnowledgeCategory withoutTrashed()
 */
	class KnowledgeCategory extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $user_id
 * @property string $type
 * @property array<array-key, mixed> $data
 * @property bool $is_read
 * @property \Illuminate\Support\Carbon|null $read_at
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property-read \App\Models\User|null $user
 * @method static \Database\Factories\NotificationFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereData($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereIsRead($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereReadAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereType($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Notification whereUserId($value)
 */
	class Notification extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $name
 * @property string|null $description
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\User> $users
 * @property-read int|null $users_count
 * @method static \Database\Factories\RoleFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Role withoutTrashed()
 */
	class Role extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $ticket_number
 * @property string $title
 * @property string $description
 * @property int $category_id
 * @property int $priority_id
 * @property int $status_id
 * @property int $reporter_id
 * @property int|null $technician_id
 * @property int|null $department_id
 * @property int|null $asset_id
 * @property int $sla_duration_minutes
 * @property \Illuminate\Support\Carbon $sla_deadline
 * @property \Illuminate\Support\Carbon|null $resolved_at
 * @property \Illuminate\Support\Carbon|null $closed_at
 * @property bool $sla_breached
 * @property \Illuminate\Support\Carbon|null $sla_breached_at
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \App\Models\Asset|null $asset
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\TicketAttachment> $attachments
 * @property-read int|null $attachments_count
 * @property-read \App\Models\TicketCategory|null $category
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\TicketComment> $comments
 * @property-read int|null $comments_count
 * @property-read \App\Models\Department|null $department
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\TicketHistory> $histories
 * @property-read int|null $histories_count
 * @property-read \App\Models\TicketPriority|null $priority
 * @property-read \App\Models\User|null $reporter
 * @property-read \App\Models\TicketStatus|null $status
 * @property-read \App\Models\User|null $technician
 * @method static \Database\Factories\TicketFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereAssetId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereCategoryId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereClosedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereDepartmentId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket wherePriorityId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereReporterId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereResolvedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereSlaBreached($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereSlaBreachedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereSlaDeadline($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereSlaDurationMinutes($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereStatusId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereTechnicianId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereTicketNumber($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereTitle($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|Ticket withoutTrashed()
 */
	class Ticket extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $ticket_id
 * @property int $uploaded_by
 * @property string $original_filename
 * @property string $stored_filename
 * @property string $mime_type
 * @property int $file_size
 * @property string $storage_path
 * @property \Illuminate\Support\Carbon $created_at
 * @property-read \App\Models\Ticket|null $ticket
 * @property-read \App\Models\User|null $uploader
 * @method static \Database\Factories\TicketAttachmentFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereFileSize($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereMimeType($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereOriginalFilename($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereStoragePath($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereStoredFilename($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereTicketId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketAttachment whereUploadedBy($value)
 */
	class TicketAttachment extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $name
 * @property string|null $description
 * @property int|null $parent_id
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, TicketCategory> $children
 * @property-read int|null $children_count
 * @property-read TicketCategory|null $parent
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\Ticket> $tickets
 * @property-read int|null $tickets_count
 * @method static \Database\Factories\TicketCategoryFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereParentId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketCategory withoutTrashed()
 */
	class TicketCategory extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $ticket_id
 * @property int $user_id
 * @property string $body
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \App\Models\Ticket|null $ticket
 * @property-read \App\Models\User|null $user
 * @method static \Database\Factories\TicketCommentFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereBody($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereTicketId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment whereUserId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketComment withoutTrashed()
 */
	class TicketComment extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property int $ticket_id
 * @property int $user_id
 * @property string $field_changed
 * @property string|null $old_value
 * @property string|null $new_value
 * @property \Illuminate\Support\Carbon $created_at
 * @property-read \App\Models\Ticket|null $ticket
 * @property-read \App\Models\User|null $user
 * @method static \Database\Factories\TicketHistoryFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereFieldChanged($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereNewValue($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereOldValue($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereTicketId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketHistory whereUserId($value)
 */
	class TicketHistory extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $name
 * @property int $level
 * @property int $sla_minutes
 * @property string|null $description
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\Ticket> $tickets
 * @property-read int|null $tickets_count
 * @method static \Database\Factories\TicketPriorityFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereLevel($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereSlaMinutes($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketPriority withoutTrashed()
 */
	class TicketPriority extends \Eloquent {}
}

namespace App\Models{
/**
 * @property int $id
 * @property string $name
 * @property string|null $description
 * @property bool $is_closed
 * @property bool $is_final
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\Ticket> $tickets
 * @property-read int|null $tickets_count
 * @method static \Database\Factories\TicketStatusFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereDescription($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereIsClosed($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereIsFinal($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|TicketStatus withoutTrashed()
 */
	class TicketStatus extends \Eloquent {}
}

namespace App\Models{
/**
 * @method PersonalAccessToken|null currentAccessToken()
 * @property int $id
 * @property int $role_id
 * @property int|null $department_id
 * @property string $email
 * @property string $password
 * @property string $full_name
 * @property string $status
 * @property bool $must_change_password
 * @property \Illuminate\Support\Carbon|null $last_login_at
 * @property \Illuminate\Support\Carbon|null $created_at
 * @property \Illuminate\Support\Carbon|null $updated_at
 * @property \Illuminate\Support\Carbon|null $deleted_at
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\AssetAssignment> $activeAssignments
 * @property-read int|null $active_assignments_count
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \App\Models\AssetAssignment> $assetAssignments
 * @property-read int|null $asset_assignments_count
 * @property-read \App\Models\Department|null $department
 * @property-read \App\Models\EmployeeProfile|null $employeeProfile
 * @property-read \App\Models\Role|null $role
 * @property-read \Illuminate\Database\Eloquent\Collection<int, \Laravel\Sanctum\PersonalAccessToken> $tokens
 * @property-read int|null $tokens_count
 * @method static \Database\Factories\UserFactory factory($count = null, $state = [])
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User newModelQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User newQuery()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User onlyTrashed()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User query()
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereCreatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereDeletedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereDepartmentId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereEmail($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereFullName($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereLastLoginAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereMustChangePassword($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User wherePassword($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereRoleId($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereStatus($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User whereUpdatedAt($value)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User withTrashed(bool $withTrashed = true)
 * @method static \Illuminate\Database\Eloquent\Builder<static>|User withoutTrashed()
 */
	class User extends \Eloquent {}
}

