use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HTTPMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE
}

pub struct HTTPRequest {
    method: HTTPMethod,
    uri: String,
    headers: HashMap<String, String>,
    queries: HashMap<String, String>,
    body: Vec<u8>
}

impl HTTPRequest {
    pub fn new(
        method: HTTPMethod,
        uri: &str,
        body: &[u8]
    ) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            headers: HashMap::new(),
            queries: HashMap::new(),
            body: body.to_vec()
        }
    }

    /// Adds a header to the request, if the header already exists, it replaces
    /// it
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    // Adds a query to the request, if the query already exists, it replaces it
    pub fn add_query(&mut self, key: &str, value: &str) {
        self.queries.insert(key.to_string(), value.to_string());
    }

    pub fn get_method(&self) -> HTTPMethod {
        self.method
    }

    pub fn get_uri(&self) -> String {
        self.uri.clone()
    }

    pub fn get_body(&self) -> Vec<u8> {
        self.body[..].to_vec() // just to ensure deep copy
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        self.headers.get(key).map(|val| {(*val).clone()})
    }

    pub fn get_query(&self, key: &str) -> Option<String> {
        self.queries.get(key).map(|val| {(*val).clone()})
    }
}
