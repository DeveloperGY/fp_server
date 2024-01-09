use std::net::TcpStream;
use std::io::Read;

pub struct Receiver;

impl Receiver {
    pub fn new() -> Self {Self}

    pub fn receive(&self, stream: &mut TcpStream) -> Option<Vec<u8>> {
        let mut bytes = vec![];

        const BUFFER_SIZE: usize = 1024;
        let mut buffer = [0 as u8; BUFFER_SIZE];

        loop {
            let bytes_read = stream.read(&mut buffer).ok()?;

            bytes.extend_from_slice(&buffer[..bytes_read]);

            if bytes_read < BUFFER_SIZE {break};
        }

        Some(bytes)
    }
}
