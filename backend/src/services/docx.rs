use crate::domain::report::{GeneratedSections, InternshipInfo, Report, StudentInfo};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

pub struct DocxService;

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

        let document_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:rPr><w:b/><w:sz w:val="36"/></w:rPr><w:t>{{report_title}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>LAPORAN PRAKTIK KERJA LAPANGAN (PKL)</w:t></w:r></w:p>
    <w:p><w:r><w:t>Nama: {{student_name}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>NISN/NIM: {{student_id}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Sekolah/Universitas: {{student_school}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Jurusan: {{student_major}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Semester: {{student_semester}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Perusahaan: {{company_name}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Alamat: {{company_address}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Departemen: {{internship_dept}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Posisi: {{internship_role}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Periode: {{internship_start}} - {{internship_end}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Pembimbing Perusahaan: {{supervisor_company}}</w:t></w:r></w:p>
    <w:p><w:r><w:t>Pembimbing Sekolah: {{supervisor_school}}</w:t></w:r></w:p>
    <w:p><w:r><w:rPr><w:b/><w:sz w:val="28"/></w:rPr><w:t>COVER</w:t></w:r></w:p>
    <w:p><w:r><w:t>{{cover_section}}</w:t></w:r></w:p>
    <w:p><w:r><w:rPr><w:b/><w:sz w:val="28"/></w:rPr><w:t>BAB I PENDAHULUAN</w:t></w:r></w:p>
    <w:p><w:r><w:t>{{introduction_section}}</w:t></w:r></w:p>
    <w:p><w:r><w:rPr><w:b/><w:sz w:val="28"/></w:rPr><w:t>BAB II PROFIL PERUSAHAAN</w:t></w:r></w:p>
    <w:p><w:r><w:t>{{company_profile_section}}</w:t></w:r></w:p>
    <w:p><w:r><w:rPr><w:b/><w:sz w:val="28"/></w:rPr><w:t>BAB III PELAKSANAAN PRAKTIK KERJA</w:t></w:r></w:p>
    <w:p><w:r><w:t>{{activities_section}}</w:t></w:r></w:p>
    <w:p><w:r><w:rPr><w:b/><w:sz w:val="28"/></w:rPr><w:t>BAB IV KESIMPULAN DAN SARAN</w:t></w:r></w:p>
    <w:p><w:r><w:t>{{conclusion_section}}</w:t></w:r></w:p>
  </w:body>
</w:document>"#;

        let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
</Types>"#;

        let rels = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;

        let file = File::create(template_path)?;
        let mut zip = ZipWriter::new(file);
        let options =
            FileOptions::<()>::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(content_types.as_bytes())?;

        zip.start_file("_rels/.rels", options)?;
        zip.write_all(rels.as_bytes())?;

        zip.start_file("word/document.xml", options)?;
        zip.write_all(document_xml.as_bytes())?;

        zip.finish()?;
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
            placeholders.insert("student_name".to_string(), s.full_name.clone());
            placeholders.insert("student_id".to_string(), s.student_id.clone());
            placeholders.insert("student_school".to_string(), s.school.clone());
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
            placeholders.insert("student_id".to_string(), "-".into());
            placeholders.insert("student_school".to_string(), "-".into());
            placeholders.insert("student_major".to_string(), "-".into());
            placeholders.insert("student_semester".to_string(), "-".into());
        }

        if let Some(i) = internship {
            placeholders.insert("company_name".to_string(), i.company_name.clone());
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
            placeholders.insert(
                "internship_start".to_string(),
                i.start_date
                    .map(|d| d.format("%d %B %Y").to_string())
                    .unwrap_or_else(|| "-".into()),
            );
            placeholders.insert(
                "internship_end".to_string(),
                i.end_date
                    .map(|d| d.format("%d %B %Y").to_string())
                    .unwrap_or_else(|| "-".into()),
            );
            placeholders.insert(
                "supervisor_company".to_string(),
                i.company_supervisor.clone().unwrap_or_else(|| "-".into()),
            );
            placeholders.insert(
                "supervisor_school".to_string(),
                i.school_supervisor.clone().unwrap_or_else(|| "-".into()),
            );
        } else {
            placeholders.insert("company_name".to_string(), "-".into());
            placeholders.insert("company_address".to_string(), "-".into());
            placeholders.insert("internship_dept".to_string(), "-".into());
            placeholders.insert("internship_role".to_string(), "-".into());
            placeholders.insert("internship_start".to_string(), "-".into());
            placeholders.insert("internship_end".to_string(), "-".into());
            placeholders.insert("supervisor_company".to_string(), "-".into());
            placeholders.insert("supervisor_school".to_string(), "-".into());
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
