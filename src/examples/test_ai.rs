//! Teste do módulo de IA

use securityshield::ai::ThreatPredictor;
use securityshield::ai::predictor::ProcessData;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("🤖 Teste do Módulo de IA\n");
    
    let predictor = ThreatPredictor::new();
    
    // Teste 1: Processo SEGURO
    println!("📊 Teste 1: Processo SEGURO (Chrome)");
    let safe_process = ProcessData {
        name: "chrome".to_string(),
        cpu_usage: 15.0,
        memory: 1_000_000_000,  // 1GB
        threads: 20,
        connections: 5,
    };
    
    match predictor.predict(&safe_process) {
        Ok(result) => {
            if result.success {
                println!("  Score: {:.2}", result.score.unwrap_or(0.0));
                println!("  Classificação: {}", result.classification.unwrap_or_default());
            } else {
                println!("  ❌ Erro: {}", result.error.unwrap_or_default());
            }
        },
        Err(e) => {
            eprintln!("  ❌ Erro ao chamar IA: {}", e);
        }
    }
    
    println!();
    
    // Teste 2: Processo MALICIOSO
    println!("🚨 Teste 2: Processo MALICIOSO (Keylogger)");
    let malicious_process = ProcessData {
        name: "keylogger".to_string(),
        cpu_usage: 2.0,
        memory: 50_000_000,  // 50MB
        threads: 50,
        connections: 80,
    };
    
    match predictor.predict(&malicious_process) {
        Ok(result) => {
            if result.success {
                let score = result.score.unwrap_or(0.0);
                let classification = result.classification.unwrap_or_default();
                
                println!("  Score: {:.2}", score);
                println!("  Classificação: {}", classification);
                
                if classification == "threat" {
                    println!("  ✅ IA detectou corretamente como AMEAÇA!");
                }
            } else {
                println!("  ❌ Erro: {}", result.error.unwrap_or_default());
            }
        },
        Err(e) => {
            eprintln!("  ❌ Erro ao chamar IA: {}", e);
        }
    }
    
    println!();
    
    // Teste 3: Processo SUSPEITO
    println!("⚠️  Teste 3: Processo SUSPEITO");
    let suspicious_process = ProcessData {
        name: "unknown_process".to_string(),
        cpu_usage: 5.0,
        memory: 200_000_000,  // 200MB
        threads: 30,
        connections: 25,
    };
    
    match predictor.predict(&suspicious_process) {
        Ok(result) => {
            if result.success {
                println!("  Score: {:.2}", result.score.unwrap_or(0.0));
                println!("  Classificação: {}", result.classification.unwrap_or_default());
            } else {
                println!("  ❌ Erro: {}", result.error.unwrap_or_default());
            }
        },
        Err(e) => {
            eprintln!("  ❌ Erro ao chamar IA: {}", e);
        }
    }
}
