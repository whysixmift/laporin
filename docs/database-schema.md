# Laporin PostgreSQL Schema

## Core Tables

### users
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| email             | TEXT                | NOT NULL, UNIQUE                           |
| password_hash     | TEXT                | NOT NULL                                    |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| updated_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| is_active         | BOOLEAN             | NOT NULL, DEFAULT true                     |

### oauth_identities
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| user_id           | UUID                | NOT NULL, REFERENCES users(id) ON DELETE CASCADE |
| provider          | TEXT                | NOT NULL (e.g., "google")                |
| provider_user_id  | TEXT                | NOT NULL, UNIQUE (provider, provider_user_id) |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### sessions
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| user_id           | UUID                | NOT NULL, REFERENCES users(id) ON DELETE CASCADE |
| token_hash        | TEXT                | NOT NULL, UNIQUE (SHA-256 of session token)|
| expires_at        | TIMESTAMPTZ         | NOT NULL                                    |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| revoked           | BOOLEAN             | NOT NULL, DEFAULT false                    |

### email_otps
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| user_id           | UUID                | NOT NULL, REFERENCES users(id) ON DELETE CASCADE |
| otp_hash          | TEXT                | NOT NULL (SHA-256 of 6-digit numeric OTP)   |
| expires_at        | TIMESTAMPTZ         | NOT NULL                                    |
| attempts          | INTEGER             | NOT NULL, DEFAULT 0                        |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### password_reset_tokens
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| user_id           | UUID                | NOT NULL, REFERENCES users(id) ON DELETE CASCADE |
| token_hash        | TEXT                | NOT NULL, UNIQUE                           |
| expires_at        | TIMESTAMPTZ         | NOT NULL                                    |
| used              | BOOLEAN             | NOT NULL, DEFAULT false                    |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### report_projects
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| user_id           | UUID                | NOT NULL, REFERENCES users(id) ON DELETE CASCADE |
| title             | TEXT                | NOT NULL                                    |
| status            | TEXT                | NOT NULL, CHECK (status IN ('draft','researching','research_completed','generating','generated','preview_ready','payment_pending','paid','unlocked','failed','cancelled','expired')) |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| updated_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### report_student_infos
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| report_id         | UUID                | PRIMARY KEY, REFERENCES report_projects(id) ON DELETE CASCADE |
| full_name         | TEXT                | NOT NULL                                    |
| student_id        | TEXT                | NOT NULL                                    |
| school            | TEXT                | NOT NULL                                    |
| major             | TEXT                | NULLABLE                                    |
| semester          | TEXT                | NULLABLE                                    |

### report_internship_infos
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| report_id         | UUID                | PRIMARY KEY, REFERENCES report_projects(id) ON DELETE CASCADE |
| company_name      | TEXT                | NOT NULL                                    |
| company_address   | TEXT                | NULLABLE                                    |
| department        | TEXT                | NULLABLE                                    |
| role              | TEXT                | NULLABLE                                    |
| start_date        | DATE                | NULLABLE                                    |
| end_date          | DATE                | NULLABLE                                    |
| supervisor_company| TEXT                | NULLABLE                                    |
| supervisor_school | TEXT                | NULLABLE                                    |
| description       | TEXT                | NULLABLE                                    |

### research_jobs
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| report_id         | UUID                | NOT NULL, REFERENCES report_projects(id) ON DELETE CASCADE |
| status            | TEXT                | NOT NULL, CHECK (status IN ('pending','running','succeeded','failed','cancelled')) |
| attempts          | INTEGER             | NOT NULL, DEFAULT 0                        |
| max_attempts      | INTEGER             | NOT NULL, DEFAULT 3                        |
| started_at        | TIMESTAMPTZ         | NULLABLE                                    |
| finished_at       | TIMESTAMPTZ         | NULLABLE                                    |
| error_code        | TEXT                | NULLABLE                                    |
| error_message     | TEXT                | NULLABLE                                    |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### research_sources
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| job_id            | UUID                | NOT NULL, REFERENCES research_jobs(id) ON DELETE CASCADE |
| url               | TEXT                | NOT NULL                                    |
| title             | TEXT                | NULLABLE                                    |
| fetched_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| confidence_score  | NUMERIC(3,2)        | NULLABLE (0.00-1.00)                        |

