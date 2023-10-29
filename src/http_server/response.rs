pub struct Response {
    version: String,
    status_code: usize,
    status_message: String,
    headers: Vec<String>,
    body: Vec<u8>
}

impl Response {
    pub fn new(version: &str, status_code: usize, status_message: &str, body: &[u8]) -> Self {
        Self {
            version: version.to_string(),
            status_code,
            status_message: status_message.to_string(),
            headers: vec![],
            body: body.to_vec()
        }
    }

    pub fn add_header(&mut self, header: &str) {
        self.headers.push(header.to_string());
    }

    pub fn parse(&self) -> Vec<u8> {
        let mut response_bytes = vec![];

        response_bytes.extend_from_slice(self.version.as_bytes());
        response_bytes.push(b' ');
        response_bytes.extend_from_slice(self.status_code.to_string().as_bytes());
        response_bytes.push(b' ');
        response_bytes.extend_from_slice(self.status_message.as_bytes());
        response_bytes.extend_from_slice(b"\r\n");
        for header in &self.headers {
            response_bytes.extend_from_slice(header.as_bytes());
            response_bytes.extend_from_slice(b"\r\n");
        }
        response_bytes.extend_from_slice(b"\r\n");
        response_bytes.extend_from_slice(&self.body);

        response_bytes
    }
}