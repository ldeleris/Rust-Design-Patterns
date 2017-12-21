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