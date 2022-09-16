mod http;
mod server;
use http::{Method, Request};
use server::*;

fn main() {
    let server = Server::new("127.0.0.1:8080");
    let _method = Method::GET;
    server.run()
}
