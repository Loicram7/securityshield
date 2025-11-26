//! Teste do analisador de Wi-Fi

use securityshield::modules::wifi_analyzer::WifiAnalyzer;

#[tokio::main]
async fn main() {
    env_logger::init();
    
    println!("📶 Analisador de Wi-Fi\n");
    
    let analyzer = WifiAnalyzer::new();
    
    println!("🔍 Escaneando redes...\n");
    
    match analyzer.analyze() {
        Ok(result) => {
            println!("📊 RESULTADO:");
            println!("  Redes encontradas: {}", result.networks.len());
            println!("  Redes seguras: {}", result.secure_networks);
            println!("  Redes inseguras: {}", result.insecure_networks);
            println!("  Ameaças detectadas: {}", result.threats.len());
            println!();
            
            if !result.networks.is_empty() {
                println!("📡 REDES DISPONÍVEIS:\n");
                for network in &result.networks {
                    let quality = network.signal_quality();
                    let security_icon = if network.security.is_secure() { "🔒" } else { "🔓" };
                    
                    println!("  {} {} ({})", 
                             security_icon, 
                             network.ssid, 
                             network.security.risk_level());
                    println!("     BSSID: {}", network.bssid);
                    println!("     Sinal: {}% ({} dBm)", quality, network.signal_strength);
                    println!("     Canal: {} | Freq: {} MHz", network.channel, network.frequency);
                    println!("     Segurança: {:?}", network.security);
                    println!();
                }
            }
            
            if !result.threats.is_empty() {
                println!("🚨 AMEAÇAS DETECTADAS:\n");
                for threat in &result.threats {
                    println!("  • {} (Nível: {:?})", threat.name, threat.level);
                    println!("    {}", threat.description);
                    println!();
                }
            }
            
            println!("💡 RECOMENDAÇÕES:\n");
            for rec in analyzer.get_recommendations(&result) {
                println!("  {}", rec);
            }
            
            if result.networks.is_empty() {
                println!("\n⚠️  Nenhum adaptador Wi-Fi encontrado.");
                println!("   Isso é normal em desktops sem Wi-Fi.");
                println!("   O código funcionará em laptops/Android!");
            }
        },
        Err(e) => {
            eprintln!("❌ Erro ao analisar Wi-Fi: {}", e);
            eprintln!("\n💡 Dica: Instale NetworkManager se não tiver:");
            eprintln!("   sudo apt install network-manager");
        }
    }
}
