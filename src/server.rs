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
            Ok(_) => println!("server running at http://{}", self.addr),
            Err(e) => println!("{:?}", e),
        }
    }
}
