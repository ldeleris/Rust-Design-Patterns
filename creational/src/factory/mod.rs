//! `factory` module.
//!
//! # Example
//! 
//! ``` rust
//! use factory::*;
//! 
//! let client_mysql: Box<DataBaseClient> = MySqlClient::new();
//! let client_pgsql: Box<DataBaseClient> = PgSqlClient::new();
//! let res = client_mysql.execute_query("SELECT * FROM users");
//! println!("{}", res);
//! let res = client_pgsql.execute_query("SELECT * FROM employees");
//! println!("{}", res); 
//! ```

/// Simple connection to database
/// 
pub trait SimpleConnection {
    fn get_name(&self) -> String;
    fn execute_query(&self, query: &str) -> String;
}

pub struct SimpleMysqlConnection;
pub struct SimplePgsqlConnection;
impl SimpleConnection for SimpleMysqlConnection {
    fn get_name(&self) -> String {
        String::from("SimpleMysqlConnection")
    }
    fn execute_query(&self, query: &str) -> String {
        format!("Executing the query {}, the MySql way.", query)
    }
}

impl SimpleConnection for SimplePgsqlConnection {
    fn get_name(&self) -> String {
        String::from("SimplePgsqlConnection")
    }
    fn execute_query(&self, query: &str) -> String {
        format!("Executing the query {}, the PgSql way.", query)
    }
}

pub trait DataBaseClient {
    fn execute_query(&self, query: &str) -> String ;
    fn connect(&self) -> Box<SimpleConnection>;
}
pub struct MySqlClient;
pub struct PgSqlClient;

impl DataBaseClient for MySqlClient {
    fn connect(&self) -> Box<SimpleConnection> {
        Box::new(SimpleMysqlConnection)
    }
    fn execute_query(&self, query: &str) -> String {
        let connection = self.connect();
        let res = connection.execute_query(query);
        res
    }
}

impl DataBaseClient for PgSqlClient {
    fn connect(&self) -> Box<SimpleConnection> {
        Box::new(SimplePgsqlConnection)
    }
    fn execute_query(&self, query: &str) -> String {
        let connection = self.connect();
        let res = connection.execute_query(query);
        res
    }
}

impl MySqlClient {
    pub fn new() -> Box<DataBaseClient> {
        Box::new(MySqlClient)
    }
}

impl PgSqlClient {
    pub fn new() -> Box<DataBaseClient> {
        Box::new(PgSqlClient)
    }
}

