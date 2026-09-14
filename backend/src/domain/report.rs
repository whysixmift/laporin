use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Draft,
    Researching,
    ResearchCompleted,
    Generating,
    Generated,
    PreviewReady,
    PaymentPending,
    Paid,
    Unlocked,
    Failed,
    Cancelled,
    Expired,
}

impl ReportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReportStatus::Draft => "draft",
            ReportStatus::Researching => "researching",
            ReportStatus::ResearchCompleted => "research_completed",
            ReportStatus::Generating => "generating",
            ReportStatus::Generated => "generated",
            ReportStatus::PreviewReady => "preview_ready",
            ReportStatus::PaymentPending => "payment_pending",
            ReportStatus::Paid => "paid",
            ReportStatus::Unlocked => "unlocked",
            ReportStatus::Failed => "failed",
            ReportStatus::Cancelled => "cancelled",
            ReportStatus::Expired => "expired",
        }
    }

    pub fn from_str_lossy(s: &str) -> Self {
        match s {
            "draft" => ReportStatus::Draft,
            "researching" => ReportStatus::Researching,
            "research_completed" => ReportStatus::ResearchCompleted,
            "generating" => ReportStatus::Generating,
            "generated" => ReportStatus::Generated,
            "preview_ready" => ReportStatus::PreviewReady,
            "payment_pending" => ReportStatus::PaymentPending,
            "paid" => ReportStatus::Paid,
            "unlocked" => ReportStatus::Unlocked,
            "failed" => ReportStatus::Failed,
            "cancelled" => ReportStatus::Cancelled,
            "expired" => ReportStatus::Expired,
            _ => ReportStatus::Draft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentInfo {
    pub full_name: String,
    pub student_id: String,
    pub school: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternshipInfo {
    pub company_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_supervisor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub school_supervisor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub fetched_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchFact {
    pub claim: String,
    pub sources: Vec<ResearchSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedSections {
    pub cover: String,
    pub introduction: String,
    pub company_profile: String,
    pub activities: String,
    pub conclusion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCreate {
    pub title: String,
    pub student: StudentInfo,
    pub internship: InternshipInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub student: Option<StudentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internship: Option<InternshipInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub title: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub student: Option<StudentInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internship: Option<InternshipInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub research_facts: Option<Vec<ResearchFact>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_sections: Option<GeneratedSections>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_doc_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminReportSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub title: String,
    pub status: String,
    pub student_name: Option<String>,
    pub company_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

