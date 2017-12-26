//! `strategy` module.
//!

pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust,ignore
  //! use behavioral::strategy::*;
  //! 
  //! let csv_people = ParserFactory::new(".csv");
  //! let json_people = ParserFactory::new(".json");
  //! let application_csv = PersonApplication::new(csv_people);
  //! let application_json = PersonApplication::new(json_people);
  //! 
  //! println!("{:#?}", application_csv.write("persons.csv"));
  //! println!("{:#?}", application_json.write("persons.json"));
  //! ```

  use serde;
  use serde_json; 
  use csv;
  use std::fs::File;
  use std::error::Error;

  #[derive(Serialize, Deserialize, RustcDecodable, Debug)]
  pub struct Person {
    name: String,
    age: i32,
    location: String,
  }

  pub trait Parser<T> {
    fn parse(&self, file: &str) ->  Result<Vec<Person>, Box<Error>>;
  }

  pub struct CSVParser;

  impl Parser<Person> for CSVParser {
    fn parse(&self, file: &str) ->  Result<Vec<Person>, Box<Error>> {
      let mut v: Vec<Person> = Vec::new();
      let mut rdr = csv::Reader::from_file(file)?;
      for record in rdr.decode() {
          let person: Person = record?;
          v.push(person);
      };
      Ok(v)   
    }
  }

  pub struct JsonParser;

  impl Parser<Person> for JsonParser {
    fn parse(&self, file: &str) -> Result<Vec<Person>, Box<Error>> {
      let file = File::open(file)?;
      let p: Vec<Person> = serde_json::from_reader(file)?;
      Ok(p)
    }
  }

  pub struct ParserFactory;
  impl ParserFactory {
    pub fn new(file: &str) -> Box<Parser<Person>> {
      match file {
        f if f.ends_with(".json") => Box::new(JsonParser),
        f if f.ends_with(".csv") => Box::new(CSVParser),
        f => panic!(format!("Unknown format: {}", f)),
      }
    }
  }

  use std::fmt::Debug; 

  pub struct PersonApplication<T> {
    parser: Box<Parser<T>>,
  }
  impl<T: Debug> PersonApplication<T> {
    pub fn new(parser: Box<Parser<T>>) -> PersonApplication<T> {
      PersonApplication::<T> { parser }
    }
    pub fn write(&self, file: &str) -> String {
      format!("Got the following data {:#?}", self.parser.parse(file))
    }
  }
}


pub mod opt {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust,ignore
  //! use behavioral::strategy::*;
  //! 
  //! let csv_people = ParserFactory::new(".csv");
  //! let json_people = ParserFactory::new(".json");
  //! let application_csv = PersonApplication::new(csv_people);
  //! let application_json = PersonApplication::new(json_people);
  //! 
  //! println!("{:#?}", application_csv.write("persons.csv"));
  //! println!("{:#?}", application_json.write("persons.json"));
  //! ```

  use serde;
  use serde_json; 
  use csv;
  use std::fs::File;
  use std::error::Error;

  #[derive(Serialize, Deserialize, RustcDecodable, Debug)]
  pub struct Person {
    name: Option<String>,
    age: Option<i32>,
    location: Option<String>,
  }

  pub trait Parser<T> {
    fn parse(&self, file: &str) ->  Result<Vec<Person>, Box<Error>>;
  }

  pub struct CSVParser;

  impl Parser<Person> for CSVParser {
    fn parse(&self, file: &str) ->  Result<Vec<Person>, Box<Error>> {
      let mut v: Vec<Person> = Vec::new();
      let mut rdr = csv::Reader::from_file(file)?;
      for record in rdr.decode() {
          let person: Person = record?;
          v.push(person);
      };
      Ok(v)   
    }
  }

  pub struct JsonParser;

  impl Parser<Person> for JsonParser {
    fn parse(&self, file: &str) -> Result<Vec<Person>, Box<Error>> {
      let file = File::open(file)?;
      let p: Vec<Person> = serde_json::from_reader(file)?;
      Ok(p)
    }
  }

  pub struct ParserFactory;
  impl ParserFactory {
    pub fn new(file: &str) -> Box<Parser<Person>> {
      match file {
        f if f.ends_with(".json") => Box::new(JsonParser),
        f if f.ends_with(".csv") => Box::new(CSVParser),
        f => panic!(format!("Unknown format: {}", f)),
      }
    }
  }

  use std::fmt::Debug; 

  pub struct PersonApplication<T> {
    parser: Box<Parser<T>>,
  }
  impl<T: Debug> PersonApplication<T> {
    pub fn new(parser: Box<Parser<T>>) -> PersonApplication<T> {
      PersonApplication::<T> { parser }
    }
    pub fn write(&self, file: &str) -> String {
      format!("Got the following data {:#?}", self.parser.parse(file))
    }
  }
}


pub mod closure {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust,ignore
  //! use strategy::closure::*;
  //! 
  //! let application_csv = Application::new(StrategyFactory::new("persons.csv"));
  //! let application_json = Application::new(StrategyFactory::new("persons.json"));
  //! 
  //! println!("{}", application_csv.write("persons.csv"));
  //! println!("{}", application_json.write("persons.json"));
  //! ```

  use serde;
  use serde_json; 
  use csv;
  use std::fs::File;
  use std::error::Error;

  #[derive(Serialize, Deserialize, RustcDecodable, Debug)]
  pub struct Person {
    name: Option<String>,
    age: Option<i32>,
    location: Option<String>,
  }

  use std::fmt::Debug;
  pub struct Application<T> {
    strategy: Box<Fn(&str) -> Result<Vec<T>, Box<Error>>>,
  }
  impl<T: 'static + Debug> Application<T> {
    pub fn new(strategy: Box<Fn(&str) -> Result<Vec<T>, Box<Error>>>) -> Application<T> {
      Application::<T> {
        strategy,
      }
    }
    pub fn write(&self, file: &str) -> String {
      format!("Got the following data {:#?}", (self.strategy)(file))
    }
  }

  pub struct StrategyFactory;
  impl StrategyFactory {
    pub fn new(file: &str) -> Box<Fn(&str) -> Result<Vec<Person>, Box<Error>>> {
      match file {
        f if f.ends_with(".json") => Box::new(StrategyFactory::parse_json),
        f if f.ends_with(".csv") => Box::new(StrategyFactory::parse_csv),
        f => panic!(format!("Unknown format: {}", f)),
      }
    }
    pub fn parse_csv(file: &str) ->  Result<Vec<Person>, Box<Error>> {
      let mut v: Vec<Person> = Vec::new();
      let mut rdr = csv::Reader::from_file(file)?;
      for record in rdr.decode() {
          let person: Person = record?;
          v.push(person);
      };
      Ok(v)   
    } 
    pub fn parse_json(file: &str) -> Result<Vec<Person>, Box<Error>> {
      let file = File::open(file)?;
      let p: Vec<Person> = serde_json::from_reader(file)?;
      Ok(p)
    }   
  } 
}