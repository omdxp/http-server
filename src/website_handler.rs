use crate::http::{Request, Response, StatusCode};
use crate::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".into()))
    }
}
