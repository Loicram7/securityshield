//! Gerenciador de configurações

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigManager {
    pub app: AppConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub antivirus_enabled: bool,
}

impl ConfigManager {
    pub fn load() -> Result<Self> {
        Ok(Self::default())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self {
            app: AppConfig {
                theme: "dark".to_string(),
                language: "pt_BR".to_string(),
            },
            security: SecurityConfig {
                antivirus_enabled: true,
            },
        }
    }
}
