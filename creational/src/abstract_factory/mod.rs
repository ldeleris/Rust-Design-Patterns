//! `abstact_factory` module.
//!
//! # Example
//! 
//! ``` rust
//! use abstract_factory::*;
//! 
//! let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
//! let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());
//! let res = client_mysql.execute_query("SELECT * FROM users");
//! println!("{}", res);
//! let res = client_pgsql.execute_query("SELECT * FROM employees");  
//! println!("{}", res);  
//! ```

/// Trait for abstract different connector factory.
/// 
pub trait DataBaseConnectorFactory {
    fn connect(&self) -> Box<::factory::SimpleConnection>;
} 
/// Factory for MySql connection
/// 
pub struct MySqlFactory;
/// Factory for PostgreSQL
/// 
pub struct PgSqlFactory;

impl MySqlFactory {
    /// Constructs a new `DataBaseConnectorFactory` for MySql.
    ///
    /// # Examples
    ///
    /// ```
    /// use creational::abstract_factory::MySqlFactory;
    ///
    /// let mySql_connector_factory = MySqlFactory::new();
    /// ```
    pub fn new() -> Box<DataBaseConnectorFactory> {
        Box::new(MySqlFactory)
    }
}

impl PgSqlFactory {
    /// Constructs a new `DataBaseConnectorFactory for PostgreSQL`.
    ///
    /// # Examples
    ///
    /// ```
    /// use creational::abstract_factory::PgSqlFactory;
    ///
    /// let pgSql_connector_factory = PgSqlFactory::new();
    /// ```
    pub fn new() -> Box<DataBaseConnectorFactory> {
        Box::new(PgSqlFactory)
    }
}

impl DataBaseConnectorFactory for MySqlFactory {
    /// Constructs a new `SimpleConnection` for MySql.
    ///
    /// # Examples
    ///
    /// ```
    /// use creational::abstract_factory::MySqlFactory;
    ///
    /// let mySql_connector_factory = MySqlFactory::new();
    /// 
    /// let mySql_connection = mySql_connector_factory.connect();
    /// ```
    fn connect(&self) -> Box<::factory::SimpleConnection> {
        Box::new(::factory::SimpleMysqlConnection)
    }
}

impl DataBaseConnectorFactory for PgSqlFactory {
    /// Constructs a new `SimpleConnection` for PostgreSQL.
    ///
    /// # Examples
    ///
    /// ```
    /// use creational::abstract_factory::PgSqlFactory;
    /// 
    /// let pgSql_connector_factory = PgSqlFactory::new();
    /// 
    /// let pgSql_connection = pgSql_connector_factory.connect();
    /// ```
    fn connect(&self) -> Box<::factory::SimpleConnection> {
        Box::new(::factory::SimplePgsqlConnection)
    }
}

/// Database client factory
/// 
pub struct DataBaseClientFactory {
    connector_factory: Box<DataBaseConnectorFactory>,
}

impl DataBaseClientFactory {
    /// Constructs a new `DataBaseClientFactory` with the correct
    /// abstract factory as argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use creational::abstract_factory::MySqlFactory;
    /// use creational::abstract_factory::PgSqlFactory;
    /// use creational::abstract_factory::DataBaseClientFactory;
    /// 
    /// let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
    /// let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());
    /// ```
    pub fn new(connector_factory: Box<DataBaseConnectorFactory>) -> DataBaseClientFactory {
        DataBaseClientFactory {
            connector_factory
        }
    }
    /// Execute a query from the correct database
    ///
    /// # Examples
    ///
    /// ```
    /// use creational::abstract_factory::MySqlFactory;
    /// use creational::abstract_factory::PgSqlFactory;
    /// use creational::abstract_factory::DataBaseClientFactory;
    /// 
    /// let client_mysql: DataBaseClientFactory = DataBaseClientFactory::new(MySqlFactory::new());
    /// let client_pgsql: DataBaseClientFactory = DataBaseClientFactory::new(PgSqlFactory::new());
    /// 
    /// let res = client_mysql.execute_query("SELECT * FROM users");
    /// assert_eq!(String::from("Executing the query SELECT * FROM users, the MySql way."), res);
    /// 
    /// let res = client_pgsql.execute_query("SELECT * FROM employees");  
    /// assert_eq!(String::from("Executing the query SELECT * FROM employees, the PgSql way."), res);
    /// ```
    pub fn execute_query(&self, query: &str) -> String {
        let connection = self.connector_factory.connect();
        connection.execute_query(query)
    }
}
   