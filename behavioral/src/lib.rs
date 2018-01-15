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
pub mod iterator;
pub mod mediator;
pub mod memento;
pub mod observer;
pub mod state;
pub mod template_method;
pub mod visitor;

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

    let csv_people = ParserFactory::new("./ressources/persons.csv");
    let json_people = ParserFactory::new("./ressources/persons.json");
    let application_csv = PersonApplication::new(csv_people);
    let application_json = PersonApplication::new(json_people);
    
    println!("{}", application_csv.write("./ressources/persons.csv"));
    println!("{}", application_json.write("./ressources/persons.json"));
}
pub fn strategy_opt() {
    use strategy::opt::*;

    println!("--------");
    println!("strategy option");
    println!("--------");

    let csv_people = ParserFactory::new("./ressources/persons.csv");
    let json_people = ParserFactory::new("./ressources/persons.json");
    let application_csv = PersonApplication::new(csv_people);
    let application_json = PersonApplication::new(json_people);
    
    println!("{}", application_csv.write("./ressources/persons.csv"));
    println!("{}", application_json.write("./ressources/persons.json"));
}
pub fn strategy_closure() {
    use strategy::closure::*;

    println!("--------");
    println!("strategy closure");
    println!("--------");

    let files = vec!["./ressources/persons.csv", "./ressources/persons.json"];
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
pub fn interpreter_std() {
    use interpreter::std::*;

    println!("-----------");
    println!("interpreter");
    println!("-----------");

    let expr = "1 2 + 3 * 9 10 + -";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "1 2 3 4 5 * * - +";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "12 -";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
}
pub fn interpreter_closure() {
    use interpreter::closure::*;

    println!("-----------");
    println!("interpreter closure");
    println!("-----------");

    let expr = "1 2 + 3 * 9 10 + -";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "1 2 3 4 5 * * - +";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));

    let expr = "12 -";
    println!("{} = {:?}", expr.clone(), RPNInterpreter::interpret(RPNParser::parse(expr)));
}

