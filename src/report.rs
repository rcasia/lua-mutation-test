//! Mutation testing report generation.

use crate::result::MutantResult;
use crate::score::{score_results, Category, MutationScore, ScoreBreakdown};
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
        lines.push(format!(
            "[{}] {} {}:{} -> {}",
            result.category(),
            mutant.id,
            mutant.file.display(),
            mutant.line,
            mutant.replacement
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
}

fn json_report(data: ReportData<'_>) -> String {
    let breakdown = score_results(data.results);
    let results: Vec<JsonResult> = data
        .results
        .iter()
        .map(|r| {
            let m = r.mutant();
            JsonResult {
                id: m.id.clone(),
                file: m.file.to_string_lossy().to_string(),
                operator: m.operator.clone(),
                line: m.line,
                column: m.column,
                original: m.original.clone(),
                replacement: m.replacement.clone(),
                category: r.category().to_string(),
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
        rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td><code>{}</code> -> <code>{}</code></td></tr>\n",
            result.category(),
            m.id,
            m.file.display(),
            m.line,
            html_escape(&m.original),
            html_escape(&m.replacement)
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
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Survived => MutantResult::Survived {
                mutant,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::TimedOut => MutantResult::TimedOut {
                mutant,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Error => MutantResult::Error {
                mutant,
                reason: String::new(),
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
            },
            Category::Skipped => MutantResult::Survived {
                mutant,
                stdout_snippet: String::new(),
                stderr_snippet: String::new(),
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
    fn writes_report_to_file() {
        let tmp = std::env::temp_dir().join(format!("lmt-report-test-{}", std::process::id()));
        let report = generate_report(ReportFormat::Summary, dummy_data(), Some(&tmp)).unwrap();
        let contents = std::fs::read_to_string(&tmp).unwrap();
        assert_eq!(contents, report);
        let _ = std::fs::remove_file(&tmp);
    }
}
