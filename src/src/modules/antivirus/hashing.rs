//! Cálculo de hashes de arquivos

use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use anyhow::Result;

/// Calcula SHA256 de um arquivo
pub fn calculate_sha256(file_path: &Path) -> Result<String> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192]; // Buffer de 8KB
    
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    
    Ok(format!("{:x}", hasher.finalize()))
}

/// Calcula MD5 de um arquivo (alguns antivírus usam)
pub fn calculate_md5(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let digest = md5::compute(&buffer);
    Ok(format!("{:x}", digest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_sha256_empty_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"").unwrap();
        
        let hash = calculate_sha256(temp_file.path()).unwrap();
        // Hash SHA256 de string vazia
        assert_eq!(hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }
}
