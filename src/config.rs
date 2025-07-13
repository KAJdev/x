use anyhow::{Context, Result};
use dialoguer::{Input, Select};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub provider: LlmProvider,
    pub api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LlmProvider {
    OpenAI,
    Claude,
}

impl LlmProvider {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "openai" => Ok(LlmProvider::OpenAI),
            "claude" => Ok(LlmProvider::Claude),
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", s)),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Err(anyhow::anyhow!(
                "No configuration found. Please run 'x --config' to set up your LLM provider."
            ));
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        let content = toml::to_string(self).context("Failed to serialize config")?;

        fs::write(&config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir =
            config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;

        Ok(config_dir.join("x").join("config.toml"))
    }
}

pub async fn handle_config(provider: Option<String>, api_key: Option<String>) -> Result<()> {
    let provider = match provider {
        Some(p) => LlmProvider::from_str(&p)?,
        None => {
            let providers = ["OpenAI", "Claude"];
            let selection = Select::new()
                .with_prompt("Select your LLM provider")
                .items(&providers)
                .default(0)
                .interact()?;

            match selection {
                0 => LlmProvider::OpenAI,
                1 => LlmProvider::Claude,
                _ => return Err(anyhow::anyhow!("Invalid selection")),
            }
        }
    };

    let api_key = match api_key {
        Some(key) => key,
        None => {
            let prompt = match provider {
                LlmProvider::OpenAI => "Enter your OpenAI API key",
                LlmProvider::Claude => "Enter your Anthropic API key",
            };

            Input::new().with_prompt(prompt).interact_text()?
        }
    };

    let config = Config { provider, api_key };
    config.save()?;

    println!("✅ Configuration saved successfully!");
    Ok(())
}
