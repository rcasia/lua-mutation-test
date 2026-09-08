//! Mutation testing report generation.

use crate::result::MutantResult;
use crate::score::{score_results, MutationScore, ScoreBreakdown};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Supported report formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormat {
    /// Human-readable CLI summary.
    Summary,
    /// Human-readable per-mutant listing.
    PerMutant,
    /// Machine-readable JSON.
    Json,
    /// Common Test Report Format (CTRF) JSON.
    Ctrf,
    /// Self-contained HTML page.
    Html,
}

impl std::str::FromStr for ReportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "summary" => Ok(ReportFormat::Summary),
            "per-mutant" => Ok(ReportFormat::PerMutant),
            "json" => Ok(ReportFormat::Json),
            "ctrf" => Ok(ReportFormat::Ctrf),
            "html" => Ok(ReportFormat::Html),
            _ => Err(format!("unknown report format: {s}")),
        }
    }
}

/// Input data for a report.
#[derive(Debug, Clone)]
pub struct ReportData<'a> {
    pub results: &'a [MutantResult],
    pub project_root: &'a Path,
    pub source_paths: &'a [PathBuf],
}

/// Generates a report of the requested format.
pub fn generate_report(
    format: ReportFormat,
    data: ReportData<'_>,
    output: Option<&Path>,
) -> Result<String, String> {
    let content = match format {
        ReportFormat::Summary => summary_report(data),
        ReportFormat::PerMutant => per_mutant_report(data),
        ReportFormat::Json => json_report(data),
        ReportFormat::Ctrf => ctrf_report(data),
        ReportFormat::Html => html_report(data),
    };

    if let Some(path) = output {
        std::fs::write(path, &content).map_err(|e| e.to_string())?;
    }

    Ok(content)
}

fn summary_report(data: ReportData<'_>) -> String {
    let score = score_results(data.results);
    let mut lines = Vec::new();
    lines.push("Mutation Test Results".to_string());
    lines.push("=====================".to_string());
    lines.push(format!("Total mutants: {}", score.overall.total()));
    lines.push(format!("Killed: {}", score.overall.killed));
    lines.push(format!("Survived: {}", score.overall.survived));
    lines.push(format!("Timed out: {}", score.overall.timed_out));
    lines.push(format!("Errors: {}", score.overall.error));
    lines.push(format!("Equivalent: {}", score.overall.equivalent));
    if let Some(pct) = score.overall.percentage() {
        lines.push(format!("Mutation score: {:.2}%", pct));
    } else {
        lines.push("Mutation score: N/A".to_string());
    }
    lines.join("\n")
}

fn per_mutant_report(data: ReportData<'_>) -> String {
    let mut lines = Vec::new();
    for result in data.results {
        let mutant = result.mutant();
        let reason = match result {
            crate::result::MutantResult::Equivalent { reason, .. } => {
                format!(" ({reason})")
            }
            _ => String::new(),
        };
        lines.push(format!(
            "[{}] {} {}:{} -> {}{}",
            result.category(),
            mutant.id,
            mutant.file.display(),
            mutant.line,
            mutant.replacement,
            reason
        ));
        lines.push(format!("  - {}", mutant.original));
    }
    lines.join("\n")
}

#[derive(Serialize, Deserialize)]
struct JsonReport {
    metadata: ReportMetadata,
    overall: MutationScore,
    breakdown: ScoreBreakdown,
    results: Vec<JsonResult>,
}

