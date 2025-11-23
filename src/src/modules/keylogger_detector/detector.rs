//! Detector principal de keyloggers

use crate::modules::keylogger_detector::{ProcessMonitor, InputMonitor};
use crate::core::threat::{Threat, ThreatLevel, ThreatType};
use anyhow::Result;
use log::info;

pub struct KeyloggerDetector {
    process_monitor: ProcessMonitor,
    input_monitor: InputMonitor,
}

#[derive(Debug)]
pub struct DetectionResult {
    pub threats: Vec<Threat>,
    pub suspicious_processes: usize,
}

impl KeyloggerDetector {
    pub fn new() -> Self {
        Self {
            process_monitor: ProcessMonitor::new(),
            input_monitor: InputMonitor::new(),
        }
    }
    
    pub fn scan(&mut self) -> Result<DetectionResult> {
        info!("Iniciando detecção de keyloggers...");
        
        let mut threats = Vec::new();
        
        let suspicious_processes = self.process_monitor.scan_processes()?;
        
        for proc in &suspicious_processes {
            let mut threat = Threat::new(
                format!("Processo suspeito: {}", proc.name),
                ThreatType::Keylogger,
                ThreatLevel::High,
            );
            
            threat.process_id = Some(proc.pid);
            threat.description = format!(
                "Processo com comportamento suspeito. PID: {}, CPU: {:.1}%",
                proc.pid, proc.cpu_usage
            );
            
            threats.push(threat);
        }
        
        info!("Detecção concluída. {} ameaça(s)", threats.len());
        
        Ok(DetectionResult {
            threats,
            suspicious_processes: suspicious_processes.len(),
        })
    }
}
