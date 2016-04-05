use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

pub fn get_config() -> HashMap<String, String> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert("DATABASE_URL".to_owned(), database_url);

    hm
}
