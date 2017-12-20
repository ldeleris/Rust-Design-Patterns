pub mod adapter;
//pub mod decorator;
//pub mod bridge;
//pub mod composite;
//pub mod facade;
//pub mod flyweight;
//pub mod proxy;

pub fn adapter() {
    use adapter::*;

    println!("Adapter");
    let logger =  AppLogger::new();
    logger.log_info(String::from("This is an info message."));
    logger.log_debug(String::from("Debug something here."));
    logger.log_error(String::from("Show an error message."));
    logger.log_warning(String::from("About to finish."));
    logger.log_info(String::from("Bye!"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
