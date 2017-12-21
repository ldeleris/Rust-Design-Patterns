
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
