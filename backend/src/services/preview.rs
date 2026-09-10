use std::path::Path;
use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;

pub struct PreviewService;

impl PreviewService {
    pub async fn convert_docx_to_pdf(
        docx_path: &Path,
        out_pdf_path: &Path,
        scratch_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        std::fs::create_dir_all(scratch_dir)?;
        if let Some(parent) = out_pdf_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Check if soffice / libreoffice is available
        let soffice_available = Command::new("which")
            .arg("soffice")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if soffice_available {
            let docx_str = docx_path.to_str().ok_or("Invalid docx path")?;
            let scratch_str = scratch_dir.to_str().ok_or("Invalid scratch path")?;

            let mut cmd = tokio::process::Command::new("soffice");
            cmd.arg("--headless")
                .arg("--convert-to")
                .arg("pdf")
                .arg("--outdir")
                .arg(scratch_str)
                .arg(docx_str);

            let res = timeout(Duration::from_secs(60), cmd.output()).await??;
            if !res.status.success() {
                let stderr = String::from_utf8_lossy(&res.stderr);
                tracing::warn!("LibreOffice conversion failed: {}", stderr);
                return Err(format!("LibreOffice conversion failed: {}", stderr).into());
            }

            // Find output PDF in scratch_dir
            let file_stem = docx_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("document");
            let generated_pdf = scratch_dir.join(format!("{}.pdf", file_stem));
            if generated_pdf.exists() {
                std::fs::copy(&generated_pdf, out_pdf_path)?;
                return Ok(());
            }
        }

        // Fallback PDF generation: create a valid PDF binary with standard header and stream
        let fallback_pdf = Self::generate_fallback_pdf_bytes(docx_path);
        std::fs::write(out_pdf_path, fallback_pdf)?;
        Ok(())
    }

    pub fn generate_fallback_pdf_bytes(docx_path: &Path) -> Vec<u8> {
        let name = docx_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Report Preview");

        // Minimal standard PDF 1.4 document
        let pdf_content = format!(
            "%PDF-1.4\n\
            1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n\
            2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n\
            3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >> endobj\n\
            4 0 obj << /Length 120 >> stream\n\
            BT\n\
            /F1 24 Tf\n\
            50 720 Td\n\
            (LAPORIN REPORT PREVIEW - WATERMARK) Tj\n\
            /F1 12 Tf\n\
            0 -40 Td\n\
            (Report ID: {}) Tj\n\
            ET\n\
            endstream\n\
            endobj\n\
            5 0 obj << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> endobj\n\
            xref\n\
            0 6\n\
            0000000000 65535 f \n\
            0000000010 00000 n \n\
            0000000060 00000 n \n\
            0000000117 00000 n \n\
            0000000247 00000 n \n\
            0000000418 00000 n \n\
            trailer << /Size 6 /Root 1 0 R >>\n\
            startxref\n\
            495\n\
            %%EOF\n",
            name
        );
        pdf_content.into_bytes()
    }
}
