//! This is documentation for the `creational` crate.
//!

pub mod factory;
pub mod abstract_factory;
pub mod static_factory;
pub mod lazy; 
pub mod singleton;
pub mod builder;
pub mod prototype;

/// Factory design patterns example.
///
pub fn factory() {
    use factory::*;
    println!("factory");
    let client_mysql: Box<DataBaseClient> = MySqlClient::new();
    let client_pgsql: Box<DataBaseClient> = PgSqlClient::new();
    let res = client_mysql.execute_query("SELECT * FROM users");
    println!("{}", res);
    let res = client_pgsql.execute_query("SELECT * FROM employees");
    println!("{}", res);
}

/// Abstract factory design patterns example.
///
pub fn abstract_factory() {
    use abstract_factory::*;
    println!("abstract factory");
    let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
    let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());
    let res = client_mysql.execute_query("SELECT * FROM users");
    println!("{}", res);
    let res = client_pgsql.execute_query("SELECT * FROM employees");  
    println!("{}", res);  
}

/// Static factory design patterns example.
///
pub fn static_factory() {
    use static_factory::*;
    println!("static factory");
    let animal: Box<Animal> = from_str("Bird");
    println!("Animal: {}", animal.print());
}

/// Lazy design patterns example.
///
pub fn lazy() {
    use lazy::*;
    println!("lazy");
    let mut circle = Circle::new();
    println!("The basic area for a circle with radius 2.5 is {}", circle.area(2.5, false));
    println!("The precise area for a circle with radius 2.5 is {}", circle.area(2.5, true));
    println!("The basic area for a circle with radius 6.78 is {}", circle.area(6.78, false));
    println!("The precise area for a circle with radius 6.78 is {}", circle.area(6.78, true));
}

/// Singleton design patterns example.
///
pub fn singleton() {
    use singleton::*;
    use std::{thread, time};

    println!("singleton");
    println!("{}", AppRegistry::print());
    println!("Sleeping for 5 seconds.");
    thread::sleep(time::Duration::from_secs(1));
    println!("I woke up.");
    AppRegistry::add_user("1", "Laurent");
    println!("{}", AppRegistry::print());
    AppRegistry::add_user("2", "Pierre");
    AppRegistry::add_user("3", "Angel");
    println!("Is user with ID=1 registred?: {}", AppRegistry::is_user_registred("1"));
    println!("Removing ID=2");
    AppRegistry::remove_user("2");
    println!("Is user with ID=2 registred?: {}", AppRegistry::is_user_registred("2"));
    println!("All users registred are: {:?}", AppRegistry::get_all_user_names());
}

/// Builder design patterns example.
///
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

/// Builder (with type safe) design patterns example.
///
pub fn builder_type_safe() {
    use builder::type_safe::*;

    println!("builder type safe");
    let person = PersonBuilder::new()
        .set_first_name(String::from("Laurent"))
        .set_last_name(String::from("Deleris"))
        .set_age(50);
 
    println!("{:?}", person);
}

/// Builder (with optional) design patterns example.
///
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

