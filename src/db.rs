use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::collections::HashMap;

pub fn establish_connection(config: &HashMap<String, String>) -> PgConnection {
    let database_url = config.get("DATABASE_URL").unwrap();
    let conn = PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url));
    conn
}