pub fn iterator() {
    use iterator::*;

    println!("--------");
    println!("iterator");
    println!("--------");

    let mut class_room = ClassRoom::new();
    class_room.add(Student::new("Ivan", 26));
    class_room.add(Student::new("Maria", 23));
    class_room.add(Student::new("John", 25));
    class_room.for_each(|s| println!("{:?}", s));
}
pub fn mediator() {
    use mediator::std::*;

    println!("--------");
    println!("mediator");
    println!("--------");

    let mut school = School::new();
    let student1 = Student::new("Ivan", 26);
    let student2 = Student::new("Maria", 23);
    let student3 = Student::new("John", 25);

    let group1 = Group::new("Rust Design Patterns");
    let group2 = Group::new("Scala design patterns");
    let group3 = Group::new("Cloud computing");

    school.add_student_to_group(student1.clone(), group1.clone());
    school.add_student_to_group(student1.clone(), group2.clone());
    school.add_student_to_group(student1.clone(), group3.clone());

    school.add_student_to_group(student2.clone(), group1.clone());
    school.add_student_to_group(student2.clone(), group3.clone());

    school.add_student_to_group(student3.clone(), group1.clone());
    school.add_student_to_group(student3.clone(), group2.clone());

    school.notify_students_in_group(group1.clone(), "Design patterns in Rust are amazing!");

    println!("{:?} is in groups: {:?}", student3.clone(), school.get_groups_for_student(student3.clone()));

    school.remove_student_from_group(student3.clone(), group2.clone());
    println!("{:?} is in groups: {:?}", student3.clone(), school.get_groups_for_student(student3.clone()));
   
    println!("Students in {:?} are {:?}", group1.clone(), school.get_students_in_group((group1.clone())));
}
pub fn memento() {
    use memento::std::*;

    println!("-------");
    println!("memento");
    println!("-------");

    let mut text_editor_manipulator = TextEditorManipulator::new();
    text_editor_manipulator.append("This is a chapter about memento.");
    println!("The text is: {}", text_editor_manipulator.read_text());
    
    println!("Deleting 2 characters...");
    text_editor_manipulator.delete();
    text_editor_manipulator.delete();
    println!("The text is: {}", text_editor_manipulator.read_text());

    println!("Undoing...");
    text_editor_manipulator.undo();
    println!("The text is: {}", text_editor_manipulator.read_text());
    
    println!("Undoing...");
    text_editor_manipulator.undo();
    println!("The text is: {}", text_editor_manipulator.read_text());
        
}
pub fn observer() {
    use observer::std::*;

    println!("--------");
    println!("observer");
    println!("--------");

    let user_yvan = User::new("Ivan");
    let user_maria = User::new("Maria");
    let user_john = User::new("John");

    println!("Create a post...");
    let mut post = Post::new(user_yvan.clone(), "This is a post about the observer design pattern.");

    println!("Add a comment...");
    post.add_comment(Comment::new(user_yvan.clone(), "I hope you like the post!"));

    println!("Maria and John subscribe to the comments");
    post.add_observer(Box::new(user_john));
    post.add_observer(Box::new(user_maria.clone()));

    println!("Add a comment...");
    post.add_comment(Comment::new(user_yvan.clone(), "Why are you so quiet? Do you like it?"));

    println!("Add a comment...");
    post.add_comment(Comment::new(user_maria.clone(), "It is amazing! Thanks!"));
}
pub fn state() {
    use state::std::*;

    println!("-----");
    println!("state");
    println!("-----");

    let mut player = MediaPlayer::new();

    player.press_play_or_pause_button();
    player.press_play_or_pause_button();
    player.press_play_or_pause_button();
    player.press_play_or_pause_button();

    player.press_eject_insert_button();
    player.press_play_or_pause_button();
    player.press_eject_insert_button();
    player.press_play_or_pause_button();
    player.press_eject_insert_button();
}
pub fn template_method() {
    use template_method::std::*;

    println!("---------------");
    println!("template method");
    println!("---------------");

    let json_data_finder = JsonDataFinder;
    let csv_data_finder = CsvDataFinder;

    println!("Find a person with name Laurent: {:?}", json_data_finder.find(|x| x.into_iter().find(|p| p.name == Some(String::from("Laurent")))));
    println!("Find a person with name Pierre: {:?}", json_data_finder.find(|x| x.into_iter().find(|p| p.name == Some(String::from("Pierre")))));
    println!("Find a person with name Yvan: {:?}", json_data_finder.find(|x| x.into_iter().find(|p| p.name == Some(String::from("Yvan")))));

    println!("Find a person with name Laurent: {:?}", csv_data_finder.find(|x| x.into_iter().find(|p| p.name == Some(String::from("Laurent")))));
    println!("Find a person with name Pierre: {:?}", csv_data_finder.find(|x| x.into_iter().find(|p| p.name == Some(String::from("Pierre")))));
    println!("Find a person with name Yvan: {:?}", csv_data_finder.find(|x| x.into_iter().find(|p| p.name == Some(String::from("Yvan")))));
}

pub fn visitor() {
    use visitor::std::*;

    println!("-------");
    println!("visitor");
    println!("-------");

    let mut document = Document::new(
      vec!(
        Element::title("The Visitor Pattern Example."),
        Element::text("The visitor pattern helps us add extra functionality without changing the classes."),
        Element::hyperlink("Go check it online!", "https://www.google.com/"),
        Element::text("Thanks")
      )
    );

    let mut html_exporter = HtmlExporterVisitor::new();
    println!("Export to html:");
    document.accept(&mut html_exporter);
    println!("{}", html_exporter.get_html());     

    let mut plain_text_exporter = PlainTextExporterVisitor::new();
    println!("Export to plain text:");
    document.accept(&mut plain_text_exporter);
    println!("{}", plain_text_exporter.get_text()); 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
