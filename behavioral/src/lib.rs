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
