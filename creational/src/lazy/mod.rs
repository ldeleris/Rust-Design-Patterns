//! `lazy` module.
//!
//! # Example
//! 
//! ```rust,ignore
//! 
//! use creational::lazy::*;
//! 
//! let mut circle = Circle::new();
//! println!("The basic area for a circle with radius 2.5 is {}", circle.area(2.5, false));
//! println!("The precise area for a circle with radius 2.5 is {}", circle.area(2.5, true));
//! println!("The basic area for a circle with radius 6.78 is {}", circle.area(6.78, false));
//! println!("The precise area for a circle with radius 6.78 is {}", circle.area(6.78, true));
//! ```

use std::io;
use std::io::Read;
use std::fs::File;

pub struct Circle {
  basic_pi: f64,
  is_loaded: bool,
  precise_pi_val: f64,
}

impl Circle {
  pub fn new() -> Circle {
    Circle {
      basic_pi: 3.14,
      is_loaded: false,
      precise_pi_val: 0.0,       
    }
  }
  pub fn precise_pi(&mut self) -> f64 {
    if self.is_loaded {
      self.precise_pi_val
    } else {
      self.precise_pi_val = read_pi_file().unwrap();
      self.is_loaded = true;
      self.precise_pi_val
    }
  }

  pub fn area(&mut self, radius: f64, precise: bool) -> f64 {
    let pi = if precise {
      self.precise_pi()
    } else {
      self.basic_pi
    };
    pi * radius * radius
  }
}

fn read_pi_file() -> Result<f64, io::Error> {
  let mut s = String::new();
  File::open("./ressources/pi.txt")?.read_to_string(&mut s)?;
  println!("Precise pi loaded: {}", s);
  let pi: f64 = s.parse().unwrap();
  Ok(pi)
}