#[derive(Serialize, Deserialize)]
struct ReportMetadata {
    generated_at: String,
    tool_version: String,
    project_root: String,
    source_paths: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct JsonResult {
    id: String,
    file: String,
    operator: String,
    line: usize,
    column: usize,
    original: String,
    replacement: String,
    category: String,
    reason: Option<String>,
}

fn json_report(data: ReportData<'_>) -> String {
    let breakdown = score_results(data.results);
    let results: Vec<JsonResult> = data
        .results
        .iter()
        .map(|r| {
            let m = r.mutant();
            let reason = match r {
                crate::result::MutantResult::Equivalent { reason, .. } => Some(reason.clone()),
                _ => m.equivalent_reason.clone(),
            };
            JsonResult {
                id: m.id.clone(),
                file: m.file.to_string_lossy().to_string(),
                operator: m.operator.clone(),
                line: m.line,
                column: m.column,
                original: m.original.clone(),
                replacement: m.replacement.clone(),
                category: r.category().to_string(),
                reason,
            }
        })
        .collect();

    let report = JsonReport {
        metadata: ReportMetadata {
            generated_at: chrono_now(),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            project_root: data.project_root.to_string_lossy().to_string(),
            source_paths: data
                .source_paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect(),
        },
        overall: breakdown.overall.clone(),
        breakdown,
        results,
    };

    serde_json::to_string_pretty(&report).unwrap_or_default()
}

fn ctrf_report(data: ReportData<'_>) -> String {
    let score = score_results(data.results);
    let start = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;
    let mut other = 0;

    let tests: Vec<CtrfTest> = data
        .results
        .iter()
        .map(|r| {
            let (status, ctrf_status) = match r {
                MutantResult::Killed { .. } => {
                    passed += 1;
                    ("killed", "passed")
                }
                MutantResult::Survived { .. } => {
                    failed += 1;
                    ("survived", "failed")
                }
                MutantResult::TimedOut { .. } => {
                    other += 1;
                    ("timed_out", "other")
                }
                MutantResult::Error { .. } => {
                    other += 1;
                    ("error", "other")
                }
                MutantResult::Equivalent { .. } => {
                    skipped += 1;
                    ("equivalent", "skipped")
                }
            };
            let m = r.mutant();
            let duration = match r {
                MutantResult::Killed { duration_ms, .. } => *duration_ms,
                MutantResult::Survived { duration_ms, .. } => *duration_ms,
                MutantResult::TimedOut { duration_ms, .. } => *duration_ms,
                MutantResult::Error { duration_ms, .. } => *duration_ms,
                MutantResult::Equivalent { .. } => 0,
            };
            CtrfTest {
                name: format!(
                    "{} {}:{} -> {}",
                    m.operator,
                    m.file.display(),
                    m.line,
                    m.replacement
                ),
                status: ctrf_status.to_string(),
                duration,
                file_path: Some(m.file.to_string_lossy().to_string()),
                line: Some(m.line),
                suite: Some(vec![m.operator.clone()]),
                extra: Some(serde_json::json!({
                    "lua-mutation-test.mutant_id": m.id,
                    "lua-mutation-test.category": status,
                    "lua-mutation-test.operator": m.operator,
                    "lua-mutation-test.original": m.original,
                    "lua-mutation-test.replacement": m.replacement,
                    "lua-mutation-test.line": m.line,
                    "lua-mutation-test.column": m.column,
                })),
            }
        })
        .collect();

    let report = CtrfReport {
        report_format: "CTRF".to_string(),
        spec_version: "0.0.0".to_string(),
        timestamp: Some(chrono_now()),
        generated_by: Some("lua-mutation-test".to_string()),
        results: CtrfResults {
            tool: CtrfTool {
                name: "lua-mutation-test".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
                extra: None,
            },
            summary: CtrfSummary {
                tests: score.overall.total(),
                passed,
                failed,
                pending: 0,
                skipped,
                other,
                start,
                stop: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as u64)
                    .unwrap_or(0),
                extra: None,
            },
            tests,
            environment: None,
            extra: None,
        },
        extra: None,
    };

    serde_json::to_string_pretty(&report).unwrap_or_default()
}

