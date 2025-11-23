//! Módulo de antivírus

pub mod hashing;
pub mod signatures;
pub mod virustotal;
pub mod scanner;
pub mod quarantine;

pub use scanner::FileScanner;
pub use signatures::SignatureDatabase;
pub use virustotal::VirusTotalClient;
