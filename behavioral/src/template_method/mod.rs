pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::template_method::std::*;
  //! 
  //! let mut player = MediaPlayer::new();
  //! 
  //! ```
  
  use serde;
  use serde_json; 
  use csv;
  use std::fs::File;
  use std::error::Error;
  use std::io::prelude::*;

  #[derive(Serialize, Deserialize, RustcDecodable, Debug)]
  pub struct Person {
    pub name: Option<String>,
    pub age: Option<i32>,
    pub address: Option<String>,
  }
  impl Person {
    pub fn new(name: &str, age: i32, address: &str) -> Person {
      Person { name: Some(String::from(name)), age: Some(age), address: Some(String::from(address)) }
    }
  }

  pub trait DataFinder<T, Y> {
    fn find(&self, f: fn(T) -> Option<Y>) -> Option<Y> {
      let data = self.read_data();
      let parsed = self.parse(data);
      let res = f(parsed);
      self.cleanup();
      res
    }

    fn read_data(&self) -> String;
    fn parse(&self, data: String) -> T;
    fn cleanup(&self);
  }

  pub struct JsonDataFinder;
  impl DataFinder<Vec<Person>, Person> for JsonDataFinder {
    fn read_data(&self) -> String {
      let mut contents = String::new();
      if let Ok(mut file) = File::open("persons.json") {     
        file.read_to_string(&mut contents);
      };
      contents
    }
    fn parse(&self, data: String) -> Vec<Person> {
      if let Ok(p) = serde_json::from_str(&data[..]) {
        p
      } else {
        Vec::<Person>::new()
      }
    }
    fn cleanup(&self) {
      println!("Reading json: nothing to do.")
    }
  }

  pub struct CsvDataFinder;
  impl DataFinder<Vec<Person>, Person> for CsvDataFinder {
    fn read_data(&self) -> String {
      let mut contents = String::new();
      if let Ok(mut file) = File::open("persons.csv") {     
        file.read_to_string(&mut contents);
      };
      contents
    }
    fn parse(&self, data: String) -> Vec<Person> {
      let mut v: Vec<Person> = Vec::new();
      let mut rdr = csv::Reader::from_string(data);
      for record in rdr.decode() {
        if let Ok(person) = record {
          v.push(person);
        };
      };
      v  
    }
    fn cleanup(&self) {
      println!("Reading csv: nothing to do.")
    }
  }

}