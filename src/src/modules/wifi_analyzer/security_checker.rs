//! Verificação de segurança de redes

use crate::modules::wifi_analyzer::network_info::{WifiNetwork, SecurityType};
use crate::core::threat::{Threat, ThreatLevel, ThreatType};
use std::collections::HashMap;

pub struct SecurityChecker;

impl SecurityChecker {
    pub fn new() -> Self {
        Self
    }
    
    /// Analisar segurança de todas as redes
    pub fn analyze_networks(&self, networks: &[WifiNetwork]) -> Vec<Threat> {
        let mut threats = Vec::new();
        
        // 1. Detectar redes abertas (sem criptografia)
        for network in networks {
            if network.security == SecurityType::Open {
                let mut threat = Threat::new(
                    format!("Rede Wi-Fi aberta: {}", network.ssid),
                    ThreatType::Unknown,
                    ThreatLevel::High,
                );
                threat.description = format!(
                    "A rede '{}' não possui criptografia. Dados podem ser interceptados.",
                    network.ssid
                );
                threats.push(threat);
            }
        }
        
        // 2. Detectar WEP (inseguro)
        for network in networks {
            if network.security == SecurityType::WEP {
                let mut threat = Threat::new(
                    format!("Rede com WEP inseguro: {}", network.ssid),
                    ThreatType::Unknown,
                    ThreatLevel::High,
                );
                threat.description = format!(
                    "A rede '{}' usa WEP, que pode ser quebrado em minutos.",
                    network.ssid
                );
                threats.push(threat);
            }
        }
        
        // 3. Detectar Evil Twin (SSIDs duplicados)
        let duplicates = self.find_duplicate_ssids(networks);
        for (ssid, bssids) in duplicates {
            if bssids.len() > 1 {
                let mut threat = Threat::new(
                    format!("Possível Evil Twin: {}", ssid),
                    ThreatType::Unknown,
                    ThreatLevel::Critical,
                );
                threat.description = format!(
                    "Múltiplos APs com mesmo SSID '{}' detectados: {:?}. \
                     Pode ser ataque Evil Twin (AP falso).",
                    ssid, bssids
                );
                threats.push(threat);
            }
        }
        
        // 4. Detectar nomes suspeitos
        for network in networks {
            if self.is_suspicious_ssid(&network.ssid) {
                let mut threat = Threat::new(
                    format!("Nome suspeito: {}", network.ssid),
                    ThreatType::Unknown,
                    ThreatLevel::Medium,
                );
                threat.description = format!(
                    "A rede '{}' tem nome suspeito que pode indicar phishing.",
                    network.ssid
                );
                threats.push(threat);
            }
        }
        
        threats
    }
    
    fn find_duplicate_ssids(&self, networks: &[WifiNetwork]) -> HashMap<String, Vec<String>> {
        let mut ssid_map: HashMap<String, Vec<String>> = HashMap::new();
        
        for network in networks {
            ssid_map
                .entry(network.ssid.clone())
                .or_insert_with(Vec::new)
                .push(network.bssid.clone());
        }
        
        // Retornar apenas SSIDs com múltiplos BSSIDs
        ssid_map.into_iter()
            .filter(|(_, bssids)| bssids.len() > 1)
            .collect()
    }
    
    fn is_suspicious_ssid(&self, ssid: &str) -> bool {
        let suspicious_patterns = [
            "free",
            "público",
            "public",
            "grátis",
            "bank",
            "banco",
            "wifi",
            "login",
            "senha",
            "password",
        ];
        
        let ssid_lower = ssid.to_lowercase();
        suspicious_patterns.iter().any(|pattern| ssid_lower.contains(pattern))
    }
    
    /// Calcular score de segurança (0-100)
    pub fn calculate_security_score(&self, network: &WifiNetwork) -> u8 {
        let mut score = 0u8;
        
        // Criptografia (50 pontos)
        score += match network.security {
            SecurityType::WPA3 => 50,
            SecurityType::WPA2Enterprise => 50,
            SecurityType::WPA2Personal => 45,
            SecurityType::WPA => 25,
            SecurityType::WEP => 10,
            SecurityType::Open => 0,
            SecurityType::Unknown => 20,
        };
        
        // Força do sinal (25 pontos) - sinal forte = mais confiável
        score += (network.signal_quality() / 4) as u8;
        
        // SSID não suspeito (25 pontos)
        if !self.is_suspicious_ssid(&network.ssid) {
            score += 25;
        }
        
        score.min(100)
    }
}
