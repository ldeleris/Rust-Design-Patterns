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