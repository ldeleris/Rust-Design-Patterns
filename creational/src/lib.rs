pub mod factory {
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

}

pub mod abstract_factory {
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
   
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
