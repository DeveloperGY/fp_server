use super::request::HTTPRequest;
use super::response::HTTPResponse;

pub trait HTTPRequestHandler: Send + Sync {
    fn run(&self, request: HTTPRequest) -> HTTPResponse;
}
