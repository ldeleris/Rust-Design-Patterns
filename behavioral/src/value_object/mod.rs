//! `value_object` module.
//!
//! # Example
//! 
//! ```rust
//! use behavioral::value_object::*;
//! 
//! # let third_of_march = Date::new(3, String::from("MARCH"), 2016);
//! # let four_of_july = Date::new(4, String::from("JULY"), 2016);
//! # let new_year1 = Date::new(31, String::from("DECEMBER"), 2015);
//! # let new_year2 = Date::new(31, String::from("DECEMBER"), 2015);
//! 
//! # assert_eq!(false, third_of_march == four_of_july);
//! # assert_eq!(true, new_year1 == new_year2);
//! 

#[derive(Debug, PartialEq, Hash)]
pub struct Date {
  day: i32,
  month: String,
  year: i32,
}

impl Date {
  pub fn new(day: i32, month: String, year: i32) -> Date {
    Date { day, month, year }
  }
}

