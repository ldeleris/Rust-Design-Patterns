//! `singleton` module.
//!
//! # Example
//! 
//! ``` rust
//! use singleton::*;
//! use std::{thread, time};
//! 
//! println!("{}", AppRegistry::print());
//! println!("Sleeping for 5 seconds.");
//! thread::sleep(time::Duration::from_secs(1));
//! println!("I woke up.");
//! AppRegistry::add_user("1", "Laurent");
//! println!("{}", AppRegistry::print());
//! AppRegistry::add_user("2", "Pierre");
//! AppRegistry::add_user("3", "Angel");
//! println!("Is user with ID=1 registred?: {}", AppRegistry::is_user_registred("1"));
//! println!("Removing ID=2");
//! AppRegistry::remove_user("2");
//! println!("Is user with ID=2 registred?: {}", AppRegistry::is_user_registred("2"));
//! println!("All users registred are: {:?}", AppRegistry::get_all_user_names());
//! ```

use std::collections::HashMap;

static mut INSTANCE: Option<AppRegistry> = None;

#[derive(Debug)]
pub struct AppRegistry {
  users: HashMap<&'static str, &'static str>,
}

impl AppRegistry {
  fn init() {
    println!("Registry initialization block called");
    unsafe {
      INSTANCE = Some(AppRegistry { users: HashMap::new() });
    }
  }

  pub fn add_user(id: &'static str, name: &'static str) {
    unsafe {
      if let Some(ref mut app_registry) = INSTANCE {
        app_registry._add_user(id, name);
      } else {
        AppRegistry::init();
        if let Some(ref mut app_registry) = INSTANCE {
        app_registry._add_user(id, name);
        };
      };
    }
  }

  fn _add_user(&mut self, id: &'static str, name: &'static str) {
    self.users.insert(id, name);
  }

  pub fn remove_user(id: &'static str) {
    unsafe {
      if let Some(ref mut app_registry) = INSTANCE {
        app_registry._remove_user(id);
      } else {
        panic!("Application registry not initialized");
      };
    }   
  }

  fn _remove_user(&mut self, id: &'static str) {
    self.users.remove(id);
  }

  pub fn is_user_registred(id: &'static str) -> bool {
    unsafe {
      if let Some(ref mut app_registry) = INSTANCE {
        app_registry._is_user_registred(id)
      } else {
        panic!("Application registry not initialized");
      }
    }
  }

  fn _is_user_registred(&self, id: &'static str) -> bool {
    self.users.contains_key(id)
  }

  pub fn get_all_user_names() -> Vec<&'static str> {
    unsafe {
      if let Some(ref mut app_registry) = INSTANCE {
        app_registry._get_all_user_names()
      } else {
        panic!("Application registry not initialized");
      }
    }
  }

  fn _get_all_user_names(&self) -> Vec<&'static str> {
    let mut names: Vec<&'static str> = Vec::new();
    for (_, name) in &self.users {
      names.push(name);
    }
    names
  }

  pub fn print() -> String {
    unsafe {
      if let Some(_) = INSTANCE {
        format!("Application registry initialized")
      } else {
        format!("Application registry not initialized")
      }
    }    
  }
}
