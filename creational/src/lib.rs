pub mod factory;
pub mod abstract_factory;
pub mod static_factory;
pub mod lazy; 
pub mod singleton;
pub mod builder;
pub mod prototype;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
