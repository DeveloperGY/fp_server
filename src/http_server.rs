mod request;
mod response;

use std::net::{
    ToSocketAddrs,
    TcpListener,
    TcpStream
};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

pub use request::{Request, RequestMethod};
pub use response:: *;
use request::RequestParser;

/**
 * An HTTP Server
 * Currently only works for HTTP 1.1
 */
pub struct HTTPServer<T> 
    where T: Sync + Send
{
    listener: TcpListener,
    handlers: HashMap<RequestMethod, Option<fn(Request, Arc<Mutex<T>>) -> Response>>,
    server_state: Arc<Mutex<T>>
}

impl<T> HTTPServer<T> 
    where T: Sync + Send
{
    /**
     * Creates an HTTP Server
     */
    pub fn new(addr: impl ToSocketAddrs, server_state: Arc<Mutex<T>>) -> Result<Self, String> {
        let listener = match TcpListener::bind(addr) {
            Ok(listener) => listener,
            Err(_) => return Err("Error: Failed to bind to address!".into())
        };

        let mut handlers = HashMap::new();

        handlers.insert(RequestMethod::Get, None);
        handlers.insert(RequestMethod::Head, None);
        handlers.insert(RequestMethod::Post, None);
        handlers.insert(RequestMethod::Put, None);
        handlers.insert(RequestMethod::Delete, None);
        handlers.insert(RequestMethod::Connect, None);
        handlers.insert(RequestMethod::Options, None);
        handlers.insert(RequestMethod::Trace, None);
        handlers.insert(RequestMethod::Patch, None);
        
        let server = Self {
            listener,
            handlers,
            server_state: server_state.clone()
        };

        Ok(server)
    }

    /**
     * Sets the handler for the given HTTP Request method
     */
    pub fn handle(&mut self, method: RequestMethod, handler: Option<fn(Request, Arc<Mutex<T>>) -> Response>) {
        self.handlers.insert(method, handler);
    }

    /**
     * Starts the HTTP server
     */
    pub fn start(&self) {
        std::thread::scope(|s| {
            for connection in self.listener.incoming() {
                if let Ok(mut stream) = connection {
                    let state = self.server_state.clone();
                    s.spawn(move || {
                        let request_bytes = match self.receive(&mut stream) {
                            Ok(bytes) => bytes,
                            Err(msg) => {
                                stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").unwrap();
                                println!("{}", msg);
                                return;
                            }
                        };

                        let request = match self.parse(&request_bytes) {
                            Ok(request) => request,
                            Err(msg) => {
                                stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").unwrap();
                                println!("{}", msg);
                                return;
                            }
                        };

                        if let Err(msg) = self.validate(&request) {
                            stream.write_all(b"HTTP/1.1 400 Bad Request\r\n\r\n").unwrap();
                            println!("{}", msg);
                            return;
                        }
        
                        if let Some(Some(handler)) = self.handlers.get(&request.method) {
                            let response = handler(request, state);
                            stream.write_all(&response.parse()).unwrap();
                        }
                        else {
                            stream.write_all(b"HTTP/1.1 500 :P\r\n\r\n").unwrap();
                        }
                    });
                }
            }
        });

        
    }

    /**
     * Recieves a request
     */
    fn receive(&self, stream: &mut TcpStream) -> Result<Vec<u8>, String> {
        const REQUEST_BUFFER_SIZE: usize = 1024;

        let mut buffer = [0; REQUEST_BUFFER_SIZE];
        let mut request = vec![];

        loop {
            if let Ok(bytes_read) = stream.read(&mut buffer) {
                request.extend_from_slice(&buffer[..bytes_read]);

                if bytes_read < REQUEST_BUFFER_SIZE {break}; // Finished reading request
            }
            else {
                return Err("Failed to read request!".into())
            }
        }

        Ok(request)
    }

    /**
     * Parses a request
     */
    fn parse(&self, request: &Vec<u8>) -> Result<Request, String>{
        let parser = RequestParser;

        let (request_string, body) = parser.split_body(request)?;
        let (method, uri, version) = parser.get_status_line(&request_string)?;
    
        let request = Request::new(method, &uri, &version, &body);
    
        Ok(request)
    }


    fn validate(&self, request: &Request) -> Result<(), String> {
        match request.version.as_str() {
            "HTTP/1.1" => (),
            _ => return Err("Invalid Request: Bad Version!".into())
        };
        
        Ok(())
    }
}