### report_research_facts
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| report_id         | UUID                | NOT NULL, REFERENCES report_projects(id) ON DELETE CASCADE |
| source_id         | UUID                | NOT NULL, REFERENCES research_sources(id) ON DELETE CASCADE |
| claim             | TEXT                | NOT NULL                                    |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### generation_jobs
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| report_id         | UUID                | NOT NULL, REFERENCES report_projects(id) ON DELETE CASCADE |
| status            | TEXT                | NOT NULL, CHECK (status IN ('pending','running','succeeded','failed','cancelled')) |
| attempts          | INTEGER             | NOT NULL, DEFAULT 0                        |
| max_attempts      | INTEGER             | NOT NULL, DEFAULT 3                        |
| started_at        | TIMESTAMPTZ         | NULLABLE                                    |
| finished_at       | TIMESTAMPTZ         | NULLABLE                                    |
| error_code        | TEXT                | NULLABLE                                    |
| error_message     | TEXT                | NULLABLE                                    |
| doc_path          | TEXT                | NULLABLE (internal relative storage path)   |
| preview_path      | TEXT                | NULLABLE (internal relative storage path)   |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### report_generated_sections
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| report_id         | UUID                | PRIMARY KEY, REFERENCES report_projects(id) ON DELETE CASCADE |
| cover             | TEXT                | NOT NULL                                    |
| introduction      | TEXT                | NOT NULL                                    |
| company_profile   | TEXT                | NOT NULL                                    |
| activities        | TEXT                | NOT NULL                                    |
| conclusion        | TEXT                | NOT NULL                                    |
| updated_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### payments
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| report_id         | UUID                | NOT NULL, REFERENCES report_projects(id) ON DELETE CASCADE |
| status            | TEXT                | NOT NULL, CHECK (status IN ('created','pending','succeeded','failed','cancelled','expired')) |
| amount_cents      | INTEGER             | NOT NULL (e.g., 15000)                     |
| provider_tx_id    | TEXT                | NULLABLE, UNIQUE (Mayar transaction ID)     |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| updated_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |

### file_entries (internal tracking for generated & preview files)
| Column            | Type                | Constraints                                 |
|-------------------|---------------------|---------------------------------------------|
| id                | UUID                | PRIMARY KEY, DEFAULT gen_random_uuid()      |
| report_id         | UUID                | NOT NULL, REFERENCES report_projects(id) ON DELETE CASCADE |
| kind              | TEXT                | NOT NULL, CHECK (kind IN ('docx','preview','temp')) |
| path              | TEXT                | NOT NULL, UNIQUE                           |
| size_bytes        | BIGINT              | NOT NULL                                    |
| created_at        | TIMESTAMPTZ         | NOT NULL, DEFAULT now()                    |
| expires_at        | TIMESTAMPTZ         | NULLABLE (for temporary files)             |

## Indexes & Performance
- `CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);`
- `CREATE INDEX idx_report_user ON report_projects(user_id);`
- `CREATE INDEX idx_research_job_status ON research_jobs(status);`
- `CREATE INDEX idx_generation_job_status ON generation_jobs(status);`
- `CREATE INDEX idx_payments_report ON payments(report_id);`
- `CREATE INDEX idx_payments_provider_tx_id ON payments(provider_tx_id);`
- `CREATE INDEX idx_research_sources_job ON research_sources(job_id);`
- `CREATE INDEX idx_report_research_facts_report ON report_research_facts(report_id);`
- `CREATE INDEX idx_file_entries_kind ON file_entries(kind);`

## Retention & Cleanup
- Temporary files (`kind='temp'`) are deleted by a scheduled cleanup worker when `expires_at < now()`.
- Expired sessions (`expires_at < now()`) are purged daily.
- DOCX files are retained for 30 days after unlock, after which they are deleted and marked expired.
- Soft-deleted reports (`status='cancelled'` or `'expired'`) are retained for 30 days then hard-deleted.
