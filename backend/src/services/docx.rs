use crate::domain::report::{GeneratedSections, InternshipInfo, Report, StudentInfo};
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

pub struct DocxService;

const DEFAULT_EMBEDDED_TEMPLATE: &[u8] = include_bytes!("../../templates/report-template-v1.docx");

fn format_indo_date(date: Option<NaiveDate>) -> String {
    match date {
        Some(d) => {
            let month_name = match d.month() {
                1 => "Januari",
                2 => "Februari",
                3 => "Maret",
                4 => "April",
                5 => "Mei",
                6 => "Juni",
                7 => "Juli",
                8 => "Agustus",
                9 => "September",
                10 => "Oktober",
                11 => "November",
                12 => "Desember",
                _ => "",
            };
            format!("{} {} {}", d.day(), month_name, d.year())
        }
        None => "-".to_string(),
    }
}

impl DocxService {
    pub fn ensure_default_template(
        template_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if template_path.exists() {
            return Ok(());
        }

        if let Some(parent) = template_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::fs::write(template_path, DEFAULT_EMBEDDED_TEMPLATE)?;
        Ok(())
    }

    pub fn render_report(
        template_bytes: &[u8],
        title: &str,
        student: &Option<StudentInfo>,
        internship: &Option<InternshipInfo>,
        sections: &Option<GeneratedSections>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let mut placeholders = HashMap::new();

        placeholders.insert("report_title".to_string(), title.to_string());

        if let Some(s) = student {
            let name = if s.full_name.is_empty() { "-" } else { &s.full_name };
            placeholders.insert("student_name".to_string(), name.to_string());
            placeholders.insert("student_name_upper".to_string(), name.to_uppercase());
            placeholders.insert("student_id".to_string(), s.student_id.clone());
            placeholders.insert("student_school".to_string(), s.school.clone());
            placeholders.insert("student_school_upper".to_string(), s.school.to_uppercase());
            placeholders.insert(
                "student_major".to_string(),
                s.major.clone().unwrap_or_else(|| "-".into()),
            );
            placeholders.insert(
                "student_semester".to_string(),
                s.semester.clone().unwrap_or_else(|| "-".into()),
            );
        } else {
            placeholders.insert("student_name".to_string(), "-".into());
            placeholders.insert("student_name_upper".to_string(), "-".into());
            placeholders.insert("student_id".to_string(), "-".into());
            placeholders.insert("student_school".to_string(), "-".into());
            placeholders.insert("student_school_upper".to_string(), "-".into());
            placeholders.insert("student_major".to_string(), "-".into());
            placeholders.insert("student_semester".to_string(), "-".into());
        }

        if let Some(i) = internship {
            let c_name = if i.company_name.is_empty() { "-" } else { &i.company_name };
            placeholders.insert("company_name".to_string(), c_name.to_string());
            placeholders.insert("company_name_upper".to_string(), c_name.to_uppercase());
            placeholders.insert(
                "company_address".to_string(),
                i.company_address.clone().unwrap_or_else(|| "-".into()),
            );
            placeholders.insert(
                "internship_dept".to_string(),
                i.department.clone().unwrap_or_else(|| "-".into()),
            );
            placeholders.insert(
                "internship_role".to_string(),
                i.role.clone().unwrap_or_else(|| "-".into()),
            );
            placeholders.insert("internship_start".to_string(), format_indo_date(i.start_date));
            placeholders.insert("internship_end".to_string(), format_indo_date(i.end_date));
            placeholders.insert(
                "supervisor_company".to_string(),
                i.company_supervisor.clone().unwrap_or_else(|| "Pembimbing Lapangan".into()),
            );
            placeholders.insert(
                "supervisor_school".to_string(),
                i.school_supervisor.clone().unwrap_or_else(|| "Guru Pembimbing".into()),
            );
            placeholders.insert("school_principal".to_string(), "Kepala Sekolah".into());
            placeholders.insert("school_hubin".to_string(), "Wakasek Hubungan Industri".into());
        } else {
            placeholders.insert("company_name".to_string(), "-".into());
            placeholders.insert("company_name_upper".to_string(), "-".into());
            placeholders.insert("company_address".to_string(), "-".into());
            placeholders.insert("internship_dept".to_string(), "-".into());
            placeholders.insert("internship_role".to_string(), "-".into());
            placeholders.insert("internship_start".to_string(), "-".into());
            placeholders.insert("internship_end".to_string(), "-".into());
            placeholders.insert("supervisor_company".to_string(), "-".into());
            placeholders.insert("supervisor_school".to_string(), "-".into());
            placeholders.insert("school_principal".to_string(), "-".into());
            placeholders.insert("school_hubin".to_string(), "-".into());
        }

        if let Some(sec) = sections {
            placeholders.insert("cover_section".to_string(), sec.cover.clone());
            placeholders.insert("introduction_section".to_string(), sec.introduction.clone());
            placeholders.insert(
                "company_profile_section".to_string(),
                sec.company_profile.clone(),
            );
            placeholders.insert("activities_section".to_string(), sec.activities.clone());
            placeholders.insert("conclusion_section".to_string(), sec.conclusion.clone());
        } else {
            placeholders.insert("cover_section".to_string(), "-".into());
            placeholders.insert("introduction_section".to_string(), "-".into());
            placeholders.insert("company_profile_section".to_string(), "-".into());
            placeholders.insert("activities_section".to_string(), "-".into());
            placeholders.insert("conclusion_section".to_string(), "-".into());
        }

        let reader = Cursor::new(template_bytes);
        let mut archive = ZipArchive::new(reader)?;

        let mut output_bytes = Vec::new();
        let mut writer = ZipWriter::new(Cursor::new(&mut output_bytes));

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_string();
            let options = FileOptions::<()>::default().compression_method(file.compression());

            let mut content = Vec::new();
            file.read_to_end(&mut content)?;

            if name.ends_with(".xml") {
                if let Ok(mut text) = String::from_utf8(content.clone()) {
                    for (k, v) in &placeholders {
                        let tag = format!("{{{{{}}}}}", k);
                        let escaped_val = quick_xml::escape::escape(v).to_string();
                        text = text.replace(&tag, &escaped_val);
                    }
                    writer.start_file(name, options)?;
                    writer.write_all(text.as_bytes())?;
                    continue;
                }
            }

            writer.start_file(name, options)?;
            writer.write_all(&content)?;
        }

        writer.finish()?;
        Ok(output_bytes)
    }
}
