use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Version {
    Http10,
    Http11,
    Http20,
}

impl Version {
    pub fn from_httparse(v: u8) -> Version {
        match v {
            0 => Version::Http10,
            1 => Version::Http11,
            // TODO(nokaa): We want a better behaviour here.
            _ => Version::Http11,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Version::Http10 => f.write_str("HTTP/1.0"),
            Version::Http11 => f.write_str("HTTP/1.1"),
            Version::Http20 => f.write_str("HTTP/2.0"),
        }
    }
}
