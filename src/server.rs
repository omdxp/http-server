use std::io::Read;
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
