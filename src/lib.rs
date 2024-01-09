mod handler;
mod request;
mod response;
mod request_builder;
mod receiver;
mod parser;
mod serializer;

use request::*;
use response::HTTPResponse;
use handler::HTTPRequestHandler;
use receiver::Receiver;
use parser::HTTPRequestParser;
use serializer::HTTPResponseSerializer;

use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

pub struct HTTPServer {
    listener: TcpListener,
    handlers: HashMap<HTTPMethod, Option<Box<dyn HTTPRequestHandler>>>
}

impl HTTPServer {
    pub fn new(
        address: &impl std::net::ToSocketAddrs
    ) -> Result<Self, HTTPServerError> {
        let listener = TcpListener::bind(address)
            .map_err(|_| {HTTPServerError::AddressBindFailure})?;    

        let mut handlers = HashMap::new();

        handlers.insert(HTTPMethod::CONNECT, None);
        handlers.insert(HTTPMethod::DELETE, None);
        handlers.insert(HTTPMethod::GET, None);
        handlers.insert(HTTPMethod::HEAD, None);
        handlers.insert(HTTPMethod::OPTIONS, None);
        handlers.insert(HTTPMethod::PATCH, None);
        handlers.insert(HTTPMethod::POST, None);
        handlers.insert(HTTPMethod::PUT, None);
        handlers.insert(HTTPMethod::TRACE, None);

        Ok(Self {
            listener,
            handlers
        })
    }

    pub fn run(&self) {
        std::thread::scope(|t| {
            self.listener.incoming().for_each(|res| {
                if let Ok(stream) = res {
                    t.spawn(|| {self.handle_connection(stream)});
                }
            });
        });
    }

    pub fn set_handler(
        &mut self,
        method: HTTPMethod,
        handler: Option<Box<dyn HTTPRequestHandler>>
    ) {
        self.handlers.insert(method, handler);
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Option<()> {
        let receiver = Receiver::new();
        let parser = HTTPRequestParser::new();
        let serializer = HTTPResponseSerializer::new();

        let request_bytes = receiver.receive(&mut stream)?;
        let request = parser.parse(&request_bytes)?;
        let response = if let Some(handler) = self.handlers.get(&request.get_method()) {
            if let Some(handler) = handler {
                handler.run(request)
            }
            else {
                HTTPResponse::new("Unimplemented", 501, &[])
            }
        } else {
            println!("Invalid Request Method!");
            HTTPResponse::new("Invalid Request Method!", 400, &[])
        };
        let bytes = serializer.serialize(&response);
        stream.write(&bytes).ok().map(|_| {})
    }
}

pub enum HTTPServerError {
    AddressBindFailure
}
