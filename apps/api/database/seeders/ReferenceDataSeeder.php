<?php

namespace Database\Seeders;

use App\Enums\RoleName;
use App\Enums\TicketStatusName;
use App\Models\Department;
use App\Models\KnowledgeCategory;
use App\Models\Role;
use App\Models\TicketCategory;
use App\Models\TicketPriority;
use App\Models\TicketStatus;
use Illuminate\Database\Seeder;

class ReferenceDataSeeder extends Seeder
{
    /**
     * Run the database seeds.
     */
    public function run(): void
    {
        // Role
        $roles = [
            1 => [
                'name' => RoleName::Admin->value,
                'description' => 'Administrator Sistem',
            ],
            2 => [
                'name' => RoleName::Manager->value,
                'description' => 'Manager Sistem',
            ],
            3 => [
                'name' => RoleName::Technician->value,
                'description' => 'Teknisi Sistem',
            ],
            4 => [
                'name' => RoleName::Employee->value,
                'description' => 'Karyawan Sistem',
            ],
        ];
        foreach ($roles as $id => $data) {
            Role::updateOrCreate(['id' => $id], $data);
        }

        // Status Ticket
        $statuses = [
            1 => [
                'name' => TicketStatusName::Open->value,
                'description' => 'Tiket baru dibuat dan belum ditangani oleh teknisi',
                'is_closed' => false,
                'is_final' => false,
            ],
            2 => [
                'name' => TicketStatusName::Assigned->value,
                'description' => 'Ticket ditugaskan ke teknisi',
                'is_closed' => false,
                'is_final' => false,
            ],
            3 => [
                'name' => TicketStatusName::InProgress->value,
                'description' => 'Ticket sedang dalam proses ditangani oleh teknisi',
                'is_closed' => false,
                'is_final' => false,
            ],
            4 => [
                'name' => TicketStatusName::Resolved->value,
                'description' => 'Ticket telah ditangani dan diselesaikan oleh teknisi',
                'is_closed' => true,
                'is_final' => false,
            ],
            5 => [
                'name' => TicketStatusName::Closed->value,
                'description' => 'Ticket sudah selesai dan ditutup secara permanen',
                'is_closed' => true,
                'is_final' => true,
            ],
        ];
        foreach ($statuses as $id => $data) {
            TicketStatus::updateOrCreate(['id' => $id], $data);
        }

        // Ticket Priority
        $priorities = [
            1 => [
                'name' => 'Critical',
                'level' => 1,
                'sla_minutes' => 120,
                'description' => 'Gangguan sistem di seluruh perusahaan (2 jam)',
            ],
            2 => [
                'name' => 'High',
                'level' => 2,
                'sla_minutes' => 240,
                'description' => 'Karyawan tidak dapat bekerja (4 jam)',
            ],
            3 => [
                'name' => 'Medium',
                'level' => 3,
                'sla_minutes' => 480,
                'description' => 'Masalah aplikasi non-kritis (8 jam)',
            ],
            4 => [
                'name' => 'Low',
                'level' => 4,
                'sla_minutes' => 1440,
                'description' => 'Permintaan biasa (24 jam)',
            ],
        ];
        foreach ($priorities as $id => $data) {
            TicketPriority::updateOrCreate(['id' => $id], $data);
        }

        // Departemen
        $departments = [
            'Information Technology' => 'Mengelola teknologi, infrastruktur, dan dukungan TI perusahaan',
            'Finance & Accounting' => 'Mengelola keuangan perusahaan, penggajian, akuntansi, dan audit',
            'Human Resources' => 'Operasional SDM, rekrutmen, onboarding, dan hubungan karyawan',
            'Operations' => 'Logistik operasional inti, fasilitas, dan rantai pasok',
            'Marketing & Sales' => 'Akuisisi pelanggan, kampanye pemasaran, dan pengembangan bisnis',
        ];
        foreach ($departments as $name => $description) {
            Department::updateOrCreate(
                ['name' => $name],
                ['description' => $description],
            );
        }

        // Kategori Tiket
        $categoryTree = [
            'Hardware' => [
                'description' => 'Perangkat keras komputer fisik dan aksesori',
                'children' => [
                    'Laptop' => 'Perangkat laptop dan masalah layar/baterai',
                    'Desktop' => 'Unit PC desktop dan tower',
                    'Monitor' => 'Monitor eksternal dan adaptor tampilan',
                    'Printer' => 'Printer/pemindai jaringan dan kantor',
                    'Peripheral' => 'Papan ketik, tetikus, kamera web, dock, pemutar jemala',
                ],
            ],
            'Software' => [
                'description' => 'Sistem operasi dan perangkat lunak aplikasi',
                'children' => [
                    'Operating System' => 'Masalah sistem operasi Windows, macOS, atau Linux',
                    'Microsoft Office' => 'Office 365, Word, Excel, Teams, Outlook',
                    'Internal Application' => 'JarvisOps dan aplikasi bisnis internal',
                    'Installation Request' => 'Permintaan instalasi perangkat lunak baru dan lisensi',
                ],
            ],
            'Network' => [
                'description' => 'Konektivitas jaringan dan akses internet',
                'children' => [
                    'Wi-Fi' => 'Masalah koneksi jaringan nirkabel kantor',
                    'Internet' => 'Jaringan LAN kabel dan aksesibilitas internet umum',
                    'VPN' => 'Akses jarak jauh dan koneksi VPN yang aman',
                    'DNS' => 'Resolusi nama domain dan masalah rute jaringan',
                ],
            ],
            'Account' => [
                'description' => 'Identitas pengguna, log masuk, dan hak akses',
                'children' => [
                    'Password' => 'Atur ulang kata sandi dan penanganan akun terkunci',
                    'Account Access' => 'Pembuatan, pembukaan blokir, dan penyediaan akun',
                    'Permission' => 'Penyesuaian berbagi berkas, grup, dan hak akses khusus',
                ],
            ],
            'Other' => [
                'description' => 'Permintaan TI umum dan rupa-rupa',
                'children' => [
                    'Other IT Request' => 'Permintaan yang tidak tercakup dalam kategori lain',
                ],
            ],
        ];

        foreach ($categoryTree as $parentName => $config) {
            $parent = TicketCategory::updateOrCreate(
                ['name' => $parentName],
                ['description' => $config['description'], 'parent_id' => null],
            );

            foreach ($config['children'] as $childName => $childDesc) {
                TicketCategory::updateOrCreate(
                    ['name' => $childName],
                    ['description' => $childDesc, 'parent_id' => $parent->id],
                );
            }
        }

        // Kategori Knowledge
        $knowledgeCategories = [
            'Hardware Troubleshooting' => 'Panduan pengaturan perangkat keras, penanganan masalah, dan aksesori',
            'Network & Connectivity' => 'Panduan Wi-Fi, Ethernet, VPN, dan konfigurasi jaringan',
            'Software & OS' => 'Panduan sistem operasi, Office 365, dan perangkat lunak berlisensi',
            'Access & Account' => 'Panduan keamanan akun, kata sandi, dan hak akses layanan',
            'Office Facility' => 'Panduan ruang rapat, pencetakan, dan fasilitas TI kantor',
        ];
        foreach ($knowledgeCategories as $name => $description) {
            KnowledgeCategory::updateOrCreate(
                ['name' => $name],
                ['description' => $description],
            );
        }
    }
}
