use clap::Parser;
use lua_mutation_test::adapter::FrameworkAdapter;
use lua_mutation_test::baseline::run_baseline;
use lua_mutation_test::cli::{exit, Cli, Command, RunArgs, WatchArgs};
use lua_mutation_test::config::{Config, DEFAULT_CONFIG_PATH};
use lua_mutation_test::incremental;
use lua_mutation_test::mutant::MutantGenerator;
use lua_mutation_test::operators::default_operators;
use lua_mutation_test::parser::Parser as LuaParser;
use lua_mutation_test::report::{generate_report, ReportData, ReportFormat};
use lua_mutation_test::runner::RunnerConfig;
use lua_mutation_test::score::score_results;
use lua_mutation_test::test_discovery::discover_tests;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            e.print().expect("failed to print CLI error");
            return ExitCode::from(exit::CLI_ERROR as u8);
        }
    };

    match run(cli) {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(exit::CLI_ERROR as u8)
        }
    }
}

fn run(cli: Cli) -> Result<i32, String> {
    let config_path = cli.config.clone().unwrap_or_else(default_config_path);
    let mut config = if config_path.exists() {
        Config::from_file(&config_path).map_err(|e| e.to_string())?
    } else {
        Config::default()
    };

    config.merge(config_from_cli(&cli));

    match cli.command {
        Command::Run(args) => run_pipeline(&args.path, &args, config),
        Command::Watch(args) => run_watch(args, config),
        Command::ListMutants(args) => {
            let source = std::fs::read_to_string(&args.path)
                .map_err(|e| format!("failed to read {}: {e}", args.path.display()))?;
            let mut parser = LuaParser::new().map_err(|e| e.to_string())?;
            let tree = parser.parse_source(&source).map_err(|e| e.to_string())?;
            let generator = MutantGenerator::new(default_operators());
            let mutants = generator.generate(&args.path, &source, &tree);
            for mutant in mutants {
                println!(
                    "{}",
                    serde_json::to_string(&mutant).map_err(|e| e.to_string())?
                );
            }
            Ok(exit::SUCCESS)
        }
        Command::ListOperators => {
            let operators = default_operators();
            if operators.is_empty() {
                println!("Available mutation operators: (none configured yet)");
            } else {
                println!("Available mutation operators:");
                for operator in operators {
                    println!("  {}", operator.id());
                }
            }
            Ok(exit::SUCCESS)
        }
        Command::Init => {
            std::fs::write(
                DEFAULT_CONFIG_PATH,
                r#"version = "1"
test_command = "busted"
timeout = 30
test_globs = ["*_spec.lua", "*_test.lua", "test_*.lua"]
source_globs = ["*.lua"]
"#,
            )
            .map_err(|e| format!("failed to write sample config: {e}"))?;
            println!("Created {DEFAULT_CONFIG_PATH}");
            Ok(exit::SUCCESS)
        }
    }
}

fn run_watch(args: WatchArgs, config: Config) -> Result<i32, String> {
    let path = args.path.clone();
    let debounce = Duration::from_millis(args.debounce);
    let args_for_watch = args.clone();
    let path_for_watch = path.clone();

    // Perform an initial incremental run.
    run_pipeline(path.clone(), &args, config.clone())?;
    println!("Watching for changes... (press Ctrl+C to stop)");

    incremental::watch::watch_project(
        &path,
        debounce,
        move || {
            println!("Change detected, re-running mutation tests...");
            run_pipeline(path_for_watch.clone(), &args_for_watch, config.clone()).map(|code| {
                if code == exit::TEST_FAILURES {
                    println!("Surviving mutants detected.");
                }
            })
        },
        || false,
    )?;

    Ok(exit::SUCCESS)
}

