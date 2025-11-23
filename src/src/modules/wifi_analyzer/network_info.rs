//! Estruturas de dados para redes Wi-Fi

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub bssid: String,  // MAC address do AP
    pub signal_strength: i32,  // dBm
    pub channel: u8,
    pub frequency: u32,  // MHz
    pub security: SecurityType,
    pub is_connected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityType {
    Open,           // Sem criptografia
    WEP,            // Inseguro
    WPA,            // Antigo
    WPA2Personal,   // Seguro
    WPA2Enterprise, // Muito seguro
    WPA3,           // Mais seguro
    Unknown,
}

impl SecurityType {
    pub fn is_secure(&self) -> bool {
        matches!(self, 
            SecurityType::WPA2Personal | 
            SecurityType::WPA2Enterprise | 
            SecurityType::WPA3
        )
    }
    
    pub fn risk_level(&self) -> &str {
        match self {
            SecurityType::Open => "CRÍTICO",
            SecurityType::WEP => "ALTO",
            SecurityType::WPA => "MÉDIO",
            SecurityType::WPA2Personal => "BAIXO",
            SecurityType::WPA2Enterprise => "MUITO BAIXO",
            SecurityType::WPA3 => "MUITO BAIXO",
            SecurityType::Unknown => "DESCONHECIDO",
        }
    }
}

impl WifiNetwork {
    pub fn signal_quality(&self) -> u8 {
        // Converter dBm para porcentagem (0-100)
        if self.signal_strength >= -50 {
            100
        } else if self.signal_strength <= -100 {
            0
        } else {
            (2 * (self.signal_strength + 100)) as u8
        }
    }
}
