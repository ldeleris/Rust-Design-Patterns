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
pub fn strategy_std() {
    use strategy::std::*;

    println!("--------");
    println!("strategy standard");
    println!("--------");

    let csv_people = ParserFactory::new("persons.csv");
    let json_people = ParserFactory::new("persons.json");
    let application_csv = PersonApplication::new(csv_people);
    let application_json = PersonApplication::new(json_people);
    
    println!("{}", application_csv.write("persons.csv"));
    println!("{}", application_json.write("persons.json"));
}
pub fn strategy_opt() {
    use strategy::opt::*;

    println!("--------");
    println!("strategy option");
    println!("--------");

    let csv_people = ParserFactory::new("persons.csv");
    let json_people = ParserFactory::new("persons.json");
    let application_csv = PersonApplication::new(csv_people);
    let application_json = PersonApplication::new(json_people);
    
    println!("{}", application_csv.write("persons.csv"));
    println!("{}", application_json.write("persons.json"));
}
pub fn strategy_closure() {
    use strategy::closure::*;

    println!("--------");
    println!("strategy closure");
    println!("--------");

    let files = vec!["persons.csv", "persons.json"];
    for file in files {
        println!("Got the following data from {}: {:?}", file, Person::from_file(file));
    }
}
pub fn command() {
    use command::std::*;
    use std::rc::Rc;

    println!("-------");
    println!("command");
    println!("-------");

    let robot = Robot;
    let mut robot_controller = RobotController::new();

    println!("{}", robot_controller.issue_command(Rc::new(MakeSandwichCommand::new(robot.clone()))));
    println!("{}", robot_controller.issue_command(Rc::new(PourJuiceCommand::new(robot.clone()))));
    println!("I'm eating and having some juice.");
    println!("{}", robot_controller.issue_command(Rc::new(CleanupCommand::new(robot.clone()))));

    println!("Here is what I asked my robot to do:");
    robot_controller.show_history();
}

pub fn chain_of_responsability() {
    use chain_of_responsability::std::*;
    use chain_of_responsability::Money;

    println!("-----------------------");
    println!("chain of responsability");
    println!("-----------------------");

    let atm = ATM::new();
    println!("Request 135:");
    atm.request_money(Money::new(135));
    println!("Request 131:");
    atm.request_money(Money::new(131));
    println!("Request 5:");
    atm.request_money(Money::new(5));
    println!("Request 10:");
    atm.request_money(Money::new(10));
    println!("Request 20:");
    atm.request_money(Money::new(20));
    println!("Request 50:");
    atm.request_money(Money::new(50));

}

pub fn chain_of_responsability_closure() {
    use chain_of_responsability::closure::*;
    use chain_of_responsability::Money;

    println!("-----------------------");
    println!("chain of responsability closure");
    println!("-----------------------");

    println!("Request 135:");
    ATM::request_money(Money::new(135));
    println!("Request 131:");
    ATM::request_money(Money::new(131));
    println!("Request 5:");
    ATM::request_money(Money::new(5));
    println!("Request 10:");
    ATM::request_money(Money::new(10));
    println!("Request 20:");
    ATM::request_money(Money::new(20));
    println!("Request 50:");
    ATM::request_money(Money::new(50));


}
pub fn interpreter() {
    use interpreter::std::*;

    println!("-----------");
    println!("interpreter");
    println!("-----------");

    let e1 = Add::new(Box::new(Number::new(1)), Box::new(Number::new(2)));
    println!("1 + 2 = {}", e1.interpret());
    if let Some(e) = Factory::new("+", Factory::new("1", None, None), Factory::new("2", None, None)) {
        println!("1 + 2 = {}", e.interpret());
    }

    if let Some(e) = Factory::new("*", Factory::new("1", None, None), Factory::new("2", None, None)) {
        println!("{:?} = {}", e, e.interpret());
    }

    let expr = "1 2 +";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "1 2 + 3 * 9 10 + -";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "1 2 3 4 5 * * - +";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "12 -";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
