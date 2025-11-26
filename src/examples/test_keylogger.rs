//! Teste do detector de keylogger

use securityshield::modules::keylogger_detector::KeyloggerDetector;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("⌨️  Teste do Detector de Keylogger\n");
    
    let mut detector = KeyloggerDetector::new();
    
    println!("🔍 Escaneando sistema...\n");
    
    match detector.scan() {
        Ok(result) => {
            println!("📊 RESULTADO:");
            println!("  Processos suspeitos: {}", result.suspicious_processes);
            println!("  Ameaças detectadas: {}", result.threats.len());
            println!();
            
            if result.threats.is_empty() {
                println!("✅ NENHUM KEYLOGGER DETECTADO!");
            } else {
                println!("🚨 AMEAÇAS DETECTADAS:");
                for threat in &result.threats {
                    println!("\n  • {}", threat.name);
                    println!("    Nível: {:?}", threat.level);
                    if let Some(pid) = threat.process_id {
                        println!("    PID: {}", pid);
                    }
                    println!("    Descrição: {}", threat.description);
                }
            }
        },
        Err(e) => {
            eprintln!("❌ Erro ao escanear: {}", e);
        }
    }
}
