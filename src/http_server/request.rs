/**
 * HTTP Request Methods
 */
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequestMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

/**
 * An HTTP Request
 */
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Request {
    pub method: RequestMethod,
    pub uri: String,
    pub version: String,
    pub body: Vec<u8>
}

impl Request {
    /**
     * Creates an HTTP Request
     */
    pub fn new(method: RequestMethod, uri: &str, version: &str, body: &[u8]) -> Self {
        Self {
            method,
            uri: uri.to_string(),
            version: version.to_string(),
            body: body.to_vec()
        }
    }
}

/**
 * Parses an HTTP Request from bytes
 */
pub struct RequestParser;

impl RequestParser {
    pub fn split_body(&self, bytes: &[u8]) -> Result<(String, Vec<u8>), String> {
        const BODY_DELIMITER: &[u8; 4] = b"\r\n\r\n";
        let mut body: Vec<u8> = vec![];
        let mut has_valid_body = false;
        let mut body_index = 0;

        if bytes.len() < 4 {
            return Err("Invalid Request: Request not long enough!".into())
        }

        for i in 0..bytes.len()-3 {
            let byte_matches = [
                bytes[i]   == BODY_DELIMITER[0],
                bytes[i+1] == BODY_DELIMITER[1],
                bytes[i+2] == BODY_DELIMITER[2],
                bytes[i+3] == BODY_DELIMITER[3]
            ];

            let matches_delimiter = byte_matches[0] && byte_matches[1] && byte_matches[2] && byte_matches[3];
    
            if matches_delimiter {
                body_index = i+4;
                has_valid_body = true;
            }
        }
        
        if !has_valid_body {
            return Err("Invalid Request: No Body!".into());
        }
        
        let request_string = String::from_utf8_lossy(&bytes[..body_index]);
        body.extend_from_slice(&bytes[body_index..]);

        Ok((request_string.to_string(), body))
    }

    pub fn get_status_line(&self, request_string: &str) -> Result<(RequestMethod, String, String), String> {
        let request_lines: Vec<_> = request_string.lines().collect();
    
        // Parse status line
        let status_line = match request_lines.get(0) {
            Some(val) => val,
            None => return Err("Invalid Request: No Status Line!".into())
        };
    
        let status_words: Vec<_> = status_line.split_whitespace().collect();
        
    
        let method_str = match status_words.get(0) {
            Some(val) => val,
            None => return Err("Invalid Request: No Request Type!".into())
        };
    
        let method = match method_str.to_uppercase().as_str() {
            "GET" => RequestMethod::Get,
            "HEAD" => RequestMethod::Head,
            "POST" => RequestMethod::Post,
            "PUT" => RequestMethod::Put,
            "DELETE" => RequestMethod::Delete,
            "CONNECT" => RequestMethod::Connect,
            "OPTIONS" => RequestMethod::Options,
            "TRACE" => RequestMethod::Trace,
            "PATCH" => RequestMethod::Patch,
            _ => return Err("Invalid Request: Invalid Request Type!".into())
        };
    
        let uri = match status_words.get(1) {
            Some(val) => val.to_string(),
            None => return Err("Invalid Request: No URI!".into())
        };
    
        let version = match status_words.get(2) {
            Some(val) => val.to_string(),
            None => return Err("Invalid Request: No Version!".into())
        };

        Ok((method, uri, version))
    }
}