fn run_pipeline<P>(path: P, args: &impl RunArgsLike, config: Config) -> Result<i32, String>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let test_command = config
        .test_command
        .clone()
        .or(args.test_command())
        .ok_or("no test command configured")?;

    // Discover tests and run baseline.
    let tests = discover_tests(path, &config.test_globs);
    if tests.is_empty() {
        return Err("no test files discovered".to_string());
    }
    let adapter = FrameworkAdapter::from_config(config.framework.as_deref(), Some(&test_command));
    let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
    let baseline = run_baseline(&adapter, &test_refs);
    if !baseline.passed() {
        return Err("baseline test run failed; aborting".to_string());
    }

    // Discover source files and generate mutants.
    let source_files = discover_source_files(path, &config.source_globs)?;
    let mut mutants = Vec::new();
    let mut equivalent_results = Vec::new();
    let mut parser = LuaParser::new().map_err(|e| e.to_string())?;
    for file in &source_files {
        let source = std::fs::read_to_string(file)
            .map_err(|e| format!("failed to read {}: {e}", file.display()))?;
        let tree = parser.parse_source(&source).map_err(|e| e.to_string())?;
        let generator = MutantGenerator::new(default_operators());
        let (valid, _invalid, equivalent) = generator.generate_validated(file, &source, &tree);
        mutants.extend(valid.into_iter().map(|m| (m, source.clone())));
        equivalent_results.extend(equivalent.into_iter().map(|m| {
            let reason = m.equivalent_reason.clone().unwrap_or_default();
            lua_mutation_test::result::MutantResult::Equivalent { mutant: m, reason }
        }));
    }

    // Run each mutant incrementally.
    let timeout = config
        .timeout
        .or(args.timeout())
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(30));
    let runner_config = RunnerConfig {
        command: test_command.split_whitespace().map(String::from).collect(),
        timeout,
        project_root: path.to_path_buf(),
        snippet_limit: 1000,
    };

    let workers = args.workers().unwrap_or_else(num_cpus_like);
    let incremental_result = incremental::run_incremental(
        path,
        &config,
        &runner_config,
        &source_files,
        mutants,
        workers,
    )?;

    let mut results = incremental_result.results;
    results.extend(equivalent_results);

    // Score and report.
    let score = score_results(&results);
    println!(
        "Mutation score: {:.2}% (killed {}, survived {}, timed out {}, errored {}, equivalent {}, cached {}, ran {})",
        score.overall.percentage().unwrap_or(0.0),
        score.overall.killed,
        score.overall.survived,
        score.overall.timed_out,
        score.overall.error,
        score.overall.equivalent,
        incremental_result.cached,
        incremental_result.ran,
    );

    if let Some(format) = args.report_format() {
        let format: ReportFormat = format.parse().map_err(|e: String| e)?;
        let data = ReportData {
            results: &results,
            project_root: path,
            source_paths: &source_files,
        };
        let report_output = args.report_output();
        let output = report_output.as_deref();
        generate_report(format, data, output)?;
    }

    if score.overall.survived > 0 {
        Ok(exit::TEST_FAILURES)
    } else {
        Ok(exit::SUCCESS)
    }
}

fn num_cpus_like() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// Trait that abstracts over `RunArgs` and `WatchArgs` so the pipeline can be
/// shared by both commands.
trait RunArgsLike {
    fn test_command(&self) -> Option<String>;
    fn timeout(&self) -> Option<u64>;
    fn workers(&self) -> Option<usize>;
    fn report_format(&self) -> Option<String>;
    fn report_output(&self) -> Option<PathBuf>;
}

impl RunArgsLike for RunArgs {
    fn test_command(&self) -> Option<String> {
        self.test_command.clone()
    }
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
    fn workers(&self) -> Option<usize> {
        self.workers
    }
    fn report_format(&self) -> Option<String> {
        self.report_format.clone()
    }
    fn report_output(&self) -> Option<PathBuf> {
        self.report_output.clone()
    }
}

impl RunArgsLike for WatchArgs {
    fn test_command(&self) -> Option<String> {
        self.test_command.clone()
    }
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
    fn workers(&self) -> Option<usize> {
        self.workers
    }
    fn report_format(&self) -> Option<String> {
        None
    }
    fn report_output(&self) -> Option<PathBuf> {
        None
    }
}

fn discover_source_files(path: &Path, globs: &[String]) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    if path.is_file() {
        files.push(path.to_path_buf());
        return Ok(files);
    }

    for glob in globs {
        let pattern = path.join("**").join(glob).to_string_lossy().to_string();
        for entry in glob::glob(&pattern).map_err(|e| e.to_string())? {
            let p = entry.map_err(|e| e.to_string())?;
            if p.is_file() {
                files.push(p);
            }
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn default_config_path() -> PathBuf {
    PathBuf::from(DEFAULT_CONFIG_PATH)
}

fn config_from_cli(cli: &Cli) -> Config {
    let mut config = Config::default();
    match &cli.command {
        Command::Run(args) => {
            config.test_command = args.test_command.clone();
            config.timeout = args.timeout;
            if let Some(workers) = args.workers {
                config.parallelism = Some(workers);
            }
        }
        Command::Watch(args) => {
            config.test_command = args.test_command.clone();
            config.timeout = args.timeout;
            if let Some(workers) = args.workers {
                config.parallelism = Some(workers);
            }
        }
        _ => {}
    }
    config
}
