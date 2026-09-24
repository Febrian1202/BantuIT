<?php

namespace App\Authorization;

use App\Enums\RoleName;

/**
 * Sumber kebenaran tunggal (single source of truth) untuk otorisasi berbasis peran (RBAC).
 *
 * Kemampuan (*ability*) berbasis peran ditentukan murni dari peran pengguna dan
 * didaftarkan sebagai Gate. Kemampuan yang bergantung pada kepemilikan terdaftar
 * secara terpisah (getPolicyAbilities) dan ditegakkan oleh Policy.
 * Method adminGateExceptions mendefinisikan kemampuan di mana aturan penanganan
 * langsung (blanket Gate::before) untuk admin tidak boleh memotong pengecekan otorisasi.
 */
class AbilityMatrix
{
    /**
     * Kemampuan berbasis peran yang dipetakan ke peran yang diizinkan mengoperasikannya.
     *
     * @var array<string, list<RoleName>>
     */
    private const ROLE_ABILITIES = [
        // Autentikasi & profil
        'auth.login' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'auth.logout' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'profile.view-own' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'profile.change-password' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],

        // Dashboard
        'dashboard.employee' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'dashboard.technician' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
        ],
        'dashboard.manager' => [RoleName::Admin, RoleName::Manager],
        'dashboard.admin' => [RoleName::Admin],
        'analytics.technician-performance' => [
            RoleName::Admin,
            RoleName::Manager,
        ],

        // Administrasi
        'user.viewAny' => [RoleName::Admin],
        'user.view' => [RoleName::Admin],
        'user.create' => [RoleName::Admin],
        'user.update' => [RoleName::Admin],
        'user.delete' => [RoleName::Admin],
        'user.activate' => [RoleName::Admin],
        'user.deactivate' => [RoleName::Admin],
        'user.reset-password' => [RoleName::Admin],
        'user.lookup' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
        ],
        'technician.list' => [RoleName::Admin, RoleName::Manager],
        'department.viewAny' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'department.manage' => [RoleName::Admin],
        'ticket-category.viewAny' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'ticket-category.manage' => [RoleName::Admin],
        'ticket-priority.viewAny' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'ticket-priority.manage' => [RoleName::Admin],
        'ticket-status.viewAny' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'knowledge-category.viewAny' => [
            RoleName::Admin,
            RoleName::Manager,
            RoleName::Technician,
            RoleName::Employee,
        ],
        'knowledge-category.manage' => [RoleName::Admin],
        'audit-log.viewAny' => [RoleName::Admin, RoleName::Manager],
        'audit-log.view' => [RoleName::Admin, RoleName::Manager],
    ];

    /**
     * Kemampuan yang bergantung pada kepemilikan (milik sendiri / ditugaskan / terlingkup).
     * Ditegakkan oleh Policy (TicketPolicy, AttachmentPolicy, AssetPolicy,
     * ArticlePolicy, NotificationPolicy).
     *
     * @var list<string>
     */
    private const POLICY_ABILITIES = [
        // Ticket — TicketPolicy
        'ticket.viewAny',
        'ticket.view',
        'ticket.create',
        'ticket.update',
        'ticket.delete',
        'ticket.assign',
        'ticket.unassign',
        'ticket.changeStatus',
        'ticket.selfAssign',
        'ticket.changePriority',
        'ticket.comment',
        'ticket.viewHistory',
        'ticket.attach',
        // Attachment — AttachmentPolicy
        'attachment.view',
        'attachment.download',
        'attachment.create',
        'attachment.delete',
        // Asset — AssetPolicy
        'asset.viewAny',
        'asset.view',
        'asset.viewOwn',
        'asset.viewAssignable',
        'asset.create',
        'asset.update',
        'asset.delete',
        'asset.assign',
        'asset.release',
        'asset.viewHistory',
        // Knowledge base — ArticlePolicy
        'article.viewAny',
        'article.view',
        'article.create',
        'article.update',
        'article.publish',
        'article.unpublish',
        'article.delete',
        // Notification — NotificationPolicy
        'notification.viewAny',
        'notification.markAsRead',
        'notification.markAllAsRead',
    ];

    /**
     * Kemampuan yang dikecualikan dari pemotongan otorisasi langsung admin (Gate::before):
     * Notifikasi diisolasi per pengguna; admin tidak dapat menonaktifkan atau menghapus akunnya sendiri.
     *
     * @var list<string>
     */
    private const ADMIN_GATE_EXCEPTIONS = [
        'notification.viewAny',
        'notification.markAsRead',
        'notification.markAllAsRead',
        'user.deactivate',
        'user.delete',
    ];

    /**
     * Mengambil matriks kemampuan berbasis peran (ability => peran yang diizinkan).
     *
     * @return array<string, list<RoleName>>
     */
    public static function getRoleAbilities(): array
    {
        return self::ROLE_ABILITIES;
    }

    /**
     * Mengambil daftar kemampuan yang bergantung pada kepemilikan (ditegakkan oleh Policy).
     *
     * @return list<string>
     */
    public static function getPolicyAbilities(): array
    {
        return self::POLICY_ABILITIES;
    }

    /**
     * Mengambil daftar nama kemampuan yang dikecualikan dari aturan Gate::before admin.
     *
     * @return list<string>
     */
    public static function adminGateExceptions(): array
    {
        return self::ADMIN_GATE_EXCEPTIONS;
    }

    /**
     * Menentukan daftar kemampuan (*abilities*) yang diizinkan untuk suatu peran.
     *
     * @return list<string>
     */
    public static function permissionsFor(RoleName $role): array
    {
        $permissions = [];

        foreach (self::ROLE_ABILITIES as $ability => $roles) {
            if (in_array($role, $roles, true)) {
                $permissions[] = $ability;
            }
        }

        return $permissions;
    }
}
