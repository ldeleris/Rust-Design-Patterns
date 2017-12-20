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

pub mod optional {
#[derive(Debug)]
  pub struct Person {
    first_name: Option<String>,
    last_name: Option<String>,
    age: Option<u32>,
  }

  impl Person {
    pub fn new() -> Person {
      Person {
        first_name: None,
        last_name: None,
        age: None,
      }
    }
    pub fn set_first_name(&self, first_name: String) -> Person {
      Person {
        first_name: Some(first_name),
        last_name: self.last_name.clone(),
        age: self.age.clone(),
      }
    }
    pub fn set_last_name(&self, last_name: String) -> Person {
      Person {
        first_name: self.first_name.clone(),
        last_name: Some(last_name),
        age: self.age.clone(),
      }
    }
    pub fn set_age(&self, age: u32) -> Person {
      Person {
        first_name: self.first_name.clone(),
        last_name: self.last_name.clone(),
        age: Some(age),
      }
    }

    pub fn new_with_check() -> Result<Person, String> {
      Ok(
        Person {
          first_name: None,
          last_name: None,
          age: None,
        }
      )
    }

    pub fn set_with_check_first(person: &Result<Person, String>, first_name: String) -> Result<Person, String> {
      match *person {
        Ok(ref p) => 
          if first_name.len() > 10 {
            Err(String::from("First name should be less than 20 characters."))
          } else {
            Ok(
              Person {
                first_name: Some(first_name),
                last_name: p.last_name.clone(),
                age: p.age.clone(),
              }
            )       
          },
        Err(ref s) => Err(s.clone()),
      }
    }
    pub fn set_with_check_last(person: &Result<Person, String>, last_name: String) -> Result<Person, String> {
      match *person {
        Ok(ref p) => 
          if last_name.len() > 10 {
            Err(String::from("Last name should be less than 20 characters."))
          } else {
            Ok(
              Person {
                first_name: p.first_name.clone(),
                last_name: Some(last_name),
                age: p.age.clone(),
              }
            )       
          },
        Err(ref s) => Err(s.clone()),
      }
    }
    pub fn set_with_check_age(person: &Result<Person, String>, age: u32) -> Result<Person, String> {
      match *person {
        Ok(ref p) => 
          if age == 0 {
            Err(String::from("Age should be more than zero!"))
          } else {
            Ok(
              Person {
                first_name: p.first_name.clone(),
                last_name: p.last_name.clone(),
                age: Some(age),
              }
            )       
          },
        Err(ref s) => Err(s.clone()),
      }
    }
  }
  

}