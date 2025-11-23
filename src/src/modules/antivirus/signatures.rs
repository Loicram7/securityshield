//! Banco de assinaturas de malware

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub id: u32,
    pub name: String,
    pub threat_type: String,
    pub hash_sha256: Option<String>,
    pub hash_md5: Option<String>,
    pub pattern: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureDatabase {
    pub version: String,
    pub last_updated: String,
    pub signatures: Vec<Signature>,
}

impl SignatureDatabase {
    /// Carrega banco de assinaturas de arquivo JSON
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let db: SignatureDatabase = serde_json::from_str(&content)?;
        Ok(db)
    }
    
    /// Carrega banco padrão (embedded)
    pub fn load_default() -> Self {
        // Banco básico inicial (EICAR test file)
        Self {
            version: "1.0.0".to_string(),
            last_updated: chrono::Utc::now().to_rfc3339(),
            signatures: vec![
                Signature {
                    id: 1,
                    name: "EICAR-Test-File".to_string(),
                    threat_type: "test".to_string(),
                    hash_sha256: Some("275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f".to_string()),
                    hash_md5: Some("44d88612fea8a8f36de82e1278abb02f".to_string()),
                    pattern: None,
                    description: "Arquivo de teste EICAR (não malicioso)".to_string(),
                },
            ],
        }
    }
    
    /// Busca assinatura por hash SHA256
    pub fn find_by_sha256(&self, hash: &str) -> Option<&Signature> {
        self.signatures.iter()
            .find(|sig| sig.hash_sha256.as_ref() == Some(&hash.to_string()))
    }
    
    /// Busca assinatura por hash MD5
    pub fn find_by_md5(&self, hash: &str) -> Option<&Signature> {
        self.signatures.iter()
            .find(|sig| sig.hash_md5.as_ref() == Some(&hash.to_string()))
    }
    
    /// Salva banco em arquivo
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
