//! # SecurityShield
//! 
//! Ferramenta de segurança defensiva open-source
//! 
//! ATENÇÃO: Esta é uma ferramenta DEFENSIVA.
//! Uso para ataques é estritamente proibido.

// Módulos públicos
pub mod core;
pub mod modules;
pub mod auth;
pub mod config;
pub mod ipc;
pub mod security;
pub mod updater;
pub mod utils;
pub mod ai;

// Re-exportar tipos principais
pub use core::engine::SecurityEngine;
pub use core::threat::{Threat, ThreatLevel};
pub use config::manager::ConfigManager;

/// Versão do SecurityShield
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Inicializar logger
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
