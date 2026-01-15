// ABOUTME: Configuration management for the Resend CLI.
// ABOUTME: Handles YAML config files, profiles, and environment variables.

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::types::OutputFormat;

const DEFAULT_PROFILE: &str = "default";

/// Profile configuration stored in config file
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    pub api_key: Option<String>,
}

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigFile {
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

/// Runtime configuration with resolved values
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub profile: String,
    pub format: OutputFormat,
    pub output: Option<String>,
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            profile: DEFAULT_PROFILE.to_string(),
            format: OutputFormat::Table,
            output: None,
            verbose: false,
        }
    }
}

impl Config {
    /// Get the config file path
    pub fn config_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "resend") {
            let config_dir = proj_dirs.config_dir();
            Some(config_dir.join("config.yml"))
        } else {
            dirs::home_dir().map(|home| home.join(".resend").join("config.yml"))
        }
    }

    /// Load configuration file
    pub fn load_config_file() -> Result<ConfigFile> {
        let path = Self::config_path();

        if let Some(path) = path {
            if path.exists() {
                let contents = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read config file: {path:?}"))?;
                let config: ConfigFile = serde_yaml::from_str(&contents)
                    .with_context(|| "Failed to parse config file")?;
                return Ok(config);
            }
        }

        Ok(ConfigFile::default())
    }

    /// Save configuration file
    pub fn save_config_file(config_file: &ConfigFile) -> Result<()> {
        let path = Self::config_path()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config file path"))?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {parent:?}"))?;
        }

        let contents =
            serde_yaml::to_string(config_file).with_context(|| "Failed to serialize config")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write config file: {path:?}"))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    /// Load configuration with priority: env vars > config file > defaults
    pub fn load(
        profile: Option<&str>,
        format: Option<OutputFormat>,
        output: Option<&str>,
        verbose: bool,
    ) -> Result<Self> {
        let profile_name = profile
            .map(|s| s.to_string())
            .or_else(|| std::env::var("RESEND_PROFILE").ok())
            .unwrap_or_else(|| DEFAULT_PROFILE.to_string());

        let config_file = Self::load_config_file().unwrap_or_default();
        let file_profile = config_file.profiles.get(&profile_name);

        // Resolve API key: env > config file
        let resolved_api_key = std::env::var("RESEND_API_KEY")
            .ok()
            .or_else(|| file_profile.and_then(|p| p.api_key.clone()));

        Ok(Self {
            api_key: resolved_api_key,
            profile: profile_name,
            format: format.unwrap_or(OutputFormat::Table),
            output: output.map(|s| s.to_string()),
            verbose,
        })
    }

    /// Check if configuration has required credentials
    pub fn is_valid(&self) -> bool {
        self.api_key.is_some()
    }

    /// Set a profile in the config file
    pub fn set_profile(profile_name: &str, api_key: &str) -> Result<()> {
        let mut config_file = Self::load_config_file().unwrap_or_default();

        config_file.profiles.insert(
            profile_name.to_string(),
            Profile {
                api_key: Some(api_key.to_string()),
            },
        );

        Self::save_config_file(&config_file)
    }

    /// List all profiles
    pub fn list_profiles() -> Result<Vec<String>> {
        let config_file = Self::load_config_file()?;
        Ok(config_file.profiles.keys().cloned().collect())
    }

    /// Mask a key for display (show first 8 chars + asterisks)
    pub fn mask_key(key: &str) -> String {
        let char_count = key.chars().count();
        if char_count <= 8 {
            "*".repeat(char_count)
        } else {
            let prefix: String = key.chars().take(8).collect();
            format!("{prefix}********")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.api_key.is_none());
        assert_eq!(config.profile, "default");
        assert_eq!(config.format, OutputFormat::Table);
        assert!(!config.verbose);
    }

    #[test]
    fn test_config_is_valid_with_key() {
        let config = Config {
            api_key: Some("re_test_123".to_string()),
            ..Default::default()
        };
        assert!(config.is_valid());
    }

    #[test]
    fn test_config_is_invalid_without_key() {
        let config = Config::default();
        assert!(!config.is_valid());
    }

    #[test]
    fn test_mask_key_short() {
        assert_eq!(Config::mask_key("abc"), "***");
        assert_eq!(Config::mask_key("12345678"), "********");
    }

    #[test]
    fn test_mask_key_long() {
        assert_eq!(Config::mask_key("re_123456789"), "re_12345********");
    }

    #[test]
    fn test_config_file_default() {
        let config_file = ConfigFile::default();
        assert!(config_file.profiles.is_empty());
    }

    #[test]
    fn test_profile_serialize() {
        let profile = Profile {
            api_key: Some("re_test".to_string()),
        };
        let yaml = serde_yaml::to_string(&profile).unwrap();
        assert!(yaml.contains("api_key: re_test"));
    }
}
