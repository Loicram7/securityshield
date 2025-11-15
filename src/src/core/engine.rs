//! Security Engine

use crate::config::manager::ConfigManager;
use anyhow::Result;
use log::info;

pub struct SecurityEngine {
    config: ConfigManager,
    running: bool,
}

impl SecurityEngine {
    pub fn new(config: ConfigManager) -> Result<Self> {
        Ok(Self {
            config,
            running: false,
        })
    }
    
    pub async fn start(&mut self) -> Result<()> {
        info!("Iniciando Security Engine...");
        self.running = true;
        info!("✓ Engine iniciado");
        Ok(())
    }
    
    pub async fn stop(&mut self) -> Result<()> {
        info!("Parando Security Engine...");
        self.running = false;
        info!("✓ Engine parado");
        Ok(())
    }
    
    pub fn is_running(&self) -> bool {
        self.running
    }
}
