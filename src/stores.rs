use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::Post;
use schema::posts::dsl::*;
use db::establish_connection;

pub struct PostStore {
    connection: PgConnection
}

impl PostStore {
    pub fn new() -> PostStore {
        PostStore{connection: establish_connection()}
    }

    pub fn get_all(&self) -> Option<Vec<Post>> {
        let result = posts.load::<Post>(&self.connection);
        match result {
            Result::Ok(p) => Some(p),
            Result::Err(_) => None
        }
    }
}
