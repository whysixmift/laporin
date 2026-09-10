# storage.md – Storage Layout, Security, and Lifecycle Contract

## 1. Directory Structure
All application files reside strictly under a configured root path (default: `/home/avrjulian/laporin/storage` or configurable via `STORAGE_ROOT_DIR`).

```text
storage/
├── templates/
│   └── report-template-v1.docx      (0444, read-only template)
├── reports/
│   └── {report_id}/
│       ├── docx/
│       │   └── {file_uuid}.docx      (0640, final generated document)
│       └── preview/
│           └── {file_uuid}.pdf       (0640, low-res preview document)
└── tmp/
    └── {job_uuid}/                   (0700, transient scratch directory)
```

## 2. File Naming & Path Traversal Protection
1. **Deterministic UUIDs**: User-supplied filenames are never used on disk. All generated artifacts use RFC 4122 UUID v4 filenames: `{uuid}.docx` and `{uuid}.pdf`.
2. **Canonicalization & Boundary Checking**:
   - Every file operation must resolve the target path using canonical absolute paths (`std::fs::canonicalize`).
   - The backend validates that `target_path.starts_with(STORAGE_ROOT_DIR)` is `true`.
   - Any path traversing outside `STORAGE_ROOT_DIR` returns an immediate `SecurityError` and aborts the operation.

## 3. Permissions & Ownership
- **Directory Permissions**: `0700` (`rwx------`) for directories created by the app.
- **File Permissions**: `0640` (`rw-r-----`) for report files, owned by Unix user `laporin` and group `laporin`.
- **Templates**: `0444` (`r--r--r--`) to prevent accidental modification during generation.

## 4. Size Limits
- **Generated DOCX**: Maximum **2 MiB**.
- **Generated PDF Preview**: Maximum **5 MiB**.
- **External Fetched Research Pages**: Maximum **2 MiB** per source.

## 5. Retention & Cleanup Policies
| File Kind     | Location                          | Retention Period | Cleanup Trigger |
|---------------|-----------------------------------|------------------|-----------------|
| Temporary     | `storage/tmp/*`                   | 30 minutes       | Scheduled worker runs every 15 minutes. |
| Preview PDF   | `storage/reports/{id}/preview/*`  | Duration of report active lifecycle | Removed 30 days after project cancellation or expiration. |
| Final DOCX    | `storage/reports/{id}/docx/*`     | 30 days post-unlock | Deleted after 30 days, status marked `expired`. |

## 6. Disk Pressure Safety (16 GB Storage Protection)
The backend monitors available disk space on the filesystem mounting `STORAGE_ROOT_DIR` via periodic health checks:
- **Warning Threshold (< 2 GB Free)**: Structured log alert `WARN: disk_space_low`. Immediate trigger of temporary directory purge.
- **Critical Threshold (< 1 GB Free)**:
  1. Backend rejects all `POST /reports`, `POST /reports/{id}/research/start`, and `POST /reports/{id}/generation/start` requests with HTTP 503 (`INSUFFICIENT_STORAGE`).
  2. Active jobs pause execution until disk space is reclaimed.
