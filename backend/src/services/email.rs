use crate::config::Config;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

#[derive(Clone)]
pub struct EmailService {
    client: Client,
    pub resend_api_key: Option<String>,
    pub brevo_api_key: Option<String>,
    pub email_from: String,
    pub environment: String,
}

impl EmailService {
    pub fn new(config: &Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        let resend_api_key = std::env::var("RESEND_API_KEY")
            .ok()
            .filter(|s| !s.trim().is_empty() && !s.contains("placeholder"));

        let brevo_api_key = std::env::var("BREVO_API_KEY")
            .ok()
            .filter(|s| !s.trim().is_empty() && !s.contains("placeholder"));

        let email_from = std::env::var("EMAIL_FROM").unwrap_or_else(|_| {
            if resend_api_key.is_some() {
                "Laporin <onboarding@resend.dev>".to_string()
            } else {
                "Laporin <noreply@laporin.app>".to_string()
            }
        });

        Self {
            client,
            resend_api_key,
            brevo_api_key,
            email_from,
            environment: config.environment.clone(),
        }
    }

    pub async fn send_otp(&self, recipient_email: &str, otp: &str) -> Result<bool, String> {
        // Always log OTP to server console for troubleshooting and audit
        tracing::info!(
            "\n=======================================================\n\
            🔑 [LAPORIN OTP DISPATCH]\n\
            📧 Penerima: {}\n\
            🔢 Kode OTP: {}\n\
            ⏱️  Berlaku:  10 Menit\n\
            =======================================================",
            recipient_email,
            otp
        );

        let subject = format!("Kode Verifikasi OTP Laporin: {}", otp);
        let html_content = format!(
            r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <style>
    body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; background-color: #0f172a; color: #f8fafc; padding: 24px; margin: 0; }}
    .card {{ max-width: 500px; margin: 0 auto; background-color: #1e293b; border-radius: 12px; border: 1px solid #334155; padding: 32px; box-shadow: 0 4px 12px rgba(0,0,0,0.3); }}
    .header {{ text-align: center; margin-bottom: 24px; }}
    .logo {{ font-size: 24px; font-weight: 800; color: #38bdf8; letter-spacing: -0.5px; }}
    .subtitle {{ font-size: 14px; color: #94a3b8; margin-top: 4px; }}
    .otp-container {{ text-align: center; margin: 28px 0; background: #0f172a; padding: 20px; border-radius: 8px; border: 1px dashed #6366f1; }}
    .otp-code {{ font-size: 36px; font-weight: 800; letter-spacing: 10px; color: #818cf8; font-family: monospace; }}
    .note {{ font-size: 13px; color: #94a3b8; line-height: 1.6; margin-top: 20px; text-align: center; }}
    .footer {{ margin-top: 32px; padding-top: 16px; border-top: 1px solid #334155; text-align: center; font-size: 12px; color: #64748b; }}
    .contact {{ color: #38bdf8; text-decoration: none; font-weight: 600; }}
  </style>
</head>
<body>
  <div class="card">
    <div class="header">
      <div class="logo">⚡ LAPORIN</div>
      <div class="subtitle">Platform Generator Laporan PKL Otomatis</div>
    </div>
    
    <p style="font-size: 15px; color: #e2e8f0; text-align: center;">
      Halo! Gunakan kode verifikasi di bawah ini untuk mengaktifkan akun Anda:
    </p>

    <div class="otp-container">
      <div class="otp-code">{}</div>
    </div>

    <p class="note">
      Kode ini berlaku selama <strong>10 menit</strong>.<br>
      Jika Anda tidak merasa mendaftar di Laporin, abaikan email ini.
    </p>

    <div class="footer">
      Butuh bantuan? Hubungi Admin via WhatsApp 
      <a class="contact" href="https://wa.me/6285117206413">0851-1720-6413</a> / 
      <a class="contact" href="https://wa.me/6288809028653">0888-0902-8653</a>
    </div>
  </div>
</body>
</html>"#,
            otp
        );

        // 1. Try Resend if configured
        if let Some(ref api_key) = self.resend_api_key {
            let res = self
                .client
                .post("https://api.resend.com/emails")
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&json!({
                    "from": self.email_from,
                    "to": [recipient_email],
                    "subject": subject,
                    "html": html_content
                }))
                .send()
                .await;

            match res {
                Ok(resp) if resp.status().is_success() => {
                    tracing::info!(recipient = %recipient_email, "Email OTP sent successfully via Resend");
                    return Ok(true);
                }
                Ok(resp) => {
                    let err_text = resp.text().await.unwrap_or_default();
                    tracing::error!(recipient = %recipient_email, error = %err_text, "Failed to send email via Resend");
                }
                Err(e) => {
                    tracing::error!(recipient = %recipient_email, error = %e, "HTTP error sending email via Resend");
                }
            }
        }

        // 2. Try Brevo if configured
        if let Some(ref api_key) = self.brevo_api_key {
            let from_email = if self.email_from.contains('<') {
                self.email_from
                    .split('<')
                    .nth(1)
                    .and_then(|s| s.strip_suffix('>'))
                    .unwrap_or("noreply@laporin.app")
            } else {
                &self.email_from
            };

            let res = self
                .client
                .post("https://api.brevo.com/v3/smtp/email")
                .header("api-key", api_key)
                .header("Content-Type", "application/json")
                .json(&json!({
                    "sender": { "name": "Laporin", "email": from_email },
                    "to": [{ "email": recipient_email }],
                    "subject": subject,
                    "htmlContent": html_content
                }))
                .send()
                .await;

            match res {
                Ok(resp) if resp.status().is_success() => {
                    tracing::info!(recipient = %recipient_email, "Email OTP sent successfully via Brevo");
                    return Ok(true);
                }
                Ok(resp) => {
                    let err_text = resp.text().await.unwrap_or_default();
                    tracing::error!(recipient = %recipient_email, error = %err_text, "Failed to send email via Brevo");
                }
                Err(e) => {
                    tracing::error!(recipient = %recipient_email, error = %e, "HTTP error sending email via Brevo");
                }
            }
        }

        // Return false to indicate email service was not reachable/configured, but logged to stdout
        Ok(false)
    }
}
