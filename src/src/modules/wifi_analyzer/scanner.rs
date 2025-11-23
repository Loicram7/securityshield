//! Scanner de redes Wi-Fi

use crate::modules::wifi_analyzer::network_info::{WifiNetwork, SecurityType};
use std::process::Command;
use anyhow::{Result, Context};
use log::{info, warn};

pub struct WifiScanner;

impl WifiScanner {
    pub fn new() -> Self {
        Self
    }
    
    /// Verifica se tem adaptador Wi-Fi disponível
    pub fn has_wifi_adapter() -> bool {
        if let Ok(output) = Command::new("nmcli")
            .args(&["device", "status"])
            .output()
        {
            if let Ok(text) = String::from_utf8(output.stdout) {
                return text.contains("wifi");
            }
        }
        false
    }
    
    /// Escanear redes Wi-Fi disponíveis
    pub fn scan_networks(&self) -> Result<Vec<WifiNetwork>> {
        info!("Escaneando redes Wi-Fi...");
        
        // Verificar se tem Wi-Fi
        if !Self::has_wifi_adapter() {
            warn!("Nenhum adaptador Wi-Fi encontrado");
            return Ok(Vec::new());
        }
        
        // Usar nmcli (NetworkManager) - disponível na maioria das distros Linux
        let output = Command::new("nmcli")
            .args(&[
                "-t",  // Formato tabular
                "-f",  // Campos específicos
                "SSID,BSSID,CHAN,FREQ,SIGNAL,SECURITY",
                "device",
                "wifi",
                "list"
            ])
            .output()
            .context("Erro ao executar nmcli. NetworkManager instalado?")?;
        
        if !output.status.success() {
            warn!("nmcli falhou. Usuário tem permissões?");
            return Ok(Vec::new());
        }
        
        let text = String::from_utf8(output.stdout)
            .context("Saída do nmcli não é UTF-8 válido")?;
        
        let mut networks = Vec::new();
        
        for line in text.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() < 6 {
                continue;
            }
            
            let ssid = parts[0].to_string();
            if ssid.is_empty() || ssid == "--" {
                continue;  // SSID oculto
            }
            
            let network = WifiNetwork {
                ssid,
                bssid: parts[1].to_string(),
                channel: parts[2].parse().unwrap_or(0),
                frequency: parts[3].parse().unwrap_or(0),
                signal_strength: parts[4].parse().unwrap_or(-100),
                security: Self::parse_security(parts[5]),
                is_connected: false,  // TODO: Verificar rede conectada
            };
            
            networks.push(network);
        }
        
        info!("Encontradas {} redes Wi-Fi", networks.len());
        Ok(networks)
    }
    
    fn parse_security(security_str: &str) -> SecurityType {
        let s = security_str.to_uppercase();
        
        if s.is_empty() || s == "--" {
            SecurityType::Open
        } else if s.contains("WPA3") {
            SecurityType::WPA3
        } else if s.contains("WPA2") {
            if s.contains("ENTERPRISE") || s.contains("802.1X") {
                SecurityType::WPA2Enterprise
            } else {
                SecurityType::WPA2Personal
            }
        } else if s.contains("WPA") {
            SecurityType::WPA
        } else if s.contains("WEP") {
            SecurityType::WEP
        } else {
            SecurityType::Unknown
        }
    }
}
