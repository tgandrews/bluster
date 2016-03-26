#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

pub mod schema;
pub mod models;
pub mod stores;
mod db;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rustc_serialize;
