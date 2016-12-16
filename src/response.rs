use std::default::Default;
use std::fmt::{self, Write};
use std::io;
use std::io::Write as IoWrite;

use chrono::UTC;

use status::Status;

#[derive(Clone, Debug)]
pub struct Response {
    headers: Vec<(String, String)>,
    response: Vec<u8>,
    status_code: u32,
    status_message: &'static str,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            headers: Vec::new(),
            response: Vec::new(),
            status_code: 200,
            status_message: "OK",
        }
    }
}

impl Response {
    pub fn new() -> Response {
        Default::default()
    }

    pub fn status(&mut self, status: Status) -> &mut Response {
        self.status_code = status.code();
        self.status_message = status.reason();
        self
    }

    pub fn custom_status(&mut self, code: u32, reason: &'static str) -> &mut Response {
        self.status_code = code;
        self.status_message = reason;
        self
    }

    pub fn add_header(&mut self, name: String, value: String) -> &mut Response {
        self.headers.push((name, value));
        self
    }

    pub fn body(&mut self, body: &[u8]) -> Result<&mut Response, io::Error> {
        self.write_all(body)?;
        Ok(self)
    }
}

impl IoWrite for Response {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.response.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

pub fn encode(msg: Response, buf: &mut Vec<u8>) {
    let code = msg.status_code;
    let message = msg.status_message;
    let length = msg.response.len();
    let now = UTC::now().to_rfc2822();

    write!(FastWrite(buf),
           "\
        HTTP/1.1 {} {}\r\nServer: Example\r\nContent-Length: {}\r\nDate: {}\r\n",
           code,
           message,
           length,
           now)
        .unwrap();

    for &(ref name, ref value) in &msg.headers {
        buf.extend_from_slice(name.as_bytes());
        buf.extend_from_slice(b": ");
        buf.extend_from_slice(value.as_bytes());
        buf.extend_from_slice(b"\r\n");
    }

    buf.extend_from_slice(b"\r\n");
    let mut response = msg.response;
    buf.append(&mut response);
}

struct FastWrite<'a>(&'a mut Vec<u8>);

impl<'a> fmt::Write for FastWrite<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, args)
    }
}
