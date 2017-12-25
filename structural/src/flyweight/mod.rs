//! `flyweight` module.
//!
//! # Example
//! 
//! ``` rust
//! use structural::flyweight::*;
//! 
//! let mut graphic = Graphic::new();
//! let mut circles = CircleFactory::new();
//! graphic.add_circle(1, 1, 1.0, circles.make_circle(Color::Green));
//! graphic.add_circle(1, 2, 1.0, circles.make_circle(Color::Red));
//! graphic.add_circle(2, 1, 1.0, circles.make_circle(Color::Blue));
//! graphic.add_circle(2, 2, 1.0, circles.make_circle(Color::Green));
//! graphic.add_circle(2, 3, 1.0, circles.make_circle(Color::Yellow));
//! graphic.add_circle(3, 2, 1.0, circles.make_circle(Color::Magenta));
//! graphic.add_circle(3, 3, 1.0, circles.make_circle(Color::Blue));
//! graphic.add_circle(4, 3, 1.0, circles.make_circle(Color::Blue));
//! graphic.add_circle(3, 4, 1.0, circles.make_circle(Color::Yellow));
//! graphic.add_circle(4, 4, 1.0, circles.make_circle(Color::Red));
//! 
//! graphic.draw();
//! 
//! println!("Total number of circle objects created: {}", circles.circles_created());
//! ```
 
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Color {
  Red,
  Green,
  Blue,
  Yellow,
  Magenta,
}

#[derive(Debug, Clone)]
pub struct Circle {
  pub color:  Color,
}
impl Circle {
  fn new(color: Color) -> Circle {
    println!("Creating a circle with {:?} color.", color);
    Circle { color }
  }
}

pub struct CircleFactory {
  cache: HashMap<Color, Circle>,
}
impl CircleFactory {
  pub fn new() -> CircleFactory {
    CircleFactory {
      cache: HashMap::new(),
    }
  }
  pub fn make_circle(&mut self, color: Color) -> Circle {
    let circle = self.cache.entry(color.clone()).or_insert_with(|| Circle::new(color));
    circle.clone()
  }

  pub fn circles_created(&self) -> usize {
    self.cache.len()
  }
}

pub struct Graphic {
  pub items: Vec<(i32, i32, f32, Circle)>,
}
impl Graphic {
  pub fn new() -> Graphic {
    Graphic {
      items: Vec::new(),
    }
  }
  pub fn add_circle(&mut self, x: i32, y: i32, radius: f32, circle: Circle) {
    self.items.push((x, y, radius, circle));
  }
  pub fn draw(&self) {
    for i in self.items.iter() {
      println!("Drawing a circle : {:?}", i);
    }
  }
}

