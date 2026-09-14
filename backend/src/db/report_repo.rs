use crate::domain::report::{
    AdminReportSummary, GeneratedSections, InternshipInfo, Report, ReportCreate, ReportUpdate,
    ResearchFact, ResearchSource, StudentInfo,
};
use chrono::{DateTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::{Pool, Postgres};
use std::str::FromStr;
use uuid::Uuid;

pub struct ReportRepo;

#[derive(sqlx::FromRow)]
struct ReportProjectRow {
    id: Uuid,
    title: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ReportRepo {
    pub async fn create_report(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        req: ReportCreate,
    ) -> Result<Report, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let report_id = Uuid::new_v4();
        let now = Utc::now();

        // 1. Insert report_projects
        sqlx::query!(
            r#"
            INSERT INTO report_projects (id, user_id, title, status, created_at, updated_at)
            VALUES ($1, $2, $3, 'draft', $4, $4)
            "#,
            report_id,
            user_id,
            req.title,
            now
        )
        .execute(&mut *tx)
        .await?;

        // 2. Insert student info
        sqlx::query!(
            r#"
            INSERT INTO report_student_infos (report_id, full_name, student_id, school, major, semester)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            report_id,
            req.student.full_name,
            req.student.student_id,
            req.student.school,
            req.student.major,
            req.student.semester
        )
        .execute(&mut *tx)
        .await?;

        // 3. Insert internship info
        sqlx::query!(
            r#"
            INSERT INTO report_internship_infos (
                report_id, company_name, company_address, department, role,
                start_date, end_date, supervisor_company, supervisor_school, description
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            report_id,
            req.internship.company_name,
            req.internship.company_address,
            req.internship.department,
            req.internship.role,
            req.internship.start_date,
            req.internship.end_date,
            req.internship.company_supervisor,
            req.internship.school_supervisor,
            req.internship.description
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(Report {
            id: report_id,
            title: req.title,
            status: "draft".to_string(),
            created_at: now,
            updated_at: Some(now),
            student: Some(req.student),
            internship: Some(req.internship),
            research_facts: Some(vec![]),
            generated_sections: None,
            generated_doc_path: None,
        })
    }

    pub async fn get_report_by_id(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<Option<(Report, Uuid)>, sqlx::Error> {
        let project = sqlx::query!(
            r#"
            SELECT id, user_id, title, status, created_at, updated_at
            FROM report_projects
            WHERE id = $1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        let project = match project {
            Some(p) => p,
            None => return Ok(None),
        };

        // Fetch student info
        let student = sqlx::query_as!(
            StudentInfo,
            r#"
            SELECT full_name, student_id, school, major, semester
            FROM report_student_infos
            WHERE report_id = $1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        // Fetch internship info
        let internship = sqlx::query_as!(
            InternshipInfo,
            r#"
            SELECT company_name, company_address, department, role,
                   start_date, end_date,
                   supervisor_company as company_supervisor,
                   supervisor_school as school_supervisor,
                   description
            FROM report_internship_infos
            WHERE report_id = $1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        // Fetch facts with source
        let facts_rows = sqlx::query!(
            r#"
            SELECT rf.claim, rs.url, rs.title, rs.fetched_at, rs.confidence_score
            FROM report_research_facts rf
            JOIN research_sources rs ON rf.source_id = rs.id
            WHERE rf.report_id = $1
            "#,
            report_id
        )
        .fetch_all(pool)
        .await?;

        let mut facts = Vec::new();
        for row in facts_rows {
            let conf_f64 = row
                .confidence_score
                .and_then(|c| f64::from_str(&c.to_string()).ok());

            facts.push(ResearchFact {
                claim: row.claim,
                sources: vec![ResearchSource {
                    url: row.url,
                    title: row.title,
                    fetched_at: row.fetched_at,
                    confidence: conf_f64,
                }],
            });
        }

        // Fetch generated sections
        let sections = sqlx::query_as!(
            GeneratedSections,
            r#"
            SELECT cover, introduction, company_profile, activities, conclusion
            FROM report_generated_sections
            WHERE report_id = $1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        // Fetch latest doc path if any
        let file_row = sqlx::query!(
            r#"
            SELECT path FROM file_entries
            WHERE report_id = $1 AND kind = 'docx'
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            report_id
        )
        .fetch_optional(pool)
        .await?;

        let report = Report {
            id: project.id,
            title: project.title,
            status: project.status,
            created_at: project.created_at,
            updated_at: Some(project.updated_at),
            student,
            internship,
            research_facts: Some(facts),
            generated_sections: sections,
            generated_doc_path: file_row.map(|f| f.path),
        };

        Ok(Some((report, project.user_id)))
    }

    pub async fn list_reports_by_user(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        status_filter: Option<&str>,
    ) -> Result<Vec<Report>, sqlx::Error> {
        let rows: Vec<ReportProjectRow> = if let Some(st) = status_filter {
            sqlx::query_as!(
                ReportProjectRow,
                r#"
                SELECT id, title, status, created_at, updated_at
                FROM report_projects
                WHERE user_id = $1 AND status = $2
                ORDER BY created_at DESC
                "#,
                user_id,
                st
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                ReportProjectRow,
                r#"
                SELECT id, title, status, created_at, updated_at
                FROM report_projects
                WHERE user_id = $1
                ORDER BY created_at DESC
                "#,
                user_id
            )
            .fetch_all(pool)
            .await?
        };

        let mut list = Vec::new();
        for r in rows {
            let (report, _) = match Self::get_report_by_id(pool, r.id).await? {
                Some(res) => res,
                None => continue,
            };
            list.push(report);
        }

        Ok(list)
    }

    pub async fn update_report(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        update: ReportUpdate,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        if let Some(student) = update.student {
            sqlx::query!(
                r#"
                UPDATE report_student_infos
                SET full_name = $2, student_id = $3, school = $4, major = $5, semester = $6
                WHERE report_id = $1
                "#,
                report_id,
                student.full_name,
                student.student_id,
                student.school,
                student.major,
                student.semester
            )
            .execute(&mut *tx)
            .await?;
        }

        if let Some(internship) = update.internship {
            sqlx::query!(
                r#"
                UPDATE report_internship_infos
                SET company_name = $2, company_address = $3, department = $4, role = $5,
                    start_date = $6, end_date = $7, supervisor_company = $8, supervisor_school = $9, description = $10
                WHERE report_id = $1
                "#,
                report_id,
                internship.company_name,
                internship.company_address,
                internship.department,
                internship.role,
                internship.start_date,
                internship.end_date,
                internship.company_supervisor,
                internship.school_supervisor,
                internship.description
            )
            .execute(&mut *tx)
            .await?;
        }

        sqlx::query!(
            r#"
            UPDATE report_projects
            SET updated_at = now()
            WHERE id = $1
            "#,
            report_id
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn update_status(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE report_projects
            SET status = $2, updated_at = now()
            WHERE id = $1
            "#,
            report_id,
            status
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn save_research_facts(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        job_id: Uuid,
        facts: &[ResearchFact],
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        for fact in facts {
            for src in &fact.sources {
                let source_id = Uuid::new_v4();
                let conf_dec = src
                    .confidence
                    .and_then(|c| BigDecimal::from_str(&format!("{:.2}", c)).ok());

                sqlx::query!(
                    r#"
                    INSERT INTO research_sources (id, job_id, url, title, fetched_at, confidence_score)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    "#,
                    source_id,
                    job_id,
                    src.url,
                    src.title,
                    src.fetched_at,
                    conf_dec
                )
                .execute(&mut *tx)
                .await?;

                sqlx::query!(
                    r#"
                    INSERT INTO report_research_facts (id, report_id, source_id, claim, created_at)
                    VALUES ($1, $2, $3, $4, now())
                    "#,
                    Uuid::new_v4(),
                    report_id,
                    source_id,
                    fact.claim
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn save_generated_sections(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        sections: &GeneratedSections,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO report_generated_sections (report_id, cover, introduction, company_profile, activities, conclusion, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, now())
            ON CONFLICT (report_id) DO UPDATE
            SET cover = EXCLUDED.cover,
                introduction = EXCLUDED.introduction,
                company_profile = EXCLUDED.company_profile,
                activities = EXCLUDED.activities,
                conclusion = EXCLUDED.conclusion,
                updated_at = now()
            "#,
            report_id,
            sections.cover,
            sections.introduction,
            sections.company_profile,
            sections.activities,
            sections.conclusion
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn save_file_entry(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        kind: &str,
        path: &str,
        size_bytes: i64,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO file_entries (id, report_id, kind, path, size_bytes, created_at)
            VALUES ($1, $2, $3, $4, $5, now())
            ON CONFLICT (path) DO UPDATE
            SET size_bytes = EXCLUDED.size_bytes, created_at = now()
            "#,
            id,
            report_id,
            kind,
            path,
            size_bytes
        )
        .execute(pool)
        .await?;

        Ok(id)
    }

    pub async fn get_latest_file(
        pool: &Pool<Postgres>,
        report_id: Uuid,
        kind: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT path FROM file_entries
            WHERE report_id = $1 AND kind = $2
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            report_id,
            kind
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.path))
    }

    pub async fn list_all_reports_admin(
        pool: &Pool<Postgres>,
        limit: i64,
        offset: i64,
        status: Option<&str>,
        search: Option<&str>,
    ) -> Result<Vec<AdminReportSummary>, sqlx::Error> {
        let pattern = search.map(|s| format!("%{}%", s.to_lowercase()));
        let rows = sqlx::query!(
            r#"
            SELECT rp.id, rp.user_id, u.email as user_email, rp.title, rp.status,
                   rsi.full_name as student_name,
                   rii.company_name as company_name,
                   rp.created_at, rp.updated_at
            FROM report_projects rp
            JOIN users u ON u.id = rp.user_id
            LEFT JOIN report_student_infos rsi ON rsi.report_id = rp.id
            LEFT JOIN report_internship_infos rii ON rii.report_id = rp.id
            WHERE ($1::text IS NULL OR rp.status = $1)
              AND ($2::text IS NULL OR rp.title ILIKE $2 OR u.email ILIKE $2 OR rsi.full_name ILIKE $2 OR rii.company_name ILIKE $2)
            ORDER BY rp.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
            status,
            pattern,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        let reports = rows
            .into_iter()
            .map(|r| AdminReportSummary {
                id: r.id,
                user_id: r.user_id,
                user_email: r.user_email,
                title: r.title,
                status: r.status,
                student_name: Some(r.student_name),
                company_name: Some(r.company_name),
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect();


        Ok(reports)
    }

    pub async fn count_all_reports_admin(
        pool: &Pool<Postgres>,
        status: Option<&str>,
        search: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let pattern = search.map(|s| format!("%{}%", s.to_lowercase()));
        let row = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM report_projects rp
            JOIN users u ON u.id = rp.user_id
            LEFT JOIN report_student_infos rsi ON rsi.report_id = rp.id
            LEFT JOIN report_internship_infos rii ON rii.report_id = rp.id
            WHERE ($1::text IS NULL OR rp.status = $1)
              AND ($2::text IS NULL OR rp.title ILIKE $2 OR u.email ILIKE $2 OR rsi.full_name ILIKE $2 OR rii.company_name ILIKE $2)
            "#,
            status,
            pattern
        )
        .fetch_one(pool)
        .await?;

        Ok(row.count.unwrap_or(0))
    }

    pub async fn delete_report(
        pool: &Pool<Postgres>,
        report_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let res = sqlx::query!(
            r#"
            DELETE FROM report_projects
            WHERE id = $1
            "#,
            report_id
        )
        .execute(pool)
        .await?;

        Ok(res.rows_affected() > 0)
    }
}

