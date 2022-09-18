use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.into() }
    }
    pub fn run(&self) {
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
                                        Ok(request) => {
                                            dbg!(request);
                                            Response::new(
                                                StatusCode::Ok,
                                                Some("<h1> It works</h1>".to_string()),
                                            )
                                        }
                                        Err(e) => {
                                            println!("{:?}", e);
                                            Response::new(StatusCode::BadRequest, None)
                                        }
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
