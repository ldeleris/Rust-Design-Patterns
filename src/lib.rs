extern crate colored;
extern crate creational;
extern crate structural;
extern crate behavioral;
extern crate functional;

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

pub fn behavioral() {
    //use behavioral::*;

    println!("==== Behavioral ====");
}

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
