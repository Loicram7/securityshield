//! Integração com VirusTotal API

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{info, warn};

#[derive(Debug, Clone)]
pub struct VirusTotalClient {
    api_key: String,
    client: Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct VTResponse {
    data: VTData,
}

#[derive(Debug, Deserialize)]
struct VTData {
    attributes: VTAttributes,
}

#[derive(Debug, Deserialize)]
struct VTAttributes {
    last_analysis_stats: AnalysisStats,
    last_analysis_results: HashMap<String, EngineResult>,
}

#[derive(Debug, Deserialize)]
struct AnalysisStats {
    malicious: u32,
    suspicious: u32,
    undetected: u32,
    harmless: u32,
}

#[derive(Debug, Deserialize)]
struct EngineResult {
    category: String,
    engine_name: String,
    result: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VTScanResult {
    pub file_hash: String,
    pub detections: u32,
    pub total_scanners: u32,
    pub is_malicious: bool,
    pub engines: Vec<EngineDetection>,
    pub scan_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineDetection {
    pub engine_name: String,
    pub result: String,
    pub category: String,
}

impl VirusTotalClient {
    /// Cria novo cliente VirusTotal
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://www.virustotal.com/api/v3".to_string(),
        }
    }
    
    /// Consulta hash no VirusTotal
    pub async fn scan_hash(&self, sha256: &str) -> Result<VTScanResult> {
        info!("Consultando VirusTotal para hash: {}...", &sha256[..16]);
        
        let url = format!("{}/files/{}", self.base_url, sha256);
        
        let response = self.client
            .get(&url)
            .header("x-apikey", &self.api_key)
            .send()
            .await
            .context("Erro ao conectar com VirusTotal")?;
        
        if response.status().as_u16() == 404 {
            // Hash não encontrado no VirusTotal
            warn!("Hash não encontrado no VirusTotal");
            return Ok(VTScanResult {
                file_hash: sha256.to_string(),
                detections: 0,
                total_scanners: 0,
                is_malicious: false,
                engines: vec![],
                scan_date: chrono::Utc::now().to_rfc3339(),
            });
        }
        
        let vt_response: VTResponse = response
            .json()
            .await
            .context("Erro ao parsear resposta do VirusTotal")?;
        
        let stats = vt_response.data.attributes.last_analysis_stats;
        let total = stats.malicious + stats.suspicious + stats.undetected + stats.harmless;
        
        // Coletar detecções de engines
        let mut engines = Vec::new();
        for (_, result) in vt_response.data.attributes.last_analysis_results.iter() {
            if result.category == "malicious" || result.category == "suspicious" {
                engines.push(EngineDetection {
                    engine_name: result.engine_name.clone(),
                    result: result.result.clone().unwrap_or_default(),
                    category: result.category.clone(),
                });
            }
        }
        
        info!("VirusTotal: {}/{} engines detectaram ameaça", stats.malicious + stats.suspicious, total);
        
        Ok(VTScanResult {
            file_hash: sha256.to_string(),
            detections: stats.malicious + stats.suspicious,
            total_scanners: total,
            is_malicious: stats.malicious > 0,
            engines,
            scan_date: chrono::Utc::now().to_rfc3339(),
        })
    }
    
    /// Verifica se API key está válida
    pub async fn test_connection(&self) -> Result<bool> {
        info!("Testando conexão com VirusTotal...");
        
        // Testar com hash conhecido (EICAR)
        let test_hash = "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f";
        
        match self.scan_hash(test_hash).await {
            Ok(_) => {
                info!("✓ Conexão com VirusTotal OK");
                Ok(true)
            },
            Err(e) => {
                warn!("✗ Erro ao conectar com VirusTotal: {}", e);
                Ok(false)
            }
        }
    }
}
