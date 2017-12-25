//! `null_object` module.
//!
//! # Example
//! 
//! ```rust
//! use behavioral::null_object::*;
//! 
//! assert!(true);
//! ```

use std::sync::{Mutex, Arc};
use std::{thread, time};

#[derive(Debug, PartialEq, Hash)]
pub struct Message {
  number: i32,
}
impl Message {
  pub fn new(number: i32) -> Message {
    Message { number }
  }
}

pub struct DataGenerator {
  //max_val: i32,
  //max_time: i32,
  //is_stop: bool,
  queue: Arc<Mutex<Vec<i32>>>,
}
impl DataGenerator {
  pub fn new() -> DataGenerator {
    DataGenerator {
      //max_val: 10,
      //max_time: 10000,
      //is_stop: false,
      queue: Arc::new(Mutex::new(Vec::new())),
    }
  }
  pub fn run(&mut self) {
    let queue = Arc::clone(&self.queue);
    thread::spawn(move || {
      thread::sleep(time::Duration::from_millis(50));
      let mut num = queue.lock().unwrap();
      (*num).push(0);
    });
  }
  pub fn get_message(&mut self) -> Option<Message> {
    let queue = Arc::clone(&self.queue);
    let mut num = queue.lock().unwrap();
    if let Some(num) = (*num).pop() {
      Some(Message::new(num))
    } else {
      None
    }
  }
}



