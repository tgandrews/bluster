use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

pub fn get_config() -> HashMap<String, String> {
    dotenv().ok();
    let mut hm: HashMap<String, String> = HashMap::new();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    hm.insert("DATABASE_URL".to_owned(), database_url);

    let port = env::var("PORT")
        .expect("PORT must be set");
    hm.insert("PORT".to_owned(), port);

    hm
}
