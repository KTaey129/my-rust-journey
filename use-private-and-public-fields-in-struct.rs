//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
#[derive(Debug)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// 
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// error occurs because there're private fields
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let mut config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// config.enable();
/// config.set_level(LogLevel::Info);
/// config.set_destination(LogOutput::Stderr);
/// ```
#[derive(Debug)]
pub struct Logging {
    pub enabled: bool,
    level: LogLevel,
    destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    // Add getters/setters to the Logging struct
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn get_level(&self) -> &LogLevel {
        &self.level
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn get_desitnation(&self) -> &LogOutput {
        &self.destination
    }

    pub fn set_destination(&mut self, destination: LogOutput) {
        self.destination = destination;
    }
}

// Experiment with Field Visibility in a New Struct:
// Create a struct in another module (e.g., src/settings.rs):
// pub struct Settings {
//     pub app_name: String,
//     version: String,
// }

// impl Settings {
//     pub fn new(app_name: String, version: String) -> Self {
//         Self { app_name, version }
//     }

//     pub fn get_version(&self) -> &str {
//         &self.version
//     }
// }

// This ensures the documentation examples align with the updated visibility rules and functionality.
// cargo test --doc

// src/main.rs
// use cli_utils::config::{Logging, LogLevel, LogOutput};


// fn main() {
//     let mut config = Logging::new();
//     config.set_level(LogLevel::Warn);
//     config.set_destination(LogOutput::Stderr);
//     println!("{:?}", config);
// }
