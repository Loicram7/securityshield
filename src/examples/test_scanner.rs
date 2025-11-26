//! Teste do scanner de antivírus

use securityshield::modules::antivirus::FileScanner;
use std::path::Path;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("🦠 Teste do Scanner de Antivírus\n");
    
    // Criar scanner (sem VirusTotal por enquanto)
    let scanner = FileScanner::new(None);
    
    // Arquivo para testar
    let test_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Uso: cargo run --example test_scanner <caminho_do_arquivo>");
            std::process::exit(1);
        });
    
    println!("📁 Arquivo: {}", test_file);
    println!("🔍 Escaneando...\n");
    
    match scanner.scan_file(Path::new(&test_file)).await {
        Ok(result) => {
            println!("📊 RESULTADO:");
            println!("  Arquivo: {}", result.file_path);
            println!("  Tamanho: {} bytes", result.file_size);
            println!("  SHA256: {}", result.sha256);
            println!("  MD5: {}", result.md5);
            println!("  Tempo: {}ms", result.scan_time_ms);
            println!();
            
            if result.is_infected {
                println!("🚨 INFECTADO! {} ameaça(s) detectada(s):", result.threats.len());
                for threat in &result.threats {
                    println!("  • {} ({})", threat.name, threat.description);
                }
            } else {
                println!("✅ ARQUIVO LIMPO!");
            }
        },
        Err(e) => {
            eprintln!("❌ Erro ao escanear: {}", e);
        }
    }
}
