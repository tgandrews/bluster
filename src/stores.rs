use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::Post;
use models::NewPost;
use schema::posts::dsl::*;
use db::establish_connection;
use std::collections::HashMap;

pub struct PostStore {
    connection: PgConnection
}

impl PostStore {
    pub fn new(config: &HashMap<String, String>) -> PostStore {
        PostStore{connection: establish_connection(config)}
    }

    pub fn get_all(&self) -> Option<Vec<Post>> {
        let result = posts.load::<Post>(&self.connection);
        match result {
            Result::Ok(p) => Some(p),
            Result::Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }

    pub fn get_by_id(&self, post_id: i32) -> Option<Post> {
        let result: Result<Post, diesel::result::Error> = posts.find(post_id).first(&self.connection);
        match result {
            Result::Ok(p) => Some(p),
            Result::Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }

    pub fn create(&self, new_post: &NewPost) -> Result<Post, String> {
        use schema::posts;

        let result = diesel::insert(new_post)
            .into(posts::table)
            .get_result(&self.connection);
        match result {
            Result::Ok(p) => Ok(p),
            Result::Err(_) => Err("Error inserting data".to_string())
        }
    }
}
