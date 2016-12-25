use std::convert::From;
use std::ascii::AsciiExt;

/// Headers that may be used by an HTTP request or response.
#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Host,
    Connection,
    KeepAlive,
    ContentLength,
    TransferEncoding,
    Raw(String),
}

impl<'a> From<&'a str> for Header {
    fn from(s: &'a str) -> Header {
        if s.eq_ignore_ascii_case("Host") {
            Header::Host
        } else if s.eq_ignore_ascii_case("Connection") {
            Header::Connection
        } else if s.eq_ignore_ascii_case("Keep-Alive") {
            Header::KeepAlive
        } else if s.eq_ignore_ascii_case("Content-Length") {
            Header::ContentLength
        } else if s.eq_ignore_ascii_case("Transfer-Encoding") {
            Header::TransferEncoding
        } else {
            Header::Raw(s.to_string())
        }
    }
}
