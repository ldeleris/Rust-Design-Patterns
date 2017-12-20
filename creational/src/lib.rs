pub mod factory;
pub mod abstract_factory;
pub mod static_factory;
pub mod lazy; 
pub mod singleton;
pub mod builder;
pub mod prototype;

pub fn factory() {
    use factory::*;
    println!("factory");
    let client_mysql: Box<DataBaseClient> = MySqlClient::new();
    let client_pgsql: Box<DataBaseClient> = PgSqlClient::new();
    client_mysql.execute_query("SELECT * FROM users");
    client_pgsql.execute_query("SELECT * FROM employees");
}

pub fn abstract_factory() {
    use abstract_factory::*;
    println!("abstract factory");
    let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
    let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());
    client_mysql.execute_query("SELECT * FROM users");
    client_pgsql.execute_query("SELECT * FROM employees");    
}

pub fn static_factory() {
    use static_factory::*;
    println!("static factory");
    let animal: Box<Animal> = from_str("Bird");
    animal.print();
}

pub fn lazy() {
    use lazy::*;
    println!("lazy");
    let mut circle = Circle::new();
    println!("The basic area for a circle with radius 2.5 is {}", circle.area(2.5, false));
    println!("The precise area for a circle with radius 2.5 is {}", circle.area(2.5, true));
    println!("The basic area for a circle with radius 6.78 is {}", circle.area(6.78, false));
    println!("The precise area for a circle with radius 6.78 is {}", circle.area(6.78, true));
 }

pub fn singleton() {
    use singleton::*;
    use std::{thread, time};

    println!("singleton");
    AppRegistry::print();
    println!("Sleeping for 5 seconds.");
    thread::sleep(time::Duration::from_secs(1));
    println!("I woke up.");
    AppRegistry::add_user("1", "Laurent");
    AppRegistry::add_user("2", "Pierre");
    AppRegistry::add_user("3", "Angel");
    println!("Is user with ID=1 registred?: {}", AppRegistry::is_user_registred("1"));
    println!("Removing ID=2");
    AppRegistry::remove_user("2");
    println!("Is user with ID=2 registred?: {}", AppRegistry::is_user_registred("2"));
    println!("All users registred are: {:?}", AppRegistry::get_all_user_names());
}

pub fn builder() {
    use builder::standard::*;

    println!("builder");
    let person = PersonBuilder::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"))
        .set_age(50)
        .build();
    println!("{:?}", person);
}


pub fn builder_type_safe() {
    use builder::type_safe::*;

    println!("builder type safe");
    let person = PersonBuilder::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"))
        .set_age(50);
 
    println!("{:?}", person);
}

pub fn builder_optional() {
    use builder::optional::*;

    println!("builder optional");
    let person = Person::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"))
        .set_age(50);
 
    println!("{:?}", person);

    let person = Person::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"));
 
    println!("{:?}", person);

    let person = Person::new_with_check();
    let person = Person::set_with_check_first(&person, String::from("Laurent"));
    let person = Person::set_with_check_last(&person, String::from("Deleris"));
    let person = Person::set_with_check_age(&person, 50);
    println!("{:?}", person);

    let person = Person::new_with_check();
    let person = Person::set_with_check_first(&person, String::from("12345678910"));
    let person = Person::set_with_check_last(&person, String::from("Deleris"));
    let person = Person::set_with_check_age(&person, 50);
    println!("{:?}", person);
}

pub fn prototype() {
    use prototype::*;

    println!("prototype");

    let initial_cell = Cell {
        dna: String::from("abcd"),
        proteins: vec![String::from("protein1"), String::from("protein2")],
    };
    let copy1 = initial_cell.clone();
    let copy2 = initial_cell.clone();
    let copy3 = Cell {
        dna: String::from("1234"),
        ..initial_cell.clone()
    };
    println!("The prototype is: {:?}", initial_cell);
    println!("cell 1: {:?}", copy1);
    println!("cell 2: {:?}", copy2);
    println!("cell 3: {:?}", copy3);
    println!("1 and 2 are equal: {}", copy1 == copy2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
