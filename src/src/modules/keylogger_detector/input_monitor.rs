//! Monitor de dispositivos de entrada

use std::path::Path;
use std::fs;
use anyhow::Result;
use log::info;

#[derive(Debug, Clone)]
pub struct InputDevice {
    pub path: String,
    pub name: String,
}

pub struct InputMonitor;

impl InputMonitor {
    pub fn new() -> Self {
        Self
    }
    
    pub fn list_input_devices() -> Result<Vec<String>> {
        let input_dir = Path::new("/dev/input");
        
        if !input_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut devices = Vec::new();
        
        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with("event") {
                        devices.push(path.display().to_string());
                    }
                }
            }
        }
        
        info!("Encontrados {} dispositivos de entrada", devices.len());
        Ok(devices)
    }
    
    pub fn check_input_access(&self) -> Result<Vec<InputDevice>> {
        let devices = Self::list_input_devices()?;
        let mut result = Vec::new();
        
        for device_path in devices {
            let device = InputDevice {
                path: device_path.clone(),
                name: self.get_device_name(&device_path)?,
            };
            result.push(device);
        }
        
        Ok(result)
    }
    
    fn get_device_name(&self, path: &str) -> Result<String> {
        if let Some(name) = Path::new(path).file_name() {
            if let Some(name_str) = name.to_str() {
                return Ok(name_str.to_string());
            }
        }
        Ok("unknown".to_string())
    }
}
