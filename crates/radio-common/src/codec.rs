//! Message codec related functionality.

use {
    super::{FmtHeader, Header, Message, RfsHeader},
    std::{
        io::{self, Write},
        marker::PhantomData,
    },
    tokio_util::{
        bytes::{BufMut, BytesMut},
        codec::{Decoder, Encoder},
    },
};

/// A FMT message codec.
pub struct Fmt;

/// An RFS message codec.
pub struct Rfs;

/// A codec for decoding, and encoding messages.
pub trait Codec {
    fn decode(source: &mut BytesMut) -> io::Result<Option<Message>>;
    fn encode(message: Message, destination: &mut BytesMut) -> io::Result<()>;
}

impl Codec for Fmt {
    fn decode(source: &mut BytesMut) -> io::Result<Option<Message>> {
        if !FmtHeader::is_enough_data(source) {
            return Ok(None);
        }

        let FmtHeader {
            len,
            message_sequence,
            acknowledge_sequence,
            category,
            which,
            kind,
        } = FmtHeader::decode(source)?;

        let len = len as usize;

        if len > FmtHeader::MAX_MESSAGE_DATA_LEN {
            return Err(exceeds_capacity());
        } else if source.len() < len {
            return Ok(None);
        }

        let data = source[FmtHeader::HEADER_LEN..len].to_vec();

        Ok(Some(Message {
            message_sequence,
            acknowledge_sequence,
            category,
            which,
            kind,
            data,
        }))
    }

    fn encode(message: Message, destination: &mut BytesMut) -> io::Result<()> {
        let Message {
            message_sequence,
            acknowledge_sequence,
            category,
            which,
            kind,
            data,
        } = message;

        if data.len() > FmtHeader::MAX_MESSAGE_DATA_LEN {
            return Err(exceeds_capacity());
        }

        let len = (data.len() + FmtHeader::HEADER_LEN) as u16;
        let header = FmtHeader {
            len,
            message_sequence,
            acknowledge_sequence,
            category,
            which,
            kind,
        };

        header.encode(destination)?;
        destination.writer().write_all(&data)?;

        Ok(())
    }
}

impl Codec for Rfs {
    fn decode(source: &mut BytesMut) -> io::Result<Option<Message>> {
        if !RfsHeader::is_enough_data(source) {
            return Ok(None);
        }

        let RfsHeader {
            which, sequence, ..
        } = RfsHeader::decode(source)?;

        // TODO: Determine whether any of this is needed
        // let len = len as usize;

        // if len > RfsHeader::MAX_MESSAGE_DATA_LEN {
        //     return Err(exceeds_capacity());
        // } else if source.len() < len {
        //     return Ok(None);
        // }

        // let data = source[RfsHeader::HEADER_LEN..len].to_vec();

        Ok(Some(Message {
            message_sequence: 0,
            acknowledge_sequence: sequence,
            category: 0,
            which,
            kind: 0,
            data: Vec::new(),
        }))
    }

    fn encode(message: Message, destination: &mut BytesMut) -> io::Result<()> {
        let Message {
            message_sequence,
            which,
            data,
            ..
        } = message;

        // TODO: Determine whether any of this is needed
        // if data.len() > FmtHeader::MAX_MESSAGE_DATA_LEN {
        //     return Err(exceeds_capacity());
        // }

        let len = (data.len() + RfsHeader::HEADER_LEN) as u32;
        let header = RfsHeader {
            len,
            which,
            sequence: message_sequence,
        };

        header.encode(destination)?;

        Ok(())
    }
}

/// Returns an "exceeds capacity" error.
fn exceeds_capacity() -> io::Error {
    io::Error::new(
        io::ErrorKind::OutOfMemory,
        "Message length exceeds capacity",
    )
}

/// Adapt a [`Codec`] to tokio's codec traits.
pub(crate) struct CodecAdapter<C: Codec>(PhantomData<C>);

impl<C: Codec> CodecAdapter<C> {
    #[inline]
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: Codec> Decoder for CodecAdapter<C> {
    type Item = Message;
    type Error = io::Error;

    #[inline]
    fn decode(&mut self, source: &mut BytesMut) -> io::Result<Option<Message>> {
        <C as Codec>::decode(source)
    }
}

impl<C: Codec> Encoder<Message> for CodecAdapter<C> {
    type Error = io::Error;

    #[inline]
    fn encode(&mut self, message: Message, destination: &mut BytesMut) -> io::Result<()> {
        <C as Codec>::encode(message, destination)
    }
}
