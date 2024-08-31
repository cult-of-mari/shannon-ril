//! Message header related functionality.

use {
    binrw::{
        meta::{ReadEndian, WriteEndian},
        BinRead, BinWrite,
    },
    std::io::{self, Write},
    tokio_util::bytes::{BufMut, BytesMut},
};

/// A message header.
pub trait Header: BinRead + BinWrite + ReadEndian + WriteEndian
where
    for<'a> <Self as BinRead>::Args<'a>: Default,
    for<'a> <Self as BinWrite>::Args<'a>: Default,
{
    /// The length of this header (in bytes).
    const HEADER_LEN: usize;

    /// The maximum length of a message (in bytes).
    const MAX_MESSAGE_LEN: usize;

    /// The maximum length of message data (in bytes).
    const MAX_MESSAGE_DATA_LEN: usize = Self::MAX_MESSAGE_LEN - Self::HEADER_LEN;

    /// Determine whether `source` has enough data to decode a header.
    fn is_enough_data(source: &mut BytesMut) -> bool {
        source.len() < Self::HEADER_LEN
    }

    /// Decode a header.
    fn decode(source: &mut BytesMut) -> io::Result<Self> {
        Self::read(&mut io::Cursor::new(&source[..])).map_err(into_error)
    }

    /// Encode into `destination`.
    fn encode(&self, destination: &mut BytesMut) -> io::Result<()>;
}

/// Implement a message header.
macro_rules! impl_header {
    ($ty:ty, $header_len:expr, $max_message_len:expr) => {
        impl $ty {
            /// Encode into a byte array.
            pub fn to_bytes(self) -> io::Result<[u8; Self::HEADER_LEN]> {
                let mut cursor = io::Cursor::new([0; Self::HEADER_LEN]);

                self.write(&mut cursor).map_err(into_error)?;

                Ok(cursor.into_inner())
            }
        }

        impl Header for $ty {
            const HEADER_LEN: usize = $header_len;
            const MAX_MESSAGE_LEN: usize = $max_message_len;

            fn encode(&self, destination: &mut BytesMut) -> io::Result<()> {
                destination.writer().write_all(&self.to_bytes()?)?;

                Ok(())
            }
        }
    };
}

/// A FMT message header.
#[binrw::binrw]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[brw(little)]
pub struct FmtHeader {
    pub len: u16,
    pub message_sequence: u8,
    pub acknowledge_sequence: u8,
    pub category: u8,
    pub which: u8,
    pub kind: u8,
}

/// An RFS message header.
#[binrw::binrw]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[brw(little)]
pub struct RfsHeader {
    pub len: u32,
    pub which: u8,
    pub sequence: u8,
}

impl_header!(FmtHeader, 7, u16::MAX as usize);
impl_header!(RfsHeader, 6, u32::MAX as usize);

/// Transform a `binrw::Error` into an `io::Error`.
fn into_error(error: binrw::Error) -> io::Error {
    match error {
        binrw::Error::Io(error) => error,
        error => io::Error::other(error),
    }
}
