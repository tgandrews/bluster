#[macro_use]
extern crate nickel;
extern crate api;
extern crate diesel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter, JsonBody};
use self::api::*;
use self::api::models::*;
use self::diesel::prelude::*;
use rustc_serialize::json::{self, ToJson, Json};

fn main() {
    use api::schema::posts::dsl::*;

    // let mut server = Nickel::new();

    let connection = establish_connection();
    println!("Connection made!");

    // server.post("/post", middleware! { |req, res|
    //     let post = req.json_as::<Post>().unwrap();
    //     println!("Post {}", post.title);
    // });

    // server.get("/post", middleware! { |req, res|
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");
    let first_post = results.first().unwrap();

    println!("Posts {}", json::encode(first_post).unwrap());
    // });

    // server.listen("127.0.0.1:6767");
}
