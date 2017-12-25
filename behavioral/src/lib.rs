//! This is documentation for the `behavioral` crate.
//!

pub mod value_object;
pub mod null_object;
pub mod strategy;
pub mod command; 
pub mod chain_of_responsability;
pub mod interpreter;

pub fn value_object() {
    use value_object::*;

    println!("value object");

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

    println!("null object");
}
pub fn strategy() {
    use strategy::*;

    println!("strategy");
}
pub fn command() {
    use command::*;

    println!("command");
}
pub fn chain_of_responsability() {
    use chain_of_responsability::*;

    println!("chain of responsability");
}
pub fn interpreter() {
    use interpreter::*;

    println!("interpreter");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
