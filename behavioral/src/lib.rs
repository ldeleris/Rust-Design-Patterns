//! This is documentation for the `behavioral` crate.
//!


#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate csv;
extern crate rustc_serialize;

pub mod value_object;
pub mod null_object;
pub mod strategy;
pub mod command; 
pub mod chain_of_responsability;
pub mod interpreter;

pub fn value_object() {
    use value_object::*;

    println!("------------");
    println!("value object");
    println!("------------");

    let third_of_march = Date::new(3, String::from("MARCH"), 2016);
    let four_of_july = Date::new(4, String::from("JULY"), 2016);
    let new_year1 = Date::new(31, String::from("DECEMBER"), 2015);
    let new_year2 = Date::new(31, String::from("DECEMBER"), 2015);

    println!("The 3rd of March 2016 is the same as the 4th of July 2016: {}",
        third_of_march == four_of_july);
    println!("The new year of 2105 is here twice: {}", new_year1 == new_year2);
}

pub fn null_object() {
    use null_object::*;
    use std::{thread, time};

    println!("-----------");
    println!("null object");
    println!("-----------");

    let mut data_generator = DataGenerator::new();
    data_generator.run();
    for _ in 0..10 {
        thread::sleep(time::Duration::from_millis(10));
        println!("Message: {:?}", data_generator.get_message());
    };
}
pub fn strategy() {
    use strategy::*;

    println!("--------");
    println!("strategy");
    println!("--------");

    let csv_people = ParserFactory::new("persons.csv");
    let json_people = ParserFactory::new("persons.json");
    let application_csv = PersonApplication::new(csv_people);
    let application_json = PersonApplication::new(json_people);
    
    println!("{}", application_csv.write("persons.csv"));
    println!("{}", application_json.write("persons.json"));
}
pub fn command() {
    use command::*;

    println!("-------");
    println!("command");
    println!("-------");

}
pub fn chain_of_responsability() {
    use chain_of_responsability::*;

    println!("-----------------------");
    println!("chain of responsability");
    println!("-----------------------");

}
pub fn interpreter() {
    use interpreter::*;

    println!("-----------");
    println!("interpreter");
    println!("-----------");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
