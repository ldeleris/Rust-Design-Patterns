pub mod standard {
  #[derive(Debug)]
  pub struct Person {
    first_name: String,
    last_name: String,
    age: u32,
  }

  pub struct PersonBuilder {
    first_name: String,
    last_name: String,
    age: u32,
  }

  impl PersonBuilder {
    pub fn new() -> PersonBuilder {
      PersonBuilder {
        first_name: String::new(),
        last_name: String::new(),
        age: 0,
      }
    }
    pub fn set_first_name(&self, first_name: String) -> PersonBuilder {
      PersonBuilder {
        first_name,
        last_name: self.last_name.clone(),
        age: self.age.clone(),
      }
    }
    pub fn set_last_name(&self, last_name: String) -> PersonBuilder {
      PersonBuilder {
        first_name: self.first_name.clone(),
        last_name,
        age: self.age.clone(),
      }
    }
    pub fn set_age(&self, age: u32) -> PersonBuilder {
      PersonBuilder {
        first_name: self.first_name.clone(),
        last_name: self.last_name.clone(),
        age,
      }
    }
    pub fn build(&self) -> Person {
      Person {
        first_name: self.first_name.clone(),
        last_name: self.last_name.clone(),
        age: self.age.clone(),
      }
    }
  }
}

pub mod type_safe {
  #[derive(Debug)]
  pub struct Person {
    first_name: String,
    last_name: String,
    age: u32,
  }
  
  pub struct PersonBuilder;

  pub struct FirstName;
  pub struct LastName {
    first_name: String,
  }
  pub struct Age {
    first_name: String,
    last_name: String,
  }
  
  impl PersonBuilder {
    pub fn new() -> FirstName {
      FirstName
    } 
  }

  impl FirstName {
    pub fn set_first_name(&self, first_name: String) -> LastName {
      LastName {
        first_name,
      }
    }
  }

  impl LastName {
    pub fn set_last_name(&self, last_name: String) -> Age {
      Age {
        first_name: self.first_name.clone(),
        last_name,
      }
    }
  }

  impl Age {
    pub fn set_age(&self, age: u32) -> Person {
      Person {
        first_name: self.first_name.clone(),
        last_name: self.last_name.clone(),
        age,
      }
    }
  }
}