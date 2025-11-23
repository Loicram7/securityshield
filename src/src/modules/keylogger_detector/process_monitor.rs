//! Monitor de processos suspeitos

use sysinfo::{System, Process, Pid};
use std::collections::HashMap;
use anyhow::Result;
use log::{info, warn};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cmd: Vec<String>,
    pub cpu_usage: f32,
    pub memory: u64,
    pub open_files: usize,
}

pub struct ProcessMonitor {
    system: System,
    known_safe_processes: Vec<String>,
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            system,
            known_safe_processes: vec![
                "systemd".to_string(),
                "gnome-shell".to_string(),
                "Xorg".to_string(),
                "gdm".to_string(),
                "sddm".to_string(),
                "plasma".to_string(),
                "kwin".to_string(),
                "nautilus".to_string(),
                "konsole".to_string(),
                "gnome-terminal".to_string(),
                "test_keylogger".to_string(),
                "securityshield".to_string(),
                "cargo".to_string(), 
            ],
        }
    }
    
    pub fn scan_processes(&mut self) -> Result<Vec<ProcessInfo>> {
        info!("Escaneando processos...");
        
        self.system.refresh_processes();
        
        let mut suspicious = Vec::new();
        
        for (pid, process) in self.system.processes() {
            if self.is_suspicious_process(process) {
                let info = self.extract_process_info(*pid, process);
                warn!("Processo suspeito detectado: {} (PID: {})", info.name, info.pid);
                suspicious.push(info);
            }
        }
        
        info!("Encontrados {} processos suspeitos", suspicious.len());
        Ok(suspicious)
    }
    
    fn is_suspicious_process(&self, process: &Process) -> bool {
        let name = process.name();
        
        if self.known_safe_processes.iter().any(|safe| name.contains(safe)) {
            return false;
        }
        
        let cpu_usage = process.cpu_usage();
        let memory = process.memory();
        
        let suspicious_names = ["key", "log", "capture", "hook", "spy", "monitor"];
        let has_suspicious_name = suspicious_names.iter()
            .any(|pattern| name.to_lowercase().contains(pattern));
        
        let low_cpu_active = cpu_usage > 0.0 && cpu_usage < 2.0;
        let low_memory = memory < 50_000_000;
        
        has_suspicious_name && low_cpu_active && low_memory
    }
    
    fn extract_process_info(&self, pid: Pid, process: &Process) -> ProcessInfo {
        ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            cmd: process.cmd().to_vec(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            open_files: 0,
        }
    }
}
