-- 20260915000000_add_logbook_and_section_user_edited.sql
-- Add PKL logbook entries table and user_edited flag on generated sections

CREATE TABLE IF NOT EXISTS report_logbook_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES report_projects(id) ON DELETE CASCADE,
    entry_date DATE NOT NULL,
    activity_title TEXT NOT NULL,
    tasks_performed TEXT NOT NULL,
    tools_technologies TEXT,
    problems_encountered TEXT,
    solutions_applied TEXT,
    skills_learned TEXT,
    evidence_notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_report_logbook_report_date ON report_logbook_entries(report_id, entry_date ASC);

ALTER TABLE report_generated_sections 
ADD COLUMN IF NOT EXISTS is_user_edited BOOLEAN NOT NULL DEFAULT false;
