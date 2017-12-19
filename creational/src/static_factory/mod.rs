use std;

pub trait Animal 
    where Self: std::fmt::Debug
{
    fn print(&self) {
        println!("{:?}", self);
    }
}
#[derive(Debug)]
struct Bird;
#[derive(Debug)]
struct Mammal;
#[derive(Debug)]
struct Fish;

impl Animal for Bird {}
impl Animal for Mammal {}
impl Animal for Fish {}

pub fn from_str(animal: &str) -> Box<Animal> {
    match animal {
        "Bird" => Box::new(Bird),
        "Mammal" => Box::new(Mammal),
        "Fish" => Box::new(Fish),
        s => panic!("Unknown animal: {}", s),
    }
}