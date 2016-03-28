#[macro_use]
extern crate nickel;
extern crate api;
extern crate diesel;
extern crate rustc_serialize;

use nickel::{Nickel, HttpRouter, JsonBody};
use nickel::status::StatusCode;
use self::api::models::*;
use self::api::stores::*;
use rustc_serialize::json::{self};
use std::str::FromStr;

fn main() {
    let mut server = Nickel::new();

    server.post("/posts", middleware! { |req, res|
        let post = req.json_as::<Post>().unwrap();
        println!("Post {}", post.title);
    });

    server.get("/posts/:post_id", middleware! { |req, mut res|
        let post_id = i32::from_str(req.param("post_id").unwrap()).unwrap();
        println!("Looking for post id: {}", post_id);
        let post_store = PostStore::new();
        let post = post_store.get_by_id(post_id);
        match post {
            Option::Some(p) => json::encode(&p).unwrap(),
            Option::None => {
                res.set(StatusCode::NotFound);
                "Not found".to_string()
            }
        }
    });

    server.get("/posts", middleware! { |_, res|
        let post_store = PostStore::new();
        let results = post_store.get_all();

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
