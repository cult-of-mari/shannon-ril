//! Channel device related functionality.

use {
    super::{Codec, CodecAdapter, Inner, Message},
    futures_util::{Sink, Stream},
    pin_project::pin_project,
    std::{
        io::{self},
        path::Path,
        pin::Pin,
        task::{Context, Poll},
    },
    tokio_util::codec::{FramedRead, FramedWrite},
};

/// A channel receiver.
#[pin_project]
pub struct Receiver<C: Codec> {
    #[pin]
    inner: FramedRead<Inner, CodecAdapter<C>>,
}

/// A channel sender.
#[pin_project]
pub struct Sender<C: Codec> {
    #[pin]
    inner: FramedWrite<Inner, CodecAdapter<C>>,
}

impl<C: Codec> Receiver<C> {
    /// Create a new `Receiver`.
    #[inline]
    pub(crate) fn new(inner: Inner) -> Self {
        let inner = FramedRead::new(inner, CodecAdapter::<C>::new());

        Self { inner }
    }
}

impl<C: Codec> Sender<C> {
    /// Create a new `Sender`.
    #[inline]
    pub(crate) fn new(inner: Inner) -> Self {
        let inner = FramedWrite::new(inner, CodecAdapter::<C>::new());

        Self { inner }
    }
}

impl<C: Codec> Stream for Receiver<C> {
    type Item = io::Result<Message>;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

impl<C: Codec> Sink<Message> for Sender<C> {
    type Error = io::Error;

    #[inline]
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_ready(cx)
    }

    #[inline]
    fn start_send(self: Pin<&mut Self>, message: Message) -> io::Result<()> {
        self.project().inner.start_send(message)
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_flush(cx)
    }

    #[inline]
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project().inner.poll_close(cx)
    }
}

/// Open a channel device.
pub fn open<C: Codec, P: AsRef<Path>>(path: P) -> io::Result<(Sender<C>, Receiver<C>)> {
    let receiver = Inner::open(path)?;
    let sender = receiver.clone();

    Ok((Sender::new(sender), Receiver::new(receiver)))
}
