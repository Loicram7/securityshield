//! Scanner de arquivos principal

use crate::modules::antivirus::{hashing, signatures, virustotal};
use crate::core::threat::{Threat, ThreatLevel, ThreatType};
use std::path::Path;
use anyhow::Result;
use log::{info, warn};

pub struct FileScanner {
    signature_db: signatures::SignatureDatabase,
    vt_client: Option<virustotal::VirusTotalClient>,
    online_scan_enabled: bool,
}

#[derive(Debug)]
pub struct ScanResult {
    pub file_path: String,
    pub file_size: u64,
    pub sha256: String,
    pub md5: String,
    pub is_infected: bool,
    pub threats: Vec<Threat>,
    pub scan_time_ms: u128,
}

impl FileScanner {
    /// Criar novo scanner
    pub fn new(vt_api_key: Option<String>) -> Self {
        let vt_client = vt_api_key.map(|key| virustotal::VirusTotalClient::new(key));
        let online_scan_enabled = vt_client.is_some();  // ← Calcula ANTES de mover
        
        Self {
            signature_db: signatures::SignatureDatabase::load_default(),
            vt_client,
            online_scan_enabled,
        }
    }
    
    /// Escanear arquivo
    pub async fn scan_file<P: AsRef<Path>>(&self, file_path: P) -> Result<ScanResult> {
        let path = file_path.as_ref();
        let start_time = std::time::Instant::now();
        
        info!("Escaneando arquivo: {}", path.display());
        
        // Obter informações do arquivo
        let metadata = std::fs::metadata(path)?;
        let file_size = metadata.len();
        
        // Calcular hashes
        info!("  → Calculando hashes...");
        let sha256 = hashing::calculate_sha256(path)?;
        let md5 = hashing::calculate_md5(path)?;
        
        info!("  → SHA256: {}...", &sha256[..16]);
        
        let mut threats = Vec::new();
        
        // FASE 1: Verificar assinaturas locais
        info!("  → Verificando assinaturas locais...");
        if let Some(sig) = self.signature_db.find_by_sha256(&sha256) {
            info!("  ✓ Detectado por assinatura local: {}", sig.name);
            
            let mut threat = Threat::new(
                sig.name.clone(),
                self.parse_threat_type(&sig.threat_type),
                ThreatLevel::High,
            );
            threat.file_path = Some(path.display().to_string());
            threat.description = sig.description.clone();
            
            threats.push(threat);
        }
        
        // FASE 2: Consultar VirusTotal (se habilitado e não detectado localmente)
        if self.online_scan_enabled && threats.is_empty() {
            if let Some(vt_client) = &self.vt_client {
                info!("  → Consultando VirusTotal...");
                
                match vt_client.scan_hash(&sha256).await {
                    Ok(vt_result) => {
                        if vt_result.is_malicious {
                            info!("  ✓ VirusTotal: {}/{} engines detectaram", 
                                  vt_result.detections, vt_result.total_scanners);
                            
                            let mut threat = Threat::new(
                                format!("Malware detectado por {} engines", vt_result.detections),
                                ThreatType::Unknown,
                                if vt_result.detections >= 5 { 
                                    ThreatLevel::Critical 
                                } else { 
                                    ThreatLevel::High 
                                },
                            );
                            
                            threat.file_path = Some(path.display().to_string());
                            threat.description = format!(
                                "Detectado por {}/{} antivírus no VirusTotal. Engines: {}",
                                vt_result.detections,
                                vt_result.total_scanners,
                                vt_result.engines.iter()
                                    .take(3)
                                    .map(|e| e.engine_name.as_str())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            );
                            
                            threats.push(threat);
                        } else {
                            info!("  ✓ VirusTotal: Limpo ({}/{})", 
                                  vt_result.detections, vt_result.total_scanners);
                        }
                    },
                    Err(e) => {
                        warn!("  ✗ Erro ao consultar VirusTotal: {}", e);
                    }
                }
            }
        }
        
        let scan_time_ms = start_time.elapsed().as_millis();
        let is_infected = !threats.is_empty();
        
        if is_infected {
            warn!("  🚨 ARQUIVO INFECTADO!");
        } else {
            info!("  ✓ Arquivo limpo");
        }
        
        info!("  → Scan concluído em {}ms", scan_time_ms);
        
        Ok(ScanResult {
            file_path: path.display().to_string(),
            file_size,
            sha256,
            md5,
            is_infected,
            threats,
            scan_time_ms,
        })
    }
    
    fn parse_threat_type(&self, type_str: &str) -> ThreatType {
        match type_str.to_lowercase().as_str() {
            "virus" => ThreatType::Virus,
            "trojan" => ThreatType::Trojan,
            "worm" => ThreatType::Worm,
            "spyware" => ThreatType::Spyware,
            "keylogger" => ThreatType::Keylogger,
            "ransomware" => ThreatType::Ransomware,
            "adware" => ThreatType::Adware,
            "rootkit" => ThreatType::Rootkit,
            _ => ThreatType::Unknown,
        }
    }
}
