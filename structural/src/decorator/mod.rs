//! `decorator` module.
//!
//! # Example
//! 
//! ``` rust
//! use decorator::desserts::*;
//! 
//! let dessert = Crepe::new();
//! let dessert = Chantilly::new(dessert);
//! println!("{}", dessert.to_string());
//! 
//! let dessert = Gauffre::new();
//! let dessert = Chocolat::new(dessert);
//! let dessert = Chantilly::new(dessert);
//! println!("{}", dessert.to_string());
//! 
//! let dessert = Chocolat::new(Chantilly::new(Crepe::new()));
//! println!("{}", dessert.to_string());
//! 
//! use decorator::input_readers::*;
//! 
//! let reader = AdvancedInputReader::new("input.txt");
//! let mut reader = CapitalizedInputReader::new(reader);
//! let res = reader.read_lines();
//! for r in res.iter() {
//!     println!("{}", r);
//! }
//! 
//! let reader = AdvancedInputReader::new("input.txt");
//! let reader = CapitalizedInputReader::new(reader);
//! let mut reader = LenghtInputReader::new(reader);
//! let res = reader.read_lines();
//! for r in res.iter() {
//!     println!("{}", r);
//! }
//! ```
 
pub mod desserts {
  //! `desserts` module.
  //! 

  pub trait Dessert {
    fn get_label(&self) -> String;
    fn get_price(&self) -> f32;
    fn to_string(&self) -> String {
      format!("{}: {}", self.get_label(), self.get_price())
    } 
  }

  struct ClassDessert {
    label: String,
    price: f32,
  }

  impl ClassDessert {
    fn new(label: String, price: f32) -> ClassDessert {
      ClassDessert {
        label,
        price,
      }
    }
  }

  impl Dessert for ClassDessert {
    fn get_label(&self) -> String { self.label.clone() }
    fn get_price(&self) -> f32 { self.price }
    fn to_string(&self) -> String {
      format!("{} = {}", self.label, self.price)
    }
  }

  pub struct Gauffre {
    dessert: ClassDessert,
  }
  impl Gauffre {
    pub fn new() -> Box<Dessert> {
      Box::new(Gauffre {
        dessert: ClassDessert::new(String::from("Gauffre"), 1.5),
        })
    }
  }

  impl Dessert for Gauffre {
    fn get_label(&self) -> String { self.dessert.get_label().clone() }
    fn get_price(&self) -> f32 { self.dessert.price }
    fn to_string(&self) -> String {
      self.dessert.to_string()
    }
  }

  pub struct Crepe {
    dessert: ClassDessert,
  }
  impl Crepe {
    pub fn new() -> Box<Dessert> {
      Box::new(Gauffre{
        dessert: ClassDessert::new(String::from("Crepe"), 1.0),
      })
    }
  }

  impl Dessert for Crepe {
    fn get_label(&self) -> String { self.dessert.get_label().clone() }
    fn get_price(&self) -> f32 { self.dessert.price }
    fn to_string(&self) -> String {
      self.dessert.to_string()
    }
  }

  pub struct Chantilly {
    dessert: Box<Dessert>,
  }

  impl Chantilly {
    pub fn new(dessert: Box<Dessert>) -> Box<Dessert> {
      Box::new(
        Chantilly {
          dessert,
        })
    }
  }

  impl Dessert for Chantilly {
    fn get_label(&self) -> String {
      format!("{} Chantilly", self.dessert.get_label().clone())
    }
    fn get_price(&self) -> f32 {
      self.dessert.get_price() + 0.3
    }
  }


  pub struct Chocolat {
    dessert: Box<Dessert>,
  }

  impl Chocolat {
    pub fn new(dessert: Box<Dessert>) -> Box<Dessert> {
      Box::new(
        Chocolat {
          dessert,
        })
    }
  }

  impl Dessert for Chocolat {
    fn get_label(&self) -> String {
      format!("{} Chocolat", self.dessert.get_label().clone())
    }
    fn get_price(&self) -> f32 {
      self.dessert.get_price() + 0.2
    }
  }
}

pub mod input_readers {
  //! `imput_readers` module.
  //! 

  use std::io::BufReader;
  use std::fs::File;
  use std::io::BufRead;

  pub trait InputReader {
    fn read_lines(&mut self) -> Vec<String>;
  }

  #[derive(Debug)]
  pub struct AdvancedInputReader {
    pub reader: BufReader<File>,
  }

  impl AdvancedInputReader {
    pub fn new(path: &str) -> Box<InputReader> {
      let f = File::open(path).unwrap();
      Box::new(AdvancedInputReader { reader: BufReader::new(f) })
    }
  }

  impl InputReader for AdvancedInputReader {
    fn read_lines(&mut self) -> Vec<String> {
      let mut v: Vec<String> = Vec::new();
      let mut line = String::new();
      while let Ok(num) = self.reader.read_line(&mut line)  {
        if num == 0 { break };
        v.push(String::from(line.clone().trim()));
        line = String::new();
      };
      v
    }
  }

  pub struct CapitalizedInputReader {
    input_reader: Box<InputReader>,
  }

  impl CapitalizedInputReader {
    pub fn new(input_reader: Box<InputReader>) -> Box<InputReader> {
      Box::new(CapitalizedInputReader { input_reader, })
    }
  }

  impl InputReader for CapitalizedInputReader {
    fn read_lines(&mut self) -> Vec<String> {
      let mut out: Vec<String> = Vec::new();
      let reader = self.input_reader.read_lines().clone();
      for r in reader {
        out.push(r.to_uppercase());
      }
      out
    }
  }

  pub struct LenghtInputReader {
    input_reader: Box<InputReader>,
  }

  impl LenghtInputReader {
    pub fn new(input_reader: Box<InputReader>) -> Box<InputReader> {
      Box::new(LenghtInputReader { input_reader, })
    }
  }

  impl InputReader for LenghtInputReader {
    fn read_lines(&mut self) -> Vec<String> {
      let mut out: Vec<String> = Vec::new();
      let reader = self.input_reader.read_lines().clone();
      for r in reader {
        out.push(format!("{}: {}",r, r.len()));
      }
      out
    }
  }
}
