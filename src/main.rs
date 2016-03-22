extern crate rustc_serialize;
#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, JsonBody};

#[derive(RustcDecodable, RustcEncodable)]
struct Post {
    id: i32,
    title: String,
    contents: String,
}

fn main() {
    let mut server = Nickel::new();

    server.post("/post", middleware! { |req, res|
        let post = req.json_as::<Post>().unwrap();
        println!("Post {}", post.title);
    });

    server.listen("127.0.0.1:6767");
}
