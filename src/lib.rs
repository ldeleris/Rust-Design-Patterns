extern crate creational;
extern crate structural;
extern crate behavioral;
extern crate functional;

pub fn creational() {
    use creational::*; 

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

    adapter();
}

pub fn behavioral() {
    use behavioral::*;
}

pub fn functional() {
    use functional::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
