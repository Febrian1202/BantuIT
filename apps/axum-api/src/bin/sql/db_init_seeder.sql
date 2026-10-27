-- Initial Roles
INSERT INTO roles (id, name, description) VALUES
(1, 'administrator', 'Akses penuh seluruh sistem'),
(2, 'manager', 'Akses manajemen dan laporan'),
(3, 'technician', 'Penyelesai tiket teknis'),
(4, 'employee', 'Karyawan pemohon tiket');

-- Pinned Statuses
INSERT INTO ticket_statuses (id, name, description, is_closed, is_final) VALUES
(1, 'OPEN', 'Tiket baru, belum ada teknisi', FALSE, FALSE),
(2, 'ASSIGNED', 'Tiket telah ditugaskan ke teknisi', FALSE, FALSE),
(3, 'IN_PROGRESS', 'Tiket sedang dikerjakan', FALSE, FALSE),
(4, 'RESOLVED', 'Tiket selesai dikerjakan, menunggu konfirmasi', TRUE, FALSE),
(5, 'CLOSED', 'Tiket selesai dan terkonfirmasi (final)', TRUE, TRUE);

-- Pinned Priorities
INSERT INTO ticket_priorities (id, name, level, sla_minutes, description) VALUES
(1, 'Critical', 1, 120, 'SLA 2 jam - penanganan darurat'),
(2, 'High', 2, 240, 'SLA 4 jam - penanganan cepat'),
(3, 'Medium', 3, 480, 'SLA 8 jam - penanganan standar'),
(4, 'Low', 4, 1440, 'SLA 24 jam - penanganan santai');

-- Initial Departments
INSERT INTO departments (id, name, description) VALUES
(1, 'Teknologi Informasi', 'Divisi IT & Infrastruktur'),
(2, 'Keuangan', 'Divisi Finance & Accounting'),
(3, 'Sumber Daya Manusia', 'Divisi HR & Personalia'),
(4, 'Operasional', 'Divisi Operasional & Logistik'),
(5, 'Pemasaran', 'Divisi Marketing & Sales');

-- Initial Knowledge Categories
INSERT INTO knowledge_categories (id, name, description) VALUES
(1, 'Perangkat Keras', 'Panduan teknis hardware dan printer'),
(2, 'Perangkat Lunak', 'Panduan software, OS, dan aplikasi'),
(3, 'Jaringan & Internet', 'Panduan koneksi WiFi, LAN, dan VPN'),
(4, 'Akun & Akses', 'Panduan reset password dan email kantor'),
(5, 'Kebijakan Keamanan', 'Panduan dan SOP keamanan informasi');

-- Sinkronisasi sequence ID PostgreSQL agar auto-increment selanjutnya tidak bentrok
SELECT setval('roles_id_seq', (SELECT MAX(id) FROM roles));
SELECT setval('ticket_statuses_id_seq', (SELECT MAX(id) FROM ticket_statuses));
SELECT setval('ticket_priorities_id_seq', (SELECT MAX(id) FROM ticket_priorities));
SELECT setval('departments_id_seq', (SELECT MAX(id) FROM departments));
SELECT setval('knowledge_categories_id_seq', (SELECT MAX(id) FROM knowledge_categories));
