extern crate creational;
extern crate structural;
extern crate behavioral;
extern crate functional;

use creational::factory::*;

pub fn factory() {
    let client_mysql: Box<DataBaseClient> = MySqlClient::new();
    let client_pgsql: Box<DataBaseClient> = PgSqlClient::new();
    client_mysql.execute_query("SELECT * FROM users");
    client_pgsql.execute_query("SELECT * FROM employees");
   
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
