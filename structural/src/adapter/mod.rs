//! `adapter` module.
//!
//! # Example
//! 
//! ``` rust
//! use structural::adapter::*;
//! 
//! let logger =  AppLogger::new();
//! logger.log_info(String::from("This is an info message."));
//! logger.log_debug(String::from("Debug something here."));
//! logger.log_error(String::from("Show an error message."));
//! logger.log_warning(String::from("About to finish."));
//! logger.log_info(String::from("Bye!"));
//! ```

pub trait Log {
  fn log_info(&self, message: String);
  fn log_debug(&self, message: String);
  fn log_warning(&self, message: String);
  fn log_error(&self, message: String);
}

pub struct AppLogger {
  logger: logger::Logger,
}

impl AppLogger {
  pub fn new() -> AppLogger {
    AppLogger {
      logger: logger::Logger,
    }
  }
}
impl Log for AppLogger {
  fn log_info(&self, message: String) {
    self.logger.log(message, String::from("info"))
  }
  fn log_debug(&self, message: String) {
    self.logger.log(message, String::from("debug"))
  }
  fn log_warning(&self, message: String) {
    self.logger.log(message, String::from("warning"))
  }
  fn log_error(&self, message: String) {
    self.logger.log(message, String::from("error"))
  }
}

mod logger {
  pub struct Logger;
  impl Logger {
    pub fn log(&self, message: String, severity: String) {
      println!("{}: {}", severity.to_uppercase(), message);
    }
  }
}