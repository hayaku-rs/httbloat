use tokio_core::io::EasyBuf;
use httparse;

use std::io;

use header::Header;
use method::Method;
use version::Version;

const MIN_HEADERS: usize = 16;
const MAX_HEADERS: usize = 1024;

type Slice = (usize, usize);

pub struct Request {
    method: Method,
    path: String,
    version: Version,
    headers: Vec<(Header, String)>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub fn method(&self) -> Method {
        self.method.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn headers(&self) -> Vec<(Header, String)> {
        self.headers.clone()
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    pub fn body(&self) -> Option<&[u8]> {
        match self.body {
            Some(ref b) => Some(b),
            None => None,
        }
    }
}

pub fn decode(buf: &mut EasyBuf) -> io::Result<Option<Request>> {
    let (method, path, version, headers, amt) = {
        let mut headers = [httparse::EMPTY_HEADER; MIN_HEADERS];
        let mut vec;
        let mut parser = httparse::Request::new(&mut headers);
        let mut result = parser.parse(buf.as_slice());
        if matches!(result, Err(httparse::Error::TooManyHeaders)) {
            vec = vec![httparse::EMPTY_HEADER; MAX_HEADERS];
            parser = httparse::Request::new(&mut vec);
            result = parser.parse(buf.as_slice());
        }

        let amt = match result {
            Ok(httparse::Status::Complete(amt)) => amt,
            Ok(httparse::Status::Partial) => return Ok(None),
            Err(e) => {
                let msg = format!("failed to parse http request: {:?}", e);
                return Err(io::Error::new(io::ErrorKind::Other, msg));
            }
        };
        let version = Version::from_httparse(parser.version.unwrap());
        let method = Method::from(parser.method.unwrap());
        let path = parser.path.unwrap().to_string();
        let mut headers = Vec::with_capacity(parser.headers.len());
        for header in parser.headers {
            let name = Header::from(header.name);
            let value = String::from_utf8_lossy(header.value).into_owned();
            headers.push((name, value))
        }
        (method, path, version, headers, amt)
    };

    let _ = buf.drain_to(amt);
    let body = if buf.len() > 0 {
        Some(buf.as_slice().to_vec())
    } else {
        None
    };

    Ok(Some(Request {
        method: method,
        path: path,
        version: version,
        headers: headers,
        body: body,
    }))
}
