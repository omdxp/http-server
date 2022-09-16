pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self { addr: addr.into() }
    }
    pub fn run(&self) {
        println!("server running at http://{}", self.addr)
    }
}
