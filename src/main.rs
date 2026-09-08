use clap::Parser;
use lua_mutation_test::cli::{exit, Cli, Command};
use lua_mutation_test::config::Config;
use std::path::PathBuf;
use std::process::ExitCode;

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
        Command::Run(args) => {
            println!("run: path={}", args.path.display());
            if let Some(cmd) = config.test_command.or(args.test_command) {
                println!("  test-command: {cmd}");
            }
            if let Some(timeout) = config.timeout.or(args.timeout) {
                println!("  timeout: {timeout}");
            }
            if !config.output.is_empty() {
                println!("  output: {:?}", config.output);
            }
            Ok(exit::SUCCESS)
        }
        Command::ListOperators => {
            println!("Available mutation operators: (none configured yet)");
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

fn default_config_path() -> PathBuf {
    PathBuf::from(".lua-mutation-test.toml")
}

fn config_from_cli(cli: &Cli) -> Config {
    let mut config = Config::default();
    match &cli.command {
        Command::Run(args) => {
            config.test_command = args.test_command.clone();
            config.timeout = args.timeout;
        }
        _ => {}
    }
    config
}
