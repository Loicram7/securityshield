//! Preditor de ameaças usando IA (Python)

use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use log::{info, warn};

#[derive(Debug, Serialize)]
pub struct ProcessData {
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub threads: u32,
    pub connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct PredictionResult {
    pub success: bool,
    pub score: Option<f64>,
    pub classification: Option<String>,
    pub error: Option<String>,
}

pub struct ThreatPredictor {
    python_path: String,
    script_path: String,
}

impl ThreatPredictor {
    pub fn new() -> Self {
    // Tentar obter caminho do projeto via variável de ambiente
    let project_root = std::env::var("SECURITYSHIELD_ROOT")
        .unwrap_or_else(|_| {
            // Fallback: usar diretório home
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}/projects/securityshield", home)
        });

    
    let script_path = format!("{}/python/ml_interface.py", project_root);
    
    Self {
        python_path: "python3".to_string(),
        script_path,
    }
}
    
    /// Prediz se processo é ameaça usando IA
    pub fn predict(&self, process: &ProcessData) -> Result<PredictionResult> {
        info!("Consultando IA para processo: {}", process.name);
        
        // Serializar para JSON
        let json_input = serde_json::to_string(process)
            .context("Erro ao serializar processo para JSON")?;
        
        // Chamar Python
        let output = Command::new(&self.python_path)
            .arg(&self.script_path)
            .arg(&json_input)
            .output()
            .context("Erro ao executar script Python. Python3 instalado?")?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Python script falhou: {}", stderr);
            return Ok(PredictionResult {
                success: false,
                score: None,
                classification: None,
                error: Some(stderr.to_string()),
            });
        }
        
        // Parse resultado
        let stdout = String::from_utf8(output.stdout)
            .context("Saída do Python não é UTF-8 válido")?;
        
        let result: PredictionResult = serde_json::from_str(&stdout)
            .context("Erro ao parsear resposta JSON do Python")?;
        
        if result.success {
            if let (Some(score), Some(class)) = (&result.score, &result.classification) {
                info!("  IA: Score={:.2}, Classificação={}", score, class);
            }
        }
        
        Ok(result)
    }
}
