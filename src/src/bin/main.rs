//! SecurityShield - Entry Point

use securityshield::{SecurityEngine, ConfigManager, init_logger, VERSION};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger();
    
    println!("🛡️  SecurityShield v{}", VERSION);
    println!("Ferramenta de Segurança Defensiva\n");
    
    let config = ConfigManager::load()?;
    let mut engine = SecurityEngine::new(config)?;
    
    engine.start().await?;
    
    println!("Pressione Ctrl+C para parar");
    tokio::signal::ctrl_c().await?;
    
    engine.stop().await?;
    
    Ok(())
}
