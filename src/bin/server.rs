#[macro_use]
extern crate nickel;
extern crate api;
extern crate diesel;
extern crate rustc_serialize;
extern crate time;

use nickel::{Nickel, HttpRouter, JsonBody};
use nickel::status::StatusCode;
use self::api::models::*;
use self::api::stores::*;
use self::api::config::get_config;
use rustc_serialize::json::{self};
use std::str::FromStr;
use nickel::extensions::Redirect;
use time::PreciseTime;
use std::net::Ipv4Addr;

fn main() {
    let mut server = Nickel::new();
    let config = get_config();

    let c1 = config.clone();
    server.post("/posts", middleware! { |req, mut res|
        let start = PreciseTime::now();
        let post = req.json_as::<NewPost>().unwrap();
        let post_store = PostStore::new(&c1);
        match post_store.create(&post) {
            Result::Ok(post) => {
                let end = PreciseTime::now();
                println!("POST /posts {}ms", start.to(end).num_milliseconds());
                return res.redirect(format!("/posts/{}", post.id))
            },
            Result::Err(err) => {
                res.set(StatusCode::InternalServerError);
                err
            }
        }
    });

    let c2 = config.clone();
    server.get("/posts/:post_id", middleware! { |req, mut res|
        let start = PreciseTime::now();
        let post_id = i32::from_str(req.param("post_id").unwrap()).unwrap();
        let post_store = PostStore::new(&c2);
        let post = post_store.get_by_id(post_id);
        let end = PreciseTime::now();
        match post {
            Option::Some(p) => {
                println!("GET /posts/{} {}ms", post_id, start.to(end).num_milliseconds());
                json::encode(&p).unwrap()
            },
            Option::None => {
                res.set(StatusCode::NotFound);
                "Not found".to_string()
            }
        }
    });

    let c3 = config.clone();
    server.get("/posts", middleware! { |_, res|
        let start = PreciseTime::now();
        let post_store = PostStore::new(&c3);
        let results = post_store.get_all();

        let json_response = results
            .iter()
            .fold(String::new(), |cur, post| {
                let post_encoded = json::encode(post).unwrap();
                let joiner = if cur.len() > 0 { "," } else { "" };
                format!("{}{}{}", cur, joiner, post_encoded)
            });
        let end = PreciseTime::now();
        println!("GET /posts {}ms", start.to(end).num_milliseconds());
        format!("[{}]", json_response)
    });

    let server_port = u16::from_str(config.get("PORT").unwrap()).unwrap();
    let server_addr = Ipv4Addr::from_str("0.0.0.0").unwrap();

    server.listen((server_addr, server_port));
}
