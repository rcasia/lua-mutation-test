use clap::Parser;
use lua_mutation_test::cli::{exit, Cli, Command};
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
    match cli.command {
        Command::Run(args) => {
            println!("run: path={}", args.path.display());
            if let Some(cmd) = args.test_command {
                println!("  test-command: {cmd}");
            }
            if let Some(timeout) = args.timeout {
                println!("  timeout: {timeout}");
            }
            if let Some(output) = args.output {
                println!("  output: {output}");
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
test_globs = ["*_spec.lua", "*_test.lua", "test_*.lua"]
framework = "busted"
timeout = 30
"#,
            )
            .map_err(|e| format!("failed to write sample config: {e}"))?;
            println!("Created lua-mutation-test.toml");
            Ok(exit::SUCCESS)
        }
    }
}
