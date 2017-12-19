pub trait DataBaseConnectorFactory {
    fn connect(&self) -> Box<::factory::SimpleConnection>;
} 

pub struct MySqlFactory;
pub struct PgSqlFactory;

impl MySqlFactory {
    pub fn new() -> Box<DataBaseConnectorFactory> {
        Box::new(MySqlFactory)
    }
}

impl PgSqlFactory {
    pub fn new() -> Box<DataBaseConnectorFactory> {
        Box::new(PgSqlFactory)
    }
}

impl DataBaseConnectorFactory for MySqlFactory {
    fn connect(&self) -> Box<::factory::SimpleConnection> {
        Box::new(::factory::SimpleMysqlConnection)
    }
}

impl DataBaseConnectorFactory for PgSqlFactory {
    fn connect(&self) -> Box<::factory::SimpleConnection> {
        Box::new(::factory::SimplePgsqlConnection)
    }
}

pub struct DataBaseClientFactory {
    connector_factory: Box<DataBaseConnectorFactory>,
}

impl DataBaseClientFactory {
    pub fn new(connector_factory: Box<DataBaseConnectorFactory>) -> DataBaseClientFactory {
        DataBaseClientFactory {
            connector_factory
        }
    }
    pub fn execute_query(&self, query: &str) {
        let connection = self.connector_factory.connect();
        connection.execute_query(query);
    }
}
   