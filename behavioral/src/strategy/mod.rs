//! `strategy` module.
//!
//! # Example
//! 
//! ```rust
//! use behavioral::strategy::*;
//! 
//! let csv_people = ParserFactory::new(".csv");
//! let json_people = ParserFactory::new(".json");
//! let application_csv = PersonApplication::new(csv_people);
//! let application_json = PersonApplication::new(json_people);
//! 
//! println!("{}", application_csv.write("people.csv"));
//! println!("{}", application_json.write("people.json"));
//! ```

use serde;
use serde_json; 
use csv;
use std::fs::File;

//use serde_json::Error;

#[derive(Serialize, Deserialize, RustcDecodable, Debug)]
pub struct Person {
  name: String,
  age: i32,
  location: String,
}

pub trait Parser<T> {
  fn parse(&self, file: &str) -> Vec<T>;
}

pub struct CSVParser;

impl Parser<Person> for CSVParser {
  fn parse(&self, file: &str) -> Vec<Person> {
    let mut v: Vec<Person> = Vec::new();
    let mut rdr = csv::Reader::from_file(file).unwrap();
    for record in rdr.decode() {
        let person: Person = record.unwrap();
        v.push(person);
    };
    v     
  }
}

pub struct JsonParser;

impl Parser<Person> for JsonParser {
  fn parse(&self, file: &str) -> Vec<Person> {
    let file = File::open(file).unwrap();
    let p: Vec<Person> = serde_json::from_reader(file).unwrap();
    p
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