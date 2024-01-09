use crate::request::*;
use crate::request_builder::HTTPRequestBuilder;

pub struct HTTPRequestParser;

impl HTTPRequestParser {
    pub fn new() -> Self {Self}

    pub fn parse(&self, bytes: &[u8]) -> Option<HTTPRequest> {
        let mut builder = HTTPRequestBuilder::new();
        
        let mut body_index: Option<usize> = None;
        
        for (index, window) in bytes.windows(4).enumerate().collect::<Vec<_>>() {
            let val = String::from_utf8_lossy(window);
            
            if val == "\r\n\r\n" {
                body_index = Some(index+4);
                break;
            }
        }
        
        if body_index.is_none() {return None}; // Invalid HTTP Request        
        
        let body = bytes[body_index.unwrap()..].to_vec();
        
        let meta = String::from_utf8_lossy(&bytes[..body_index.unwrap()]);
        
        let lines = meta.lines().collect::<Vec<_>>();
        
        let status_words = lines.get(0)?.split_whitespace().collect::<Vec<_>>();
        let method = *status_words.get(0)?;                
        let mut uri = *status_words.get(1)?;
        let mut queries = vec![];
        let mut headers = vec![];

        let method = match method.to_uppercase().as_str() {
            "CONNECT" => HTTPMethod::CONNECT,
            "DELETE" => HTTPMethod::DELETE,
            "GET" => HTTPMethod::GET,
            "HEAD" => HTTPMethod::HEAD,
            "OPTIONS" => HTTPMethod::OPTIONS,
            "PATCH" => HTTPMethod::PATCH,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "TRACE" => HTTPMethod::TRACE,
            _ => return None
        };

        let header_strs = &lines[1..];

        header_strs.iter().for_each(|line| {
            if let Some((key, value)) = line.split_once(":") {
                headers.push((key.trim(), value.trim()));
            }
        });
        
        if let Some((true_uri, query_str)) = uri.split_once("?") {
            // handle uri and queries
            uri = true_uri;

            query_str.split("&").for_each(|val| {
                if let Some((key, value)) = val.split_once('=') {
                    queries.push((key.trim(), value.trim()));
                }
            })
        }
        
        let mut request = builder.with_body(Some(&body)).build(method, uri);
        queries.iter().for_each(|(key, value)| {request.add_query(key, value)});
        headers.iter().for_each(|(key, value)| {request.add_header(key, value)});
        todo!()
    }
}
