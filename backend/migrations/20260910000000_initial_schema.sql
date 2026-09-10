-- 001_initial_schema.sql
-- Laporin initial database schema

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Core Tables

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    is_active BOOLEAN NOT NULL DEFAULT true
);

CREATE TABLE IF NOT EXISTS oauth_identities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (provider, provider_user_id)
);

CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    revoked BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE IF NOT EXISTS email_otps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    otp_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS report_projects (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('draft','researching','research_completed','generating','generated','preview_ready','payment_pending','paid','unlocked','failed','cancelled','expired')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS report_student_infos (
    report_id UUID PRIMARY KEY REFERENCES report_projects(id) ON DELETE CASCADE,
    full_name TEXT NOT NULL,
    student_id TEXT NOT NULL,
    school TEXT NOT NULL,
    major TEXT,
    semester TEXT
);

CREATE TABLE IF NOT EXISTS report_internship_infos (
    report_id UUID PRIMARY KEY REFERENCES report_projects(id) ON DELETE CASCADE,
    company_name TEXT NOT NULL,
    company_address TEXT,
    department TEXT,
    role TEXT,
    start_date DATE,
    end_date DATE,
    supervisor_company TEXT,
    supervisor_school TEXT,
    description TEXT
);

CREATE TABLE IF NOT EXISTS research_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES report_projects(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('pending','running','succeeded','failed','cancelled')),
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    error_code TEXT,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS research_sources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id UUID NOT NULL REFERENCES research_jobs(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    title TEXT,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    confidence_score NUMERIC(3,2) CHECK (confidence_score >= 0.00 AND confidence_score <= 1.00)
);

CREATE TABLE IF NOT EXISTS report_research_facts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES report_projects(id) ON DELETE CASCADE,
    source_id UUID NOT NULL REFERENCES research_sources(id) ON DELETE CASCADE,
    claim TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS generation_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES report_projects(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('pending','running','succeeded','failed','cancelled')),
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    error_code TEXT,
    error_message TEXT,
    doc_path TEXT,
    preview_path TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS report_generated_sections (
    report_id UUID PRIMARY KEY REFERENCES report_projects(id) ON DELETE CASCADE,
    cover TEXT NOT NULL,
    introduction TEXT NOT NULL,
    company_profile TEXT NOT NULL,
    activities TEXT NOT NULL,
    conclusion TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES report_projects(id) ON DELETE CASCADE,
    status TEXT NOT NULL CHECK (status IN ('created','pending','succeeded','failed','cancelled','expired')),
    amount_cents INTEGER NOT NULL,
    provider_tx_id TEXT UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS file_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES report_projects(id) ON DELETE CASCADE,
    kind TEXT NOT NULL CHECK (kind IN ('docx','preview','temp')),
    path TEXT NOT NULL UNIQUE,
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ
);

-- Indices
CREATE INDEX IF NOT EXISTS idx_sessions_token_hash ON sessions(token_hash);
CREATE INDEX IF NOT EXISTS idx_report_user ON report_projects(user_id);
CREATE INDEX IF NOT EXISTS idx_research_job_status ON research_jobs(status);
CREATE INDEX IF NOT EXISTS idx_generation_job_status ON generation_jobs(status);
CREATE INDEX IF NOT EXISTS idx_payments_report ON payments(report_id);
CREATE INDEX IF NOT EXISTS idx_payments_provider_tx_id ON payments(provider_tx_id);
CREATE INDEX IF NOT EXISTS idx_research_sources_job ON research_sources(job_id);
CREATE INDEX IF NOT EXISTS idx_report_research_facts_report ON report_research_facts(report_id);
CREATE INDEX IF NOT EXISTS idx_file_entries_kind ON file_entries(kind);
