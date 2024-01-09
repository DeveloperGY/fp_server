use super::response::HTTPResponse;

pub struct HTTPResponseSerializer;

impl HTTPResponseSerializer {
    pub fn new() -> Self {Self}

    pub fn serialize(&self, mut response: HTTPResponse) -> Vec<u8> {
        if !response.has_header("Content-Length") {
            response.add_header("Content-Length", response.get_body().len().to_string().as_str());
        }

        let mut str = format!("HTTP/1.1 {} {}\r\n", response.get_code(), response.get_msg());
        
        response.get_headers().iter().for_each(|(key, value)| {
            str += format!("{}: {}\r\n", key, value).as_str();
        });
        str += "\r\n";

        let mut bytes = str.bytes().collect::<Vec<_>>();

        bytes.extend_from_slice(&response.get_body());

        bytes
    }
}