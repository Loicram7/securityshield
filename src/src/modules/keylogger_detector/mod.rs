//! Detector de keyloggers

mod process_monitor;
mod input_monitor;
mod detector;

pub use process_monitor::{ProcessMonitor, ProcessInfo};
pub use input_monitor::InputMonitor;
pub use detector::{KeyloggerDetector, DetectionResult};
