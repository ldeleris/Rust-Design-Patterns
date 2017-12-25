//! `prototype` module.
//!
//! # Example
//! 
//! ``` rust
//! use prototype::*;
//! 
//! let initial_cell = Cell {
//!     dna: String::from("abcd"),
//!     proteins: vec![String::from("protein1"), String::from("protein2")],
//! };
//! let copy1 = initial_cell.clone();
//! let copy2 = initial_cell.clone();
//! let copy3 = Cell {
//!     dna: String::from("1234"),
//!     ..initial_cell.clone()
//! };
//! println!("The prototype is: {:?}", initial_cell);
//! println!("cell 1: {:?}", copy1);
//! println!("cell 2: {:?}", copy2);
//! println!("cell 3: {:?}", copy3);
//! println!("1 and 2 are equal: {}", copy1 == copy2);
//! ```

#[derive(Clone, PartialEq, Debug)]
pub struct Cell {
  pub dna: String,
  pub proteins: Vec<String>,
}