#[derive(Serialize, Deserialize)]
struct CtrfReport {
    #[serde(rename = "reportFormat")]
    report_format: String,
    #[serde(rename = "specVersion")]
    spec_version: String,
    timestamp: Option<String>,
    #[serde(rename = "generatedBy")]
    generated_by: Option<String>,
    results: CtrfResults,
    extra: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct CtrfResults {
    tool: CtrfTool,
    summary: CtrfSummary,
    tests: Vec<CtrfTest>,
    environment: Option<serde_json::Value>,
    extra: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct CtrfTool {
    name: String,
    version: Option<String>,
    extra: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct CtrfSummary {
    tests: usize,
    passed: usize,
    failed: usize,
    pending: usize,
    skipped: usize,
    other: usize,
    start: u64,
    stop: u64,
    extra: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct CtrfTest {
    name: String,
    status: String,
    duration: u64,
    #[serde(rename = "filePath")]
    file_path: Option<String>,
    line: Option<usize>,
    suite: Option<Vec<String>>,
    extra: Option<serde_json::Value>,
}

fn chrono_now() -> String {
    // Fallback that avoids adding chrono dependency for a single timestamp.
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| format!("{}.{:03}Z", d.as_secs(), d.subsec_millis()))
        .unwrap_or_else(|_| "0".to_string())
}

fn html_report(data: ReportData<'_>) -> String {
    let breakdown = score_results(data.results);
    let mut rows = String::new();
    for result in data.results {
        let m = result.mutant();
        let reason = match result {
            crate::result::MutantResult::Equivalent { reason, .. } => {
                format!("<br><small>{}</small>", html_escape(reason))
            }
            _ => String::new(),
        };
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><code>{}</code> -> <code>{}</code>{}</td></tr>\n",
            result.category(),
            m.id,
            m.file.display(),
            m.line,
            html_escape(&m.original),
            html_escape(&m.replacement),
            reason
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Mutation Test Report</title>
<style>
  body {{ font-family: sans-serif; margin: 2rem; }}
  table {{ border-collapse: collapse; width: 100%; }}
  th, td {{ border: 1px solid #ccc; padding: 0.5rem; text-align: left; }}
  th {{ background: #f0f0f0; }}
  .score {{ font-size: 1.5rem; margin-bottom: 1rem; }}
</style>
</head>
<body>
  <h1>Mutation Test Report</h1>
  <div class="score">Overall score: {}%</div>
  <p>Generated: {}</p>
  <table>
    <thead>
      <tr><th>Result</th><th>ID</th><th>File</th><th>Line</th><th>Mutation</th></tr>
    </thead>
    <tbody>
      {}
    </tbody>
  </table>
</body>
</html>"#,
        breakdown
            .overall
            .percentage()
            .map(|p| format!("{:.2}", p))
            .unwrap_or_else(|| "N/A".to_string()),
        chrono_now(),
        rows
    )
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutant::{CandidateMutant, Mutant};
    use crate::score::Category;
    use std::path::PathBuf;

    fn dummy_result(category: Category) -> MutantResult {
        let mutant = Mutant::from_candidate(
            CandidateMutant {
                start_byte: 0,
                end_byte: 0,
                replacement: String::new(),
            },
            "dummy",
            PathBuf::from("src/foo.lua"),
            "local x = 1",
        );
        match category {
            Category::Killed => MutantResult::Killed {
                mutant,
                duration_ms: 10,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Survived => MutantResult::Survived {
                mutant,
                duration_ms: 20,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::TimedOut => MutantResult::TimedOut {
                mutant,
                duration_ms: 30,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Error => MutantResult::Error {
                mutant,
                duration_ms: 40,
                reason: String::new(),
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Skipped => MutantResult::Survived {
                mutant,
                duration_ms: 50,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Equivalent => MutantResult::Equivalent {
                mutant,
                reason: String::new(),
            },
        }
    }

    fn dummy_data() -> ReportData<'static> {
        static RESULTS: std::sync::OnceLock<Vec<MutantResult>> = std::sync::OnceLock::new();
        let results = RESULTS.get_or_init(|| {
            vec![
                dummy_result(Category::Killed),
                dummy_result(Category::Survived),
            ]
        });
        static PATHS: std::sync::OnceLock<Vec<PathBuf>> = std::sync::OnceLock::new();
        let paths = PATHS.get_or_init(|| vec![PathBuf::from("src/foo.lua")]);

        ReportData {
            results,
            project_root: Path::new("."),
            source_paths: paths,
        }
    }

    #[test]
    fn summary_report_includes_score() {
        let report = generate_report(ReportFormat::Summary, dummy_data(), None).unwrap();
        assert!(report.contains("Mutation score:"));
        assert!(report.contains("Killed:"));
    }

    #[test]
    fn json_report_contains_results() {
        let report = generate_report(ReportFormat::Json, dummy_data(), None).unwrap();
        assert!(report.contains("\"killed\":"));
        assert!(report.contains("\"results\":"));
    }

    #[test]
    fn html_report_contains_table() {
        let report = generate_report(ReportFormat::Html, dummy_data(), None).unwrap();
        assert!(report.contains("<table>"));
        assert!(report.contains("</table>"));
    }

    #[test]
    fn ctrf_report_contains_required_fields() {
        let report = generate_report(ReportFormat::Ctrf, dummy_data(), None).unwrap();
        assert!(report.contains("\"reportFormat\": \"CTRF\""));
        assert!(report.contains("\"specVersion\": \"0.0.0\""));
        assert!(report.contains("\"summary\""));
        assert!(report.contains("\"tests\""));
        assert!(report.contains("\"passed\""));
        assert!(report.contains("\"failed\""));
        assert!(report.contains("\"skipped\""));
        assert!(report.contains("\"other\""));
    }

    #[test]
    fn writes_report_to_file() {
        let tmp = std::env::temp_dir().join(format!("lmt-report-test-{}", std::process::id()));
        let report = generate_report(ReportFormat::Summary, dummy_data(), Some(&tmp)).unwrap();
        let contents = std::fs::read_to_string(&tmp).unwrap();
        assert_eq!(contents, report);
        let _ = std::fs::remove_file(&tmp);
    }
}
