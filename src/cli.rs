//! Command-line interface definitions.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Mutation testing tool for Lua.
#[derive(Parser, Debug)]
#[command(
    name = "lua-mutation-test",
    version,
    about = "A mutation testing tool for Lua",
    long_about = None
)]
#[command(
    after_help = "EXAMPLES:\n  lua-mutation-test run src\n  lua-mutation-test run file.lua --test-command 'busted'\n  lua-mutation-test list-operators\n  lua-mutation-test init"
)]
pub struct Cli {
    /// Path to a configuration file.
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Enable verbose output.
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run mutation testing against the given path.
    Run(RunArgs),

    /// List generated mutants for a source file.
    ListMutants(ListMutantsArgs),

    /// List available mutation operators.
    ListOperators,

    /// Create a sample configuration file.
    Init,
}

/// Arguments for the `list-mutants` subcommand.
#[derive(Parser, Debug)]
pub struct ListMutantsArgs {
    /// Path to a Lua source file.
    pub path: PathBuf,
}

/// Arguments for the `run` subcommand.
#[derive(Parser, Debug)]
pub struct RunArgs {
    /// Path to a Lua file or directory to mutate.
    pub path: PathBuf,

    /// Custom shell command used to run tests.
    #[arg(long)]
    pub test_command: Option<String>,

    /// Timeout in seconds for each mutant test run.
    #[arg(long)]
    pub timeout: Option<u64>,

    /// Report format: summary, per-mutant, json, html.
    #[arg(long)]
    pub report_format: Option<String>,

    /// Output path for the generated report.
    #[arg(long)]
    pub report_output: Option<PathBuf>,

    /// Number of parallel workers for mutant execution.
    #[arg(long)]
    pub workers: Option<usize>,
}

/// Exit codes used by the binary.
pub mod exit {
    pub const SUCCESS: i32 = 0;
    pub const TEST_FAILURES: i32 = 1;
    pub const CLI_ERROR: i32 = 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_run_subcommand_with_flags() {
        let cli = Cli::parse_from([
            "lua-mutation-test",
            "run",
            "src",
            "--test-command",
            "busted",
            "--timeout",
            "30",
            "--report-format",
            "json",
            "--report-output",
            "report.json",
        ]);
        match cli.command {
            Command::Run(args) => {
                assert_eq!(args.path, PathBuf::from("src"));
                assert_eq!(args.test_command, Some("busted".to_string()));
                assert_eq!(args.timeout, Some(30));
                assert_eq!(args.report_format, Some("json".to_string()));
                assert_eq!(args.report_output, Some(PathBuf::from("report.json")));
            }
            _ => panic!("expected run subcommand"),
        }
    }

    #[test]
    fn parses_list_mutants_subcommand() {
        let cli = Cli::parse_from(["lua-mutation-test", "list-mutants", "src/foo.lua"]);
        match cli.command {
            Command::ListMutants(args) => {
                assert_eq!(args.path, PathBuf::from("src/foo.lua"));
            }
            _ => panic!("expected list-mutants subcommand"),
        }
    }

    #[test]
    fn parses_list_operators_subcommand() {
        let cli = Cli::parse_from(["lua-mutation-test", "list-operators"]);
        matches!(cli.command, Command::ListOperators);
    }

    #[test]
    fn parses_init_subcommand() {
        let cli = Cli::parse_from(["lua-mutation-test", "init"]);
        matches!(cli.command, Command::Init);
    }

    #[test]
    fn parses_global_options() {
        let cli = Cli::parse_from([
            "lua-mutation-test",
            "--config",
            "config.toml",
            "--verbose",
            "run",
            "src",
        ]);
        assert_eq!(cli.config, Some(PathBuf::from("config.toml")));
        assert!(cli.verbose);
        assert!(!cli.quiet);
    }
}
