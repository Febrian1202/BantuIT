-- Add migration script here
-- EKSTENSI POSTGRESQL (Untuk fallback gen_random_uuid jika diperlukan)
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- DEDICATED SEQUENCES UNTUK DISPLAY NUMBERING
CREATE SEQUENCE IF NOT EXISTS ticket_number_seq START 1;
CREATE SEQUENCE IF NOT EXISTS employee_number_seq START 1;
