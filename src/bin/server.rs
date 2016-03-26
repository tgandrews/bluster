#[macro_use]
extern crate nickel;
extern crate api;
extern crate diesel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter, JsonBody};
use self::api::models::*;
use self::api::stores::*;
use rustc_serialize::json::{self};

fn main() {
    let mut server = Nickel::new();

    server.post("/posts", middleware! { |req, res|
        let post = req.json_as::<Post>().unwrap();
        println!("Post {}", post.title);
    });

    server.get("/posts", middleware! { |req, res|
        let results = PostStore::get_all();

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
