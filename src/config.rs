use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configuration structure that holds application settings
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    base_url: String,
    model_id: String,
    api_key: String,
}

impl Config {
    /// Load the configuration from the default path.
    pub fn load() -> anyhow::Result<Self> {
        let config_content = get_config_content()?;
        let config = toml::from_str(&config_content)?;

        Ok(config)
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn model_id(&self) -> &str {
        &self.model_id
    }
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

fn get_config_content() -> anyhow::Result<String> {
    const CONFIG_EXAMPLE: &str = "base_url = ''\nmodel_id = ''\napi_key = ''";

    let home_dir_path = std::env::var("HOME").context("Failed to get home environment variable")?;
    let config_dir_path = PathBuf::from(home_dir_path)
        .join(".config")
        .join("feelflow");
    let config_file_path = config_dir_path.join("config.toml");

    match (config_dir_path.exists(), config_file_path.exists()) {
        (false, _) => {
            // Directory does not exist
            fs::create_dir_all(&config_dir_path).with_context(|| {
                format!("Failed to create config directory: {:?}", config_dir_path)
            })?;
            fs::write(&config_file_path, CONFIG_EXAMPLE).with_context(|| {
                format!(
                    "Failed to write example config file to: {:?}",
                    config_file_path
                )
            })?;
        }
        (true, false) => {
            // Directory exists but file does not
            fs::write(&config_file_path, CONFIG_EXAMPLE).with_context(|| {
                format!(
                    "Failed to write example config file to: {:?}",
                    config_file_path
                )
            })?;
        }
        (true, true) => {
            // Both directory and file exist
            let content = fs::read_to_string(&config_file_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_file_path))?;
            return Ok(content);
        }
    }

    Ok(CONFIG_EXAMPLE.to_string())
}
