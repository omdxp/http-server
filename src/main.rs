#![allow(dead_code)]

mod http;
mod server;
mod website_handler;
use server::*;
use website_handler::WebsiteHandler;

fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run(WebsiteHandler)
}
