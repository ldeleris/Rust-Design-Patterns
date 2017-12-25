//! `composite` module.
//!
//! # Example
//! 
//! ``` rust
//! use structural::composite::*;
//! 
//! let mut tree = Tree::new();
//! tree.add(Box::new(Leaf::new(String::from("leaf 1"))));
//! 
//! let mut subtree1 = Tree::new();
//! subtree1.add(Box::new(Leaf::new(String::from("leaf 2"))));
//! 
//! let mut subtree2 = Tree::new();
//! subtree2.add(Box::new(Leaf::new(String::from("leaf 3"))));
//! subtree2.add(Box::new(Leaf::new(String::from("leaf 4"))));
//! subtree1.add(Box::new(subtree2));
//! tree.add(Box::new(subtree1));
//! 
//! let mut subtree3 = Tree::new();
//! let mut subtree4 = Tree::new();
//! subtree4.add(Box::new(Leaf::new(String::from("leaf 5"))));
//! subtree4.add(Box::new(Leaf::new(String::from("leaf 6"))));
//! 
//! subtree3.add(Box::new(subtree4));
//! tree.add(Box::new(subtree3));
//! 
//! tree.print(String::from("-")); 
//! ```
 
pub trait Node {
  fn print(&self, prefix: String);
}

pub struct Leaf {
  data: String
}

impl Leaf {
  pub fn new(data: String) -> Leaf {
    Leaf { data }
  }
}

impl Node for Leaf {
  fn print(&self,  prefix: String) {
    println!("{}{}", prefix, self.data);
  }
}

pub struct Tree {
  children: Vec<Box<Node>>,
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      children: Vec::new(),
    }
  }
  pub fn add(&mut self, child: Box<Node>) {
    self.children.push(child);
  }
  pub fn remove(&mut self) {
    self.children.pop();
  }
}

impl Node for Tree {
  fn print(&self, prefix: String) {
    println!("{}(", prefix);
    for c in self.children.iter() {
      c.print(format!("{}{}", prefix, prefix));
    }
    println!("{})", prefix);
  }
}
