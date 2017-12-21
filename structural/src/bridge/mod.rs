pub mod key {
  pub trait Key {
    fn open(&self) -> String;
    fn close(&self) -> String;
  }
  
  pub struct CarDoorKey;
  impl Key for CarDoorKey {
    fn open(&self) -> String {
      String::from("I'm pushing down the button of remote control.")
    }
    fn close(&self) -> String {
      String::from("I'm pushing up the button of remote control.")
    }
  }
  
  pub struct HouseDoorKey;
  impl Key for HouseDoorKey {
    fn open(&self) -> String {
      String::from("I'm turning the key in the right side.")
    }
    fn close(&self) -> String {
      String::from("I'm turning the key in the left side.")
    }  
  }
  
  struct Door {
    key: Box<Key>,
  }
  
  impl Door {
    fn new(key: Box<Key>) -> Door {
      Door { key }
    }
  
    fn open_the_door(&self) -> String { self.key.open() }
    fn close_the_door(&self) -> String { self.key.close() }
    fn prevent_owner(&self) -> String {
      String::from("Hi Owner! You have a guest.")
    }
  }
  
  pub struct HouseOneDoor {
    door: Door,
  }
  
  impl HouseOneDoor {
    pub fn new(key: Box<Key>) -> HouseOneDoor {
      HouseOneDoor {
        door: Door::new(key),
      }
    }
    pub fn enter(&self) -> String {
      format!("{}\n{}",
        self.door.prevent_owner(),
        self.door.open_the_door()
      )
    }
    pub fn leave(&self) -> String {
      self.door.close_the_door()
    }
  }
  
  pub struct CarOneDoor {
    door: Door,
  }
  
  impl CarOneDoor {
    pub fn new(key: Box<Key>) -> HouseOneDoor {
      HouseOneDoor {
        door: Door::new(key),
      }
    }
    pub fn enter(&self) -> String {
      format!("{} You can park the car.", self.door.open_the_door())
    }
    pub fn leave(&self) -> String {
      format!("{} Have a good trip.", self.door.close_the_door())
    }
  }
}