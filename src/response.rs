use std::collections::HashMap;

pub struct HTTPResponse {
    msg: String,
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>
}

impl HTTPResponse {
    pub fn new(
        msg: &str,
        status_code: u32,
        body: &[u8]
    ) -> Self {
        Self {
            code: status_code,
            msg: msg.to_string(),
            headers: HashMap::new(),
            body: body.to_vec()
        }
    }

    /// Adds a header to the request, if the header already exists, it replaces
    /// it
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn get_body(&self) -> Vec<u8> {
        self.body[..].to_vec()
    }

    pub fn get_headers(&self) -> Vec<(String, String)> {
        self.headers.iter().map(|(key, value)| {((*key).clone(), (*value).clone())}).collect::<Vec<_>>()
    }

    pub fn get_msg(&self) -> String {
        self.msg.clone()
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }
}
