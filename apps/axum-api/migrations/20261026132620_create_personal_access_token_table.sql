-- Add migration script here

-- Personal Access Tokens (Sanctum Parity -> UUID)
CREATE TABLE personal_access_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tokenable_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL DEFAULT 'auth_token',
    token VARCHAR(64) NOT NULL UNIQUE, -- Menyimpan hash SHA-256 dari plain token
    abilities TEXT NOT NULL DEFAULT '["*"]',
    last_used_at TIMESTAMPTZ NULL,
    expires_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
