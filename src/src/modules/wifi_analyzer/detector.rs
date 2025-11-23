//! Detector principal de ameaças Wi-Fi

use crate::modules::wifi_analyzer::{WifiScanner, SecurityChecker};
use crate::modules::wifi_analyzer::network_info::WifiNetwork;
use crate::core::threat::Threat;
use anyhow::Result;
use log::info;

pub struct WifiAnalyzer {
    scanner: WifiScanner,
    security_checker: SecurityChecker,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub networks: Vec<WifiNetwork>,
    pub threats: Vec<Threat>,
    pub secure_networks: usize,
    pub insecure_networks: usize,
}

impl WifiAnalyzer {
    pub fn new() -> Self {
        Self {
            scanner: WifiScanner::new(),
            security_checker: SecurityChecker::new(),
        }
    }
    
    /// Executar análise completa
    pub fn analyze(&self) -> Result<AnalysisResult> {
        info!("Iniciando análise de Wi-Fi...");
        
        // Escanear redes
        let networks = self.scanner.scan_networks()?;
        
        if networks.is_empty() {
            info!("Nenhuma rede Wi-Fi encontrada");
            return Ok(AnalysisResult {
                networks: vec![],
                threats: vec![],
                secure_networks: 0,
                insecure_networks: 0,
            });
        }
        
        // Analisar segurança
        let threats = self.security_checker.analyze_networks(&networks);
        
        // Contar redes seguras vs inseguras
        let secure_networks = networks.iter()
            .filter(|n| n.security.is_secure())
            .count();
        let insecure_networks = networks.len() - secure_networks;
        
        info!("Análise concluída: {} redes, {} ameaças", 
              networks.len(), threats.len());
        
        Ok(AnalysisResult {
            networks,
            threats,
            secure_networks,
            insecure_networks,
        })
    }
    
    /// Obter recomendações de segurança
    pub fn get_recommendations(&self, result: &AnalysisResult) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if result.insecure_networks > 0 {
            recommendations.push(format!(
                "⚠️  {} rede(s) insegura(s) detectada(s). Evite conectar.",
                result.insecure_networks
            ));
        }
        
        if result.threats.iter().any(|t| t.name.contains("Evil Twin")) {
            recommendations.push(
                "🚨 Possível ataque Evil Twin detectado! Verifique BSSID antes de conectar.".to_string()
            );
        }
        
        if result.threats.iter().any(|t| t.name.contains("aberta")) {
            recommendations.push(
                "🔓 Redes abertas detectadas. Use VPN se precisar conectar.".to_string()
            );
        }
        
        if recommendations.is_empty() {
            recommendations.push("✅ Ambiente Wi-Fi parece seguro.".to_string());
        }
        
        recommendations
    }
}
