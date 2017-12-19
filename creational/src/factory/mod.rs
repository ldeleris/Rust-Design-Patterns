pub trait SimpleConnection {
    fn get_name(&self) -> String;
    fn execute_query(&self, query: &str);
}

pub struct SimpleMysqlConnection;
pub struct SimplePgsqlConnection;
impl SimpleConnection for SimpleMysqlConnection {
    fn get_name(&self) -> String {
        String::from("SimpleMysqlConnection")
    }
    fn execute_query(&self, query: &str) {
        println!("Executing the query {}, the MySql way.", query);
    }
}

impl SimpleConnection for SimplePgsqlConnection {
    fn get_name(&self) -> String {
        String::from("SimplePgsqlConnection")
    }
    fn execute_query(&self, query: &str) {
        println!("Executing the query {}, the PgSql way.", query);
    }
}

pub trait DataBaseClient {
    fn execute_query(&self, query: &str);
    fn connect(&self) -> Box<SimpleConnection>;
}
pub struct MySqlClient;
pub struct PgSqlClient;

impl DataBaseClient for MySqlClient {
    fn connect(&self) -> Box<SimpleConnection> {
        Box::new(SimpleMysqlConnection)
    }
    fn execute_query(&self, query: &str) {
        let connection = self.connect();
        connection.execute_query(query);
    }
}

impl DataBaseClient for PgSqlClient {
    fn connect(&self) -> Box<SimpleConnection> {
        Box::new(SimplePgsqlConnection)
    }
    fn execute_query(&self, query: &str) {
        let connection = self.connect();
        connection.execute_query(query);
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

