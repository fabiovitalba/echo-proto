/*
 * Most of the Decsriptions were taken from the official Tokio.rs documentation.
 * Be sure to check their tutorial out if you want to know more:
 * https://tokio.rs/docs/getting-started/simple-server
 */

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf, Io, Framed};
use tokio_proto::pipeline::ServerProto;

pub struct LineCodec;

impl Codec for LineCodec {
    /* Codecs in Tokio implement the Codec trait, which implements message encoding and decoding.
     * To start with, we’ll need to specify the message type. In gives the types of incoming
     * messages after decoding, while Out gives the type of outgoing messages prior to encoding.
     */
    type In = String;
    type Out = String;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            return match str::from_utf8(&line.as_ref()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend_from_slice(msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

pub struct LineProto;

impl<T: Io + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = String;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}

fn main() {
    /* Some Documentation about Tokio.rs and how it works:
     *
     * A server in tokio-proto is made up of three distinct parts:
     * A transport, which manages serialization of Rust request and response types to the underlying socket. In this guide, we will implement this using the Codec helper.
     * A protocol specification, which puts together a codec and some basic information about the protocol (is it multiplexed? streaming?).
     * A service, which says how to produce a response given a request. A service is basically an asynchronous function.
     */



    println!("Hello, world!");
}
