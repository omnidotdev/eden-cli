use std::fs;
use std::path::Path;

use serde::Deserialize;
use thiserror::Error;

/// Config file names to search for, in priority order
const CONFIG_FILES: &[&str] = &[
    "eden.toml",
    "eden.yaml",
    "eden.yml",
    "eden.json",
    "eden.jsonc",
];

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("No config file found. Run `eden init` to create one.")]
    NotFound,
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Failed to parse JSONC: {0}")]
    Json5Error(#[from] json5::Error),
    #[error("Unsupported config format: {0}")]
    UnsupportedFormat(String),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub checks: Checks,
}

#[derive(Debug, Default, Deserialize)]
pub struct Checks {
    #[serde(default)]
    pub binaries: Vec<BinaryCheck>,
    #[serde(default, alias = "env_vars")]
    pub environment: Vec<String>,
}

/// Binary check - can be a simple string or a struct with version
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BinaryCheck {
    Simple(String),
    #[allow(dead_code)] // Prepared for future version constraints
    WithVersion {
        name: String,
        version: Option<String>,
    },
}

impl BinaryCheck {
    pub fn name(&self) -> &str {
        match self {
            BinaryCheck::Simple(name) => name,
            BinaryCheck::WithVersion { name, .. } => name,
        }
    }

    #[allow(dead_code)] // Prepared for future version constraints
    pub fn version(&self) -> Option<&str> {
        match self {
            BinaryCheck::Simple(_) => None,
            BinaryCheck::WithVersion { version, .. } => version.as_deref(),
        }
    }
}

impl Config {
    /// Load config from specified path or auto-detect
    pub fn load(path: Option<String>) -> Result<Self, ConfigError> {
        let config_path = match path {
            Some(p) => {
                if Path::new(&p).exists() {
                    p
                } else {
                    return Err(ConfigError::ReadError(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Config file not found: {}", p),
                    )));
                }
            }
            None => Self::find_config()?,
        };

        let content = fs::read_to_string(&config_path)?;
        Self::parse(&config_path, &content)
    }

    /// Find config file in current directory
    fn find_config() -> Result<String, ConfigError> {
        for filename in CONFIG_FILES {
            if Path::new(filename).exists() {
                return Ok(filename.to_string());
            }
        }
        Err(ConfigError::NotFound)
    }

    /// Parse config content based on file extension
    fn parse(path: &str, content: &str) -> Result<Self, ConfigError> {
        let extension = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match extension {
            "toml" => Ok(toml::from_str(content)?),
            "yaml" | "yml" => Ok(serde_yaml::from_str(content)?),
            "json" => Ok(serde_json::from_str(content)?),
            "jsonc" => Ok(json5::from_str(content)?),
            _ => Err(ConfigError::UnsupportedFormat(extension.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toml() {
        let content = r#"
[checks]
binaries = ["docker", "node"]
environment = ["DATABASE_URL"]
"#;
        let config = Config::parse("test.toml", content).unwrap();
        assert_eq!(config.checks.binaries.len(), 2);
        assert_eq!(config.checks.environment.len(), 1);
    }

    #[test]
    fn test_parse_yaml() {
        let content = r#"
checks:
  binaries:
    - docker
    - node
  environment:
    - DATABASE_URL
"#;
        let config = Config::parse("test.yaml", content).unwrap();
        assert_eq!(config.checks.binaries.len(), 2);
    }

    #[test]
    fn test_parse_json() {
        let content = r#"{"checks": {"binaries": ["docker"], "environment": []}}"#;
        let config = Config::parse("test.json", content).unwrap();
        assert_eq!(config.checks.binaries.len(), 1);
    }
}
