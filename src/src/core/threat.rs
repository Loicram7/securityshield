//! Definições de ameaças

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatType {
    Virus,
    Trojan,
    Worm,
    Spyware,
    Keylogger,
    Ransomware,
    Adware,
    Rootkit,
    Suspicious,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub id: String,
    pub name: String,
    pub threat_type: ThreatType,
    pub level: ThreatLevel,
    pub file_path: Option<String>,
    pub process_id: Option<u32>,
    pub description: String,
    pub detected_at: DateTime<Utc>,
}

impl Threat {
    pub fn new(name: String, threat_type: ThreatType, level: ThreatLevel) -> Self {
        use uuid::Uuid;
        
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            threat_type,
            level,
            file_path: None,
            process_id: None,
            description: String::new(),
            detected_at: Utc::now(),
        }
    }
}
