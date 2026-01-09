use std::env;

use super::CheckResult;

/// Check if an environment variable is set
pub fn check_env_var(var_name: &str) -> CheckResult {
    env::var(var_name).map_or_else(
        |_| CheckResult::fail("Env", var_name, "not set"),
        |value| {
            // Show truncated value for confirmation (hide sensitive data)
            let display_value = if value.len() > 20 {
                format!("{}...", &value[..20])
            } else if value.is_empty() {
                "(empty)".to_string()
            } else {
                mask_value(&value)
            };

            CheckResult::pass("Env", var_name, format!("set ({display_value})"))
        },
    )
}

/// Mask sensitive values, showing only first and last 2 chars
fn mask_value(value: &str) -> String {
    if value.len() <= 4 {
        "*".repeat(value.len())
    } else {
        format!(
            "{}{}{}",
            &value[..2],
            "*".repeat(value.len() - 4),
            &value[value.len() - 2..]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_existing_env() {
        // PATH should exist on any system
        let result = check_env_var("PATH");
        assert!(result.passed);
    }

    #[test]
    fn test_check_missing_env() {
        let result = check_env_var("DEFINITELY_NOT_A_REAL_ENV_VAR_12345");
        assert!(!result.passed);
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(mask_value("secret123"), "se*****23");
        assert_eq!(mask_value("ab"), "**");
        assert_eq!(mask_value("abcd"), "****");
        assert_eq!(mask_value("abcde"), "ab*de");
    }
}
