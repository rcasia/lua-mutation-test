use clap::Parser;
use lua_mutation_test::adapter::FrameworkAdapter;
use lua_mutation_test::baseline::run_baseline;
use lua_mutation_test::cli::{exit, Cli, Command, RunArgs};
use lua_mutation_test::config::Config;
use lua_mutation_test::mutant::MutantGenerator;
use lua_mutation_test::operators::default_operators;
use lua_mutation_test::parser::Parser as LuaParser;
use lua_mutation_test::report::{generate_report, ReportData, ReportFormat};
use lua_mutation_test::runner::{run_mutant, RunnerConfig};
use lua_mutation_test::score::{score_results, Category};
use lua_mutation_test::test_discovery::discover_tests;
use lua_mutation_test::worker_pool::{MutantJob, WorkerPool};
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
        Command::Run(args) => run_pipeline(args, config),
        Command::ListMutants(args) => {
            let source = std::fs::read_to_string(&args.path)
                .map_err(|e| format!("failed to read {}: {e}", args.path.display()))?;
            let mut parser = LuaParser::new().map_err(|e| e.to_string())?;
            let tree = parser.parse_source(&source).map_err(|e| e.to_string())?;
            let generator = MutantGenerator::new(default_operators());
            let mutants = generator.generate(&args.path, &source, &tree);
            for mutant in mutants {
                println!("{}", serde_json::to_string(&mutant).map_err(|e| e.to_string())?);
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
                "lua-mutation-test.toml",
                r#"version = "1"
test_command = "busted"
timeout = 30
test_globs = ["*_spec.lua", "*_test.lua", "test_*.lua"]
source_globs = ["*.lua"]
"#,
            )
            .map_err(|e| format!("failed to write sample config: {e}"))?;
            println!("Created lua-mutation-test.toml");
            Ok(exit::SUCCESS)
        }
    }
}

fn run_pipeline(args: RunArgs, config: Config) -> Result<i32, String> {
    let path = args.path.clone();
    let test_command = config
        .test_command
        .clone()
        .or(args.test_command)
        .ok_or("no test command configured")?;

    // Discover tests and run baseline.
    let tests = discover_tests(&path, &config.test_globs);
    if tests.is_empty() {
        return Err("no test files discovered".to_string());
    }
    let adapter = FrameworkAdapter::from_config(
        config.framework.as_deref(),
        Some(&test_command),
    );
    let test_refs: Vec<&Path> = tests.iter().map(|p| p.as_path()).collect();
    let baseline = run_baseline(&adapter, &test_refs);
    if !baseline.passed() {
        return Err("baseline test run failed; aborting".to_string());
    }

    // Discover source files and generate mutants.
    let source_files = discover_source_files(&path, &config.source_globs)?;
    let mut mutants = Vec::new();
    let mut parser = LuaParser::new().map_err(|e| e.to_string())?;
    for file in &source_files {
        let source = std::fs::read_to_string(file)
            .map_err(|e| format!("failed to read {}: {e}", file.display()))?;
        let tree = parser.parse_source(&source).map_err(|e| e.to_string())?;
        let generator = MutantGenerator::new(default_operators());
        let (valid, _invalid) = generator.generate_validated(file, &source, &tree);
        mutants.extend(valid.into_iter().map(|m| (m, source.clone())));
    }

    // Run each mutant.
    let timeout = config
        .timeout
        .or(args.timeout)
        .map(Duration::from_secs)
        .unwrap_or_else(|| Duration::from_secs(30));
    let runner_config = RunnerConfig {
        command: test_command.split_whitespace().map(String::from).collect(),
        timeout,
        project_root: path.clone(),
        snippet_limit: 1000,
    };

    let workers = args.workers.unwrap_or_else(WorkerPool::default_workers);
    let pool = WorkerPool::new(workers)?;
    let jobs: Vec<_> = mutants
        .into_iter()
        .map(|(mutant, source)| MutantJob { mutant, source })
        .collect();
    let results = pool.run_mutants(&runner_config, jobs, None);

    // Score and report.
    let score = score_results(&results);
    println!(
        "Mutation score: {:.2}% (killed {}, survived {}, timed out {}, errored {})",
        score.overall.percentage().unwrap_or(0.0),
        score.overall.killed,
        score.overall.survived,
        score.overall.timed_out,
        score.overall.error
    );

    if let Some(format) = args.report_format {
        let format: ReportFormat = format.parse().map_err(|e: String| e)?;
        let data = ReportData {
            results: &results,
            project_root: &path,
            source_paths: &source_files,
        };
        let output = args.report_output.as_deref();
        generate_report(format, data, output)?;
    }

    if score.overall.survived > 0 {
        Ok(exit::TEST_FAILURES)
    } else {
        Ok(exit::SUCCESS)
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
    PathBuf::from(".lua-mutation-test.toml")
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
        _ => {}
    }
    config
}
