//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//! 
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

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
/// ```
/// 
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
/// 
/// Example of setting the logging destination to a file:
/// ```
/// use cli_utils::config::{Logging, LogOutput};
/// let mut config = Logging::new();
/// config.destination = LogOutput::File(String::new());
/// match config.destination {
///     LogOutput::File(ref path) if path.is_empty() => assert!(true),
///     _ => panic!("Expected empty file path"),
/// }
/// ```
/// 
// Prevents this doctest from being executed during cargo test
/// ```rust,ignore
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// assert_eq!(config.enabled, true);
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,   
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}


// Parallel Tests
// cargo test -- --test-threads=4


// Use the #[cfg(not(doctest))] attribute to conditionally compile a module only when doctests are not running.
// #[cfg(not(doctest))]
// pub mod config {
//     // The content of this module will be excluded from doctests.
//     pub fn example_function() {
//         println!("This won't be tested in doctests.");
//     }
// }

// Add the #![cfg(doctest)] attribute at the top of the file to run only during doctests. This disables the file from regular compilation.
// #![cfg(doctest)]

// Modify the Cargo.toml file to exclude specific files or modules from doctests using the exclude option under [package] or specific doctest-related flags.
// [package]
// name = "cli-utils"
// version = "0.1.0"
// edition = "2021"

// [lib]
// doctest = false  # Disable doctests for the library target

// Use #[cfg(doctest)] to include code only during doctests.
// #[cfg(doctest)]
// mod doctest_helpers {
//     pub fn helper_function() -> u32 {
//         42
//     }
// }

// /// ```
// /// // This doctest will use the helper function only when running doctests.
// /// use crate::doctest_helpers::helper_function;
// /// assert_eq!(helper_function(), 42);
// /// ```

// By default, Rust runs doctests in parallel. To disable parallel execution for debugging or other reasons, use the --test-threads=1 flag with cargo test:
// cargo test -- --test-threads=1
