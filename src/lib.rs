#[macro_use]
extern crate matches;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate httparse;
extern crate chrono;

mod header;
mod method;
mod status;
mod version;
mod request;
mod response;

use tokio_core::io::{Io, Codec, Framed, EasyBuf};
use tokio_proto::pipeline::ServerProto;

use std::io;

pub use header::Header;
pub use method::Method;
pub use status::Status;
pub use version::Version;
pub use request::Request;
pub use response::Response;

pub struct Http;

impl<T: Io + 'static> ServerProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Self::Transport>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(HttpCodec))
    }
}

pub struct HttpCodec;

impl Codec for HttpCodec {
    type In = Request;
    type Out = Response;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Request>> {
        request::decode(buf)
    }

    fn encode(&mut self, msg: Response, buf: &mut Vec<u8>) -> io::Result<()> {
        response::encode(msg, buf);
        Ok(())
    }
}
