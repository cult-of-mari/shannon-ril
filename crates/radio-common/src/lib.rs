pub(crate) use self::{
    codec::CodecAdapter,
    header::{FmtHeader, Header, RfsHeader},
    inner::Inner,
};

pub use self::{
    channel::{open, Receiver, Sender},
    codec::{Codec, Fmt, Rfs},
    message::Message,
};

mod channel;
mod codec;
mod header;
mod inner;
mod message;
