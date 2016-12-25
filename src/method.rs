/// The different HTTP methods that a request might use.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Method {
    Connect,
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
    Trace,
    Other(String),
}

impl<'a> From<&'a str> for Method {
    fn from(s: &'a str) -> Method {
        match s {
            "CONNECT" => Method::Connect,
            "DELETE" => Method::Delete,
            "GET" => Method::Get,
            "HEAD" => Method::Head,
            "OPTIONS" => Method::Options,
            "PATCH" => Method::Patch,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "TRACE" => Method::Trace,
            s => Method::Other(s.to_string()),
        }
    }
}
