mod checks;
mod config;
mod output;

use std::process::ExitCode;

use clap::{Parser, Subcommand};

use crate::checks::run_checks;
use crate::config::Config;
use crate::output::print_summary;

#[derive(Parser)]
#[command(name = "eden")]
#[command(author, version, about = "Developer onboarding preflight checks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to config file (auto-detects eden.toml/yaml/json if not specified)
    #[arg(short, long, global = true)]
    config: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run preflight checks (default)
    Check,
    /// Initialize a new eden config file
    Init {
        /// Config format to generate
        #[arg(short, long, default_value = "toml")]
        format: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Check) {
        Commands::Check => run_check_command(cli.config),
        Commands::Init { format } => run_init_command(&format),
    }
}

fn run_check_command(config_path: Option<String>) -> ExitCode {
    let config = match Config::load(config_path) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    let results = run_checks(&config);
    let (passed, failed) = print_summary(&results);

    if failed > 0 {
        // TODO change need to needs if 1
        println!("\n🌱 {} sprouted, 🥀 {} need water", passed, failed);
        ExitCode::FAILURE
    } else {
        println!(
            "\n🌻 The garden is flourishing! All {} checks passed",
            passed
        );
        ExitCode::SUCCESS
    }
}

fn run_init_command(format: &str) -> ExitCode {
    let (filename, content) = match format {
        "toml" => ("eden.toml", include_str!("../templates/eden.toml")),
        "yaml" | "yml" => ("eden.yaml", include_str!("../templates/eden.yaml")),
        "json" => ("eden.json", include_str!("../templates/eden.json")),
        _ => {
            eprintln!("Unsupported format: {}. Use toml, yaml, or json.", format);
            return ExitCode::FAILURE;
        }
    };

    if std::path::Path::new(filename).exists() {
        eprintln!("{} already exists", filename);
        return ExitCode::FAILURE;
    }

    match std::fs::write(filename, content) {
        Ok(_) => {
            println!("🌱 Planted {}", filename);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Failed to create {}: {}", filename, e);
            ExitCode::FAILURE
        }
    }
}
