use lua_mutation_test::adapter::FrameworkAdapter;
use lua_mutation_test::baseline::run_baseline;
use lua_mutation_test::config::{Config, DEFAULT_CONFIG_PATH};
use lua_mutation_test::mutant::MutantGenerator;
use lua_mutation_test::operators::default_operators;
use lua_mutation_test::parser::Parser as LuaParser;
use lua_mutation_test::report::{generate_report, ReportData, ReportFormat};
use lua_mutation_test::result::MutantResult;
use lua_mutation_test::runner::RunnerConfig;
use lua_mutation_test::score::score_results;
use lua_mutation_test::test_discovery::discover_tests;
use std::path::{Path, PathBuf};
use std::time::Duration;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn copy_fixture_to_temp(name: &str) -> tempfile::TempDir {
    let source = fixture_path(name);
    let temp = tempfile::tempdir().unwrap();
    copy_dir_all(&source, temp.path()).unwrap();
    temp
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn make_executable(path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(path).unwrap().permissions();
        perms.set_mode(perms.mode() | 0o111);
        std::fs::set_permissions(path, perms).unwrap();
    }
}

fn run_fixture(name: &str) -> (Vec<MutantResult>, Vec<PathBuf>) {
    let temp = copy_fixture_to_temp(name);
    let project_root = temp.path().to_path_buf();
    let config_path = project_root.join(DEFAULT_CONFIG_PATH);
    let config = Config::from_file(&config_path).expect("failed to load fixture config");

    let test_command = config.test_command.clone().expect("test command required");
    let runner_script = project_root.join(&test_command);
    make_executable(&runner_script);

    let tests = discover_tests(&project_root, &config.test_globs);
    assert!(!tests.is_empty(), "no tests discovered in fixture {name}");

    let adapter = FrameworkAdapter::from_config(
        config.framework.as_deref(),
        Some(runner_script.to_str().unwrap()),
    );
    let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
    let baseline = run_baseline(&adapter, &test_refs);
    assert!(baseline.passed(), "baseline failed in fixture {name}");

    let source_files = discover_source_files(&project_root, &config.source_globs);
    let mut mutants = Vec::new();
    let mut equivalent_results = Vec::new();
    let mut parser = LuaParser::new().expect("failed to create parser");
    for file in &source_files {
        let source = std::fs::read_to_string(file).expect("failed to read source");
        let tree = parser
            .parse_source(&source)
            .expect("failed to parse source");
        let generator = MutantGenerator::new(default_operators());
        let (valid, _invalid, equivalent) = generator.generate_validated(file, &source, &tree);
        mutants.extend(valid.into_iter().map(|m| (m, source.clone())));
        equivalent_results.extend(equivalent.into_iter().map(|m| {
            let reason = m.equivalent_reason.clone().unwrap_or_default();
            MutantResult::Equivalent { mutant: m, reason }
        }));
    }

    let timeout = config
        .timeout
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(30));
    let runner_config = RunnerConfig {
        command: test_command.split_whitespace().map(String::from).collect(),
        timeout,
        project_root: project_root.clone(),
        snippet_limit: 1000,
    };

    let workers = config.parallelism.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
    });
    let incremental_result = lua_mutation_test::incremental::run_incremental(
        &project_root,
        &config,
        &runner_config,
        &source_files,
        mutants,
        workers,
    )
    .expect("incremental run failed");

    let mut results = incremental_result.results;
    results.extend(equivalent_results);

    // Keep temp alive until results are collected.
    let _ = temp;
    (results, source_files)
}

fn discover_source_files(path: &Path, globs: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for glob in globs {
        let pattern = path.join(glob).to_string_lossy().to_string();
        for entry in glob::glob(&pattern).expect("invalid glob") {
            let p = entry.expect("glob entry error");
            if p.is_file() {
                files.push(p);
            }
        }
    }
    files.sort();
    files.dedup();
    files
}

#[test]
fn equivalent_mutant_fixture_flags_arithmetic_identities() {
    let (results, _source_files) = run_fixture("equivalent_mutants");
    let score = score_results(&results);

    assert!(
        score.overall.equivalent > 0,
        "expected at least one equivalent mutant"
    );

    let equivalent_count = results
        .iter()
        .filter(|r| matches!(r, MutantResult::Equivalent { .. }))
        .count();
    assert_eq!(equivalent_count, score.overall.equivalent);

    for result in &results {
        if let MutantResult::Equivalent { reason, .. } = result {
            assert!(
                reason.contains("arithmetic identity"),
                "unexpected equivalent reason: {reason}"
            );
        }
    }
}

#[test]
fn simple_math_fixture_produces_mutation_score() {
    let (results, source_files) = run_fixture("simple_math");
    let score = score_results(&results);

    assert!(
        score.overall.total() > 0,
        "expected at least one mutant to be generated"
    );
    assert!(
        score.overall.denominator() > 0,
        "expected countable mutants"
    );

    let killed = results
        .iter()
        .filter(|r| matches!(r, MutantResult::Killed { .. }))
        .count();
    assert!(killed > 0, "expected at least one killed mutant");

    // Smoke-test report generation.
    let data = ReportData {
        results: &results,
        project_root: &fixture_path("simple_math"),
        source_paths: &source_files,
    };
    let summary = generate_report(ReportFormat::Summary, data, None).unwrap();
    assert!(summary.contains("Mutation score:"));

    let json = generate_report(
        ReportFormat::Json,
        ReportData {
            results: &results,
            project_root: &fixture_path("simple_math"),
            source_paths: &source_files,
        },
        None,
    )
    .unwrap();
    assert!(json.contains("\"equivalent\":"));
}

#[test]
fn report_includes_equivalent_reason() {
    let (results, source_files) = run_fixture("equivalent_mutants");
    let data = ReportData {
        results: &results,
        project_root: &fixture_path("equivalent_mutants"),
        source_paths: &source_files,
    };
    let json = generate_report(ReportFormat::Json, data, None).unwrap();
    assert!(json.contains("\"equivalent\":"));
    assert!(json.contains("arithmetic identity"));
}

#[test]
fn equivalent_mutants_are_not_counted_in_denominator() {
    let (results, _source_files) = run_fixture("equivalent_mutants");
    let score = score_results(&results);

    assert_eq!(
        score.overall.denominator(),
        score.overall.killed + score.overall.survived + score.overall.skipped,
        "equivalent mutants should be excluded from the score denominator"
    );
}

#[test]
fn score_categories_include_equivalent() {
    let (results, _source_files) = run_fixture("equivalent_mutants");
    let score = score_results(&results);

    assert!(score.overall.equivalent > 0);
    assert!(
        score.by_operator.contains_key("arithmetic_operator"),
        "expected arithmetic operator breakdown"
    );
    let operator_score = score.by_operator.get("arithmetic_operator").unwrap();
    assert_eq!(
        operator_score.equivalent, score.overall.equivalent,
        "all equivalent mutants should come from the arithmetic operator"
    );
}
