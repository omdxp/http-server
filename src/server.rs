use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.into() }
    }
    pub fn run(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr);
        match listener {
            Ok(lis) => {
                println!("server running at http://{}", self.addr);
                loop {
                    match lis.accept() {
                        Ok((mut tcp_stream, _)) => {
                            let mut buf = [0; 1024];
                            match tcp_stream.read(&mut buf) {
                                Ok(_) => {
                                    println!("request received: {}", String::from_utf8_lossy(&buf));
                                    let response = match Request::try_from(&buf[..]) {
                                        Ok(request) => handler.handle_request(&request),
                                        Err(e) => handler.handle_bad_request(&e),
                                    };
                                    if let Err(e) = response.send(&mut tcp_stream) {
                                        println!("Failed to send response: {}", e)
                                    }
                                }
                                Err(e) => println!("{:?}", e),
                            }
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
