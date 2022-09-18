#![allow(dead_code)]

mod http;
mod server;
mod website_handler;
use server::*;
use std::env;
use website_handler::WebsiteHandler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    let server = Server::new("127.0.0.1:8080");
    server.run(WebsiteHandler::new(public_path))
}
