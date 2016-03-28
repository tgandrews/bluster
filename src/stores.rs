use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::Post;
use models::NewPost;
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

    // pub fn insert(&self, new_post: &NewPost) -> Result<Post> {
    //     let result = diesel::insert(&new_post)
    //         .into(posts::table)
    //         .get_result(&self.connection);
    //
    // }
}
