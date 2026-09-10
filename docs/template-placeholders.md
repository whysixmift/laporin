# template-placeholders.md – DOCX Template Placeholder Contract

## 1. Overview
Laporin uses a master template (`storage/templates/report-template-v1.docx`). During the generation job, placeholders in the format `{{placeholder_name}}` are replaced with user-supplied facts, research facts, or AI-generated sections.

## 2. Placeholder Mapping Table

| Placeholder Tag in Template | Source Object / Field | Category | Description |
|-----------------------------|-----------------------|----------|-------------|
| `{{report_title}}` | `report.title` | USER_FACT | Title of the PKL / Internship report |
| `{{student_name}}` | `student.full_name` | USER_FACT | Full name of the student |
| `{{student_id}}` | `student.student_id` | USER_FACT | NIM / NISN |
| `{{student_school}}` | `student.school` | USER_FACT | School / University name |
| `{{student_major}}` | `student.major` | USER_FACT | Major / Study program |
| `{{student_semester}}` | `student.semester` | USER_FACT | Semester / Class grade |
| `{{company_name}}` | `internship.company_name` | USER_FACT | Name of the host company / institution |
| `{{company_address}}` | `internship.company_address` | USER_FACT | Company office address |
| `{{internship_dept}}` | `internship.department` | USER_FACT | Assigned department / unit |
| `{{internship_role}}` | `internship.role` | USER_FACT | Assigned job role |
| `{{internship_start}}` | `internship.start_date` | USER_FACT | Start date (formatted `DD MMMM YYYY`) |
| `{{internship_end}}` | `internship.end_date` | USER_FACT | End date (formatted `DD MMMM YYYY`) |
| `{{supervisor_company}}` | `internship.company_supervisor` | USER_FACT | Company mentor / supervisor |
| `{{supervisor_school}}` | `internship.school_supervisor` | USER_FACT | Academic advisor / teacher |
| `{{cover_section}}` | `generated_sections.cover` | AI_DERIVED_TEXT | Formatted cover page block |
| `{{introduction_section}}` | `generated_sections.introduction` | AI_DERIVED_TEXT | Chapter 1 (Background, Objectives, Scope) |
| `{{company_profile_section}}` | `generated_sections.company_profile` | AI_DERIVED_TEXT | Chapter 2 (History, Org Structure, Business) |
| `{{activities_section}}` | `generated_sections.activities` | AI_DERIVED_TEXT | Chapter 3 (Daily tasks, Projects, Methodology) |
| `{{conclusion_section}}` | `generated_sections.conclusion` | AI_DERIVED_TEXT | Chapter 4 (Summary, Challenges, Suggestions) |

## 3. Formatting & Fallback Rules
1. If an optional `USER_FACT` is null, the placeholder is replaced with `-` or an empty string.
2. `AI_DERIVED_TEXT` blocks are sanitized to preserve standard paragraph breaks and list items in OpenXML without XML injection.
