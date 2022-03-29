use super::StatusCode;
use std::net::TcpStream;
use std::io::{Write, Result as IoResult};
use std::fmt::{Display, Formatter, Result as FmtResult};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        // /     Static dispatch:   ^^^^ ^^^^^ we can put any type here - TcpStream, File etc. and the compiler will figure it out
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code,
            self.status_code.reason_phrase(), 
            body
            )
    }
}
