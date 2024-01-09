use crate::request::*;

use std::collections::HashMap;

pub struct HTTPRequestBuilder {
    body: Option<Vec<u8>>,    
    headers: HashMap<String, String>,
    queries: HashMap<String, String>
}

impl HTTPRequestBuilder {
    pub fn new() -> Self {
        Self {
            body: None,
            headers: HashMap::new(),
            queries: HashMap::new()
        }
    }
    
    pub fn clear(&mut self) -> &mut Self {
        self.body = None;
        self.headers.clear();
        self.queries.clear();

        self
    }

    pub fn add_query(&mut self, key: &str, value: &str) -> &mut Self {
        self.queries.insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_body(&mut self, body: Option<&[u8]>) -> &mut Self {
        self.body = body.map(|bytes| {bytes.to_vec()});
        self
    }
    
    pub fn build(&self, method: HTTPMethod, uri: &str) -> HTTPRequest {
        let body = self.body.as_ref()
            .map(|val| {val.as_slice()})
            .unwrap_or(&[]);

        let mut request = HTTPRequest::new(method, uri, body);

        self.headers.iter().for_each(|(key, value)| {
            request.add_header(key, value);
        });

        self.queries.iter().for_each(|(key, value)| {
            request.add_query(key, value);
        });

        request
    }
}
