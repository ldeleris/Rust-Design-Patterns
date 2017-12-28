  //! `iterator` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::iterator::*;
  //! 
  //! let mut class_room = ClassRoom::new();
  //! class_room.add(Student::new("Ivan", 26));
  //! class_room.add(Student::new("Maria", 23));
  //! class_room.add(Student::new("John", 25));
  //! class_room.for_each(|s| println!("{:?}", s));
  //! ```

#[derive(Debug, Clone)]
pub struct Student {
  name: String,
  age: i32,
}
impl Student {
  pub fn new(name: &str, age: i32) -> Student {
    Student {
      name: String::from(name),
      age,
    }
  }
}

pub struct ClassRoom {
  students: Vec<Student>,
  count: usize,
}
impl ClassRoom {
  pub fn new() -> ClassRoom {
    ClassRoom {
      students: Vec::new(),
      count: 0,
    }
  }
  pub fn add(&mut self, student: Student) {
    self.students.push(student);
  }
}
impl Iterator for ClassRoom {
  type Item = Student;

  fn next(&mut self) -> Option<Self::Item> {
    self.count += 1;
    if self.count <= self.students.len() {
      Some(self.students[self.count - 1].clone())
    } else {
      None
    }
  }
}