pub mod std {
  //! `std` module.
  //!
  //! # Example
  //! 
  //! ```rust
  //! use behavioral::mediator::std::*;
  //! 
  //! let mut school = School::new();
  //! let student1 = Student::new("Ivan", 26);
  //! let student2 = Student::new("Maria", 23);
  //! let student3 = Student::new("John", 25);
  //! let group1 = Group::new("Rust Design Patterns");
  //! let group2 = Group::new("Scala design patterns");
  //! let group3 = Group::new("Cloud computing");
  //! school.add_student_to_group(student1.clone(), group1.clone());
  //! school.add_student_to_group(student1.clone(), group2.clone());
  //! school.add_student_to_group(student1.clone(), group3.clone());
  //! school.add_student_to_group(student2.clone(), group1.clone());
  //! school.add_student_to_group(student2.clone(), group3.clone());
  //! school.add_student_to_group(student3.clone(), group1.clone());
  //! school.add_student_to_group(student3.clone(), group2.clone());
  //! school.notify_students_in_group(group1.clone(), "Design patterns in Rust are amazing!");
  //! println!("{:?} is in groups: {:?}", student3.clone(), school.get_groups_for_student(student3.clone()));
  //! school.remove_student_from_group(student3.clone(), group2.clone());
  //! println!("{:?} is in groups: {:?}", student3.clone(), school.get_groups_for_student(student3.clone()));
  //! 
  //! println!("Students in {:?} are {:?}", group1.clone(), school.get_students_in_group((group1.clone())));
  //! ```
  
  use std::collections::HashSet;
  use std::collections::HashMap;

  pub trait Notifiable {
    fn notify(&self, message: &str);
  }

  #[derive(Debug, PartialEq, Eq, Hash, Clone)]
  pub struct Student {
    name: String,
    age: i32,
  }
  impl Student {
    pub fn new(name: &str, age: i32) -> Student {
      Student { name: String::from(name), age }
    }
  }
  impl Notifiable for Student {
    fn notify(&self, message: &str) {
      println!("Student {} was notified with message: {}.", self.name, message);
    }
  }

  #[derive(Debug, PartialEq, Eq, Hash, Clone)]
  pub struct Group {
    name: String,
  }
  impl Group {
    pub fn new(name: &str) -> Group {
      Group { name: String::from(name) }
    }
  }

  pub trait Mediator {
    fn add_student_to_group(&mut self, student: Student, group: Group);
    fn is_student_in_group(&mut self, student: Student, group: Group) -> bool;
    fn remove_student_from_group(&mut self, student: Student, group: Group);
    fn get_students_in_group(&mut self, group: Group) -> Vec<Student>;
    fn get_groups_for_student(&mut self, student: Student) -> Vec<Group>;
    fn notify_students_in_group(&mut self, group: Group, message: &str);
  }

  #[derive(Debug, Clone)]
  pub struct School {
    students_to_groups: HashMap<Student, HashSet<Group>>,
    groups_to_students: HashMap<Group, HashSet<Student>>,
  }
  impl School {
    pub fn new() -> School {
      School {
        students_to_groups: HashMap::new(),
        groups_to_students: HashMap::new(),        
      }
    }
  }
  impl Mediator for School {
    fn add_student_to_group(&mut self, student: Student, group: Group) {
      let set = self.students_to_groups.entry(student.clone()).or_insert(HashSet::new());
      (*set).insert(group.clone());
      let set = self.groups_to_students.entry(group).or_insert(HashSet::new());
      (*set).insert(student);
    }
    fn is_student_in_group(&mut self, student: Student, group: Group) -> bool {
      let set1 = self.students_to_groups.entry(student.clone()).or_insert(HashSet::new());
      let set2 = self.groups_to_students.entry(group.clone()).or_insert(HashSet::new());
      (*set1).contains(&group) && (*set2).contains(&student)
    }
    fn remove_student_from_group(&mut self, student: Student, group: Group) {
      let set = self.students_to_groups.entry(student.clone()).or_insert(HashSet::new());
      (*set).remove(&group);
      let set = self.groups_to_students.entry(group).or_insert(HashSet::new());
      (*set).remove(&student);
    }
    fn get_students_in_group(&mut self, group: Group) -> Vec<Student> {
      let set = self.groups_to_students.entry(group).or_insert(HashSet::new());
      let mut v: Vec<Student> = Vec::new();
      for s in &(*set) {
        v.push((*s).clone());
      };
      v
    }
    fn get_groups_for_student(&mut self, student: Student) -> Vec<Group> {
      let set = self.students_to_groups.entry(student.clone()).or_insert(HashSet::new());
      let mut v: Vec<Group> = Vec::new();
      for g in &(*set) {
        v.push((*g).clone());
      };
      v
    }
    fn notify_students_in_group(&mut self, group: Group, message: &str) {
      let set = self.groups_to_students.entry(group).or_insert(HashSet::new());
      &(*set).iter().for_each(|s| s.notify(message));
    }    
  }
}