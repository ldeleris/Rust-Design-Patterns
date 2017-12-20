extern crate design_patterns;

use design_patterns::*;

fn main() {
  println!("Design patterns");
  factory();
  abstract_factory();
  static_factory();
  lazy();
  singleton();
  builder();
  builder_type_safe();
  builder_optional();
}

