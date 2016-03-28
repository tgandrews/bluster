use super::schema::posts;

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[insertable_into(posts)]
#[derive(RustcDecodable)]
pub struct NewPost {
    title: String,
    body: String
}
