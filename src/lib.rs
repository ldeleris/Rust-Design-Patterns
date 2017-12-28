//! This is documentation for the `design-patterns` crate.
//!
extern crate creational;
extern crate structural;
extern crate behavioral;
extern crate functional;

/// Creational design patterns examples.
///
pub fn creational() {
    use creational::*; 

    println!("==== Creational ====");
    factory();
    abstract_factory();
    static_factory();
    lazy();
    singleton();
    builder();
    builder_type_safe();
    builder_optional();
    prototype();
}

/// Structural design patterns examples.
///
pub fn structural() {
    use structural::*;

    println!("==== Structural ====");
    adapter();
    decorator();
    bridge();
    composite();
    facade();
    facade_with_derive_trait();
    flyweight();
    proxy();
}

/// Behavioral design patterns examples.
///
pub fn behavioral() {
    use behavioral::*;

    println!("==== Behavioral ====");
    value_object();
    null_object();
    strategy_std();
    strategy_opt();
    strategy_closure();
    command();
    chain_of_responsability();
    chain_of_responsability_closure();
    interpreter_std();
    interpreter_closure();
    iterator();
    mediator();
    memento();
    observer();
    state();
    template_method();
    visitor();
}

/// Functional design patterns examples.
///
pub fn functional() {
    //use functional::*;

    println!("==== Functional ====");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
