#![allow(unused)]

// File-based modules: Rust maps `mod foo;` to either
//   src/foo.rs   OR   src/foo/mod.rs
// The module tree mirrors the directory tree.

mod config;
mod routes;
mod models;

fn main() {
    config::print_config();
    routes::health_route::print_health_route();
    routes::user_route::print_user_route();
    println!("main");
}
