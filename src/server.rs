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
                        Ok((tcp_stream, _)) => {}
                        Err(e) => println!("{:?}", e),
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
