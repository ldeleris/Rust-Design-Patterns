//! `facade` module.
//!

  
#[derive(Debug)]
pub struct Person {
  name: Option<String>,
  age: Option<i32>,
}

pub mod with_struct {
  //! `with_struct` module.
  //! 
  //! # Example
  //! 
  //! ``` rust
  //!  use facade::with_derive_trait::*;
  //! 
  //!  let url = String::from("google.com");
  //!  let reader = DataReader;
  //!  println!("{:?}", reader.read_person(url));
  //! ```

  use facade::Person;

  struct DataDownloader;
  impl DataDownloader {
    fn download(&self, url: String) -> String {
      println!("Downloading from: {}", url);
      String::from("ew0KICAgICJuYW1lIjogIkl2YW4iLA0KICAgICJhZ2UiOiAyNg0KfQ==")
    } 
  }

  struct DataDecoder;
  impl DataDecoder {
    fn decode(&self, data: String) -> String {
      println!("Decoding from: {}", data);
      String::from("{\n\t\"name\": \"Laurent Deleris\",\n\t\"age\": 50\n}")
    }
  }

  struct DataDeserializer;
  impl DataDeserializer {
    fn parse(&self, data: String) -> Option<Person> {
      println!("Parsing from: {}", data);
      Some(
          Person {
          name: Some(String::from("Laurent Deleris")),
          age: Some(50),
          }
      )            
    }
  }

  pub struct DataReader;

  impl DataReader {
    pub fn read_person(&self, url: String) -> Option<Person> {
      let data = DataDownloader.download(url);
      let json = DataDecoder.decode(data);
      let person = DataDeserializer.parse(json);
      person
    }
  }
}

pub mod with_derive_trait {
  //! `with_derive_trait` module.
  //! 
  //! # Example
  //! 
  //! ``` rust
  //! use facade::with_struct::*;
  //! 
  //! let url = String::from("google.com");
  //! let reader = DataReader;
  //! println!("{:?}", reader.read_person(url));
  //! ```

  use facade::Person;

  trait DataDownloader {
      fn download(&self, url: String) -> String;
  }

  trait DataDecoder {
      fn decode(&self, data: String) -> String;
  }

  trait DataDeserializer {
    fn parse(&self, data: String) -> Option<Person>;
  }

  #[derive(DataDownloader, DataDecoder, DataDeserializer)]
  pub struct DataReader;

  impl DataReader {
    pub fn read_person(&self, url: String) -> Option<Person> {
      let data = self.download(url);
      let json = self.decode(data);
      let person = self.parse(json);
      person
    }
  }
}

