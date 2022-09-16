struct Server {
    addr: String,
}

impl Server {
    fn new(addr: &str) -> Self {
        Self { addr: addr.into() }
    }
    fn run(&self) {
        println!("server running at http://{}", self.addr)
    }
}

fn main() {
    let server = Server::new("127.0.0.1:8080");
    server.run()
}
