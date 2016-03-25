#[macro_use]
extern crate nickel;
extern crate api;
extern crate diesel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter, JsonBody};
use self::api::*;
use self::api::models::*;
use self::diesel::prelude::*;
use rustc_serialize::json::{self};

fn main() {
    use api::schema::posts::dsl::*;

    let mut server = Nickel::new();

    server.post("/posts", middleware! { |req, res|
        let post = req.json_as::<Post>().unwrap();
        println!("Post {}", post.title);
    });

    server.get("/posts", middleware! { |req, res|
        let connection = establish_connection();
        println!("Connection made!");
        let results = posts
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");
        let json_response = results
            .iter()
            .fold(String::new(), |cur, post| {
                let post_encoded = json::encode(post).unwrap();
                let joiner = if cur.len() > 0 { "," } else { "" };
                format!("{}{}{}", cur, joiner, post_encoded)
            });
        format!("[{}]", json_response)
    });

    server.listen("127.0.0.1:6767");
}
