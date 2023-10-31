/**
 * An HTTP Response
 */
pub struct Response {
    version: String,
    status_code: usize,
    status_message: String,
    headers: Vec<String>,
    body: Vec<u8>
}

impl Response {
    /**
     * Creates a new HTTP Response
     */
    pub fn new(version: &str, status_code: usize, status_message: &str, body: &[u8]) -> Self {
        Self {
            version: version.to_string(),
            status_code,
            status_message: status_message.to_string(),
            headers: vec![],
            body: body.to_vec()
        }
    }

    /**
     * Creates an Ok Response (code 200)
     */
    pub fn create_200(version: &str, body: &[u8]) -> Self {
        Self::new(version, 200, "Ok", body)
    }

    /**
     * Creates a Not Found Response (code 404)
     */
    pub fn create_404(version: &str, body: &[u8]) -> Self {
        Self::new(version, 404, "Not Found", body)
    }

    /**
     * Adds a header to the response
     */
    pub fn add_header(&mut self, header: &str) {
        self.headers.push(header.to_string());
    }

    /**
     * Parses the response into a vector of bytes
     */
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