/// Prototype design patterns example.
///
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
    fn do_queries_on_database_with_factory() {
        use factory::MySqlClient;
        use factory::PgSqlClient;
        use factory::DataBaseClient;

        let client_mysql: Box<DataBaseClient> = MySqlClient::new();
        let client_pgsql: Box<DataBaseClient> = PgSqlClient::new();

        let res = client_mysql.execute_query("SELECT * FROM users");
        assert_eq!(String::from("Executing the query SELECT * FROM users, the MySql way."), res);

        let res = client_pgsql.execute_query("SELECT * FROM employees"); 
        assert_eq!(String::from("Executing the query SELECT * FROM employees, the PgSql way."), res);
    }

    #[test]
    fn do_queries_on_database_with_abstract_factory() {
        use abstract_factory::MySqlFactory;
        use abstract_factory::PgSqlFactory;
        use abstract_factory::DataBaseClientFactory;

        let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
        let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());

        let res = client_mysql.execute_query("SELECT * FROM users");
        assert_eq!(String::from("Executing the query SELECT * FROM users, the MySql way."), res);

        let res = client_pgsql.execute_query("SELECT * FROM employees");  
        assert_eq!(String::from("Executing the query SELECT * FROM employees, the PgSql way."), res);
    }

    #[test]
    fn you_can_create_new_animal() {
        use static_factory::*;

        let animal: Box<Animal> = from_str("Bird");
        assert_eq!(String::from("Bird"), animal.print());
    }

    #[test]
    #[ignore]
    fn you_can_memoize_the_resource_value_of_pi() {
        use lazy::*;

        let mut circle = Circle::new();
        assert_eq!(19.625, circle.area(2.5, false));
        assert_eq!(19.634954084936208, circle.area(2.5, true));
        assert_eq!(144.340776, circle.area(6.78, false));
        assert_eq!(144.41398773727704, circle.area(6.78, true));
    }

    #[test]
    #[ignore]
    fn you_should_create_an_app_registry_when_you_add_user() {
        use singleton::*;

        let registred = AppRegistry::print();
        assert_eq!(String::from("Application registry not initialized"), registred);
        
        AppRegistry::add_user("1", "Laurent");

        let registred = AppRegistry::print();
        assert_eq!(String::from("Application registry initialized"), registred);           
    }

    #[test]
    fn you_can_add_and_remove_user() {
        use singleton::*;
        
        AppRegistry::add_user("1", "Laurent");

        assert_eq!(true, AppRegistry::is_user_registred("1"));  

        AppRegistry::remove_user("1");      

        assert_eq!(false, AppRegistry::is_user_registred("1"));       
    }

    #[test]
    fn you_can_retrieve_all_user_names() {
        use singleton::*;
        
        AppRegistry::add_user("1", "Laurent");
        AppRegistry::add_user("2", "Pierre");
        AppRegistry::add_user("3", "Angel");

        let expected = vec!(
            "Angel",
            "Laurent",
            "Pierre"
        );

        for v in AppRegistry::get_all_user_names().iter() {
            assert!(expected.contains(v))
        }   
    }

    #[test]
    fn it_should_create_a_person_with_a_builder() {
        use builder::standard::*;

        let person1 = PersonBuilder::new()
            .set_first_name(String::from("Laurent"))
            .set_last_name(String::from("Deleris"))
            .set_age(50)
            .build();

        let person2 = PersonBuilder::new()
            .set_first_name(String::from("Laurent"))
            .set_last_name(String::from("Deleris"))
            .set_age(50)
            .build();

        assert_eq!(person1, person2);        
    }

    #[test]
    fn it_should_create_a_person_with_a_builder_type_safe() {
        use builder::type_safe::*;

        let person1 = PersonBuilder::new()
            .set_first_name(String::from("Laurent"))
            .set_last_name(String::from("Deleris"))
            .set_age(50);

        let person2 = PersonBuilder::new()
            .set_first_name(String::from("Laurent"))
            .set_last_name(String::from("Deleris"))
            .set_age(50);

        assert_eq!(person1, person2);        
    }

    #[test]
    fn it_should_create_a_person_with_a_builder_optional() {
        use builder::optional::*;

        let person1 = Person::new()
            .set_first_name(String::from("Laurent"))
            .set_last_name(String::from("Deleris"))
            .set_age(50);

        let person2 = Person::new()
            .set_first_name(String::from("Laurent"))
            .set_last_name(String::from("Deleris"))
            .set_age(50);
 
        assert_eq!(person1, person2);

        let person1 = Person::new_with_check();
        let person1 = Person::set_with_check_first(&person1, String::from("Laurent"));
        let person1 = Person::set_with_check_last(&person1, String::from("Deleris"));
        let person1 = Person::set_with_check_age(&person1, 50);

        let person2 = Person::new_with_check();
        let person2 = Person::set_with_check_first(&person2, String::from("Laurent"));
        let person2 = Person::set_with_check_last(&person2, String::from("Deleris"));
        let person2 = Person::set_with_check_age(&person2, 50);

        assert_eq!(person1.unwrap(), person2.unwrap());

        let person = Person::new_with_check();
        let person = Person::set_with_check_first(&person, String::from("12345678910"));
        let person = Person::set_with_check_last(&person, String::from("Deleris"));
        let person = Person::set_with_check_age(&person, 50);
        
        assert_eq!(Err(String::from("First name should be less than 20 characters.")), person);        
    }

    #[test]
    fn it_should_clone_instance_with_prototype() {
        use prototype::*;

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

        assert_eq!(initial_cell, copy1);
        assert_eq!(copy1, copy2);
        assert_eq!(initial_cell.proteins, copy3.proteins);
        assert_eq!(String::from("1234"), copy3.dna);        
    }
}
