mod binary;
mod env;

pub use binary::check_binary;
pub use env::check_env_var;

use crate::config::Config;

#[derive(Debug)]
pub struct CheckResult {
    pub check_type: String,
    pub name: String,
    pub passed: bool,
    pub message: String,
}

impl CheckResult {
    pub fn pass(check_type: &str, name: &str, message: impl Into<String>) -> Self {
        Self {
            check_type: check_type.to_string(),
            name: name.to_string(),
            passed: true,
            message: message.into(),
        }
    }

    pub fn fail(check_type: &str, name: &str, message: impl Into<String>) -> Self {
        Self {
            check_type: check_type.to_string(),
            name: name.to_string(),
            passed: false,
            message: message.into(),
        }
    }
}

/// Run all checks defined in config
pub fn run_checks(config: &Config) -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Binary checks
    for binary in &config.checks.binaries {
        results.push(check_binary(binary));
    }

    // Environment variable checks
    for var in &config.checks.environment {
        results.push(check_env_var(var));
    }

    results
}
