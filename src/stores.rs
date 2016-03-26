use diesel::prelude::*;
use models::Post;
use schema::posts::dsl::*;
use db::establish_connection;

pub struct PostStore {
}

impl PostStore {
    pub fn get_all() -> Option<Vec<Post>> {
        let connection = establish_connection();
        let result = posts.load::<Post>(&connection);
        match result {
            Result::Ok(p) => Some(p),
            Result::Err(err) => None
        }
    }
}
