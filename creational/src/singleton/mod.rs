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

  pub fn print() {
    unsafe {
      if let Some(ref mut app_registry) = INSTANCE {
        app_registry._print();
      } else {
        println!("Application registry not initialized")
      };
    }    
  }

  fn _print(&self) {
    println!("{:?}", self.users);
  }
}
