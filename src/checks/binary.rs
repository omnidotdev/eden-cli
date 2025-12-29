use std::process::Command;

use which::which;

use crate::config::BinaryCheck;

use super::CheckResult;

/// Check if a binary exists in PATH
pub fn check_binary(binary: &BinaryCheck) -> CheckResult {
    let name = binary.name();

    match which(name) {
        Ok(path) => {
            let version = get_version(name);
            let version_str = version.as_deref().unwrap_or("unknown version");
            let path_str = path.display();

            CheckResult::pass("Binary", name, format!("{version_str} ({path_str})"))
        }
        Err(_) => CheckResult::fail("Binary", name, "not found in PATH"),
    }
}

/// Attempt to get version string from a binary
fn get_version(name: &str) -> Option<String> {
    // try common version flags
    // TODO handle edge case `tilt`, e.g. output: 🌱 Binary: tilt - unknown shorthand flag: 'V' in -V (/usr/bin/tilt)
    for flag in ["--version", "-version", "-V", "version"] {
        if let Some(version) = try_version_flag(name, flag) {
            return Some(version);
        }
    }
    None
}

fn try_version_flag(name: &str, flag: &str) -> Option<String> {
    let output = Command::new(name).arg(flag).output().ok()?;

    if !output.status.success() && flag != "-V" {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // take first non-empty line from stdout or stderr
    let version_line = stdout
        .lines()
        .chain(stderr.lines())
        .find(|line| !line.trim().is_empty())?;

    // try to extract just the version number
    Some(extract_version(version_line))
}

/// Extract version from a version string like "docker version 20.10.8"
fn extract_version(line: &str) -> String {
    // look for semver-like patterns
    let words: Vec<&str> = line.split_whitespace().collect();

    for word in &words {
        // check if word looks like a version (starts with digit, contains dots)
        let clean = word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.');
        if clean.chars().next().is_some_and(|c| c.is_ascii_digit()) && clean.contains('.') {
            return format!("v{clean}");
        }
    }

    // fall back to first set of chars of the line
    if line.len() > 50 {
        format!("{}...", &line[..50])
    } else {
        line.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        assert_eq!(extract_version("docker version 20.10.8"), "v20.10.8");
        assert_eq!(extract_version("node v18.17.0"), "v18.17.0");
        assert_eq!(extract_version("rustc 1.72.0"), "v1.72.0");
    }

    #[test]
    fn test_check_common_binary() {
        // `ls` should exist on any Unix system
        let check = BinaryCheck::Simple("ls".to_string());
        let result = check_binary(&check);
        assert!(result.passed);
    }

    #[test]
    fn test_check_missing_binary() {
        let check = BinaryCheck::Simple("definitely-not-a-real-binary-12345".to_string());
        let result = check_binary(&check);
        assert!(!result.passed);
    }
}
