//! Internal channel device structure.

use {
    futures_util::ready,
    std::{
        fs::File,
        io::{self, Read, Write},
        path::Path,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
    tokio::io::{unix::AsyncFd, AsyncRead, AsyncWrite, ReadBuf},
};

/// Internal reference-counted channel device.
#[derive(Clone)]
pub struct Inner {
    inner: Arc<AsyncFd<File>>,
}

impl Inner {
    /// Open a channel device.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let inner = File::options()
            .read(true)
            .write(true)
            .open(path)
            .and_then(AsyncFd::new)
            .map(Arc::new)?;

        Ok(Self { inner })
    }
}

impl AsyncRead for Inner {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = ready!(self.inner.poll_read_ready(cx))?;
            let unfilled = buf.initialize_unfilled();

            match guard.try_io(|inner| inner.get_ref().read(unfilled)) {
                // An empty channel returns 0, wait for data.
                Ok(Ok(0)) => return Poll::Pending,
                Ok(Ok(len)) => {
                    buf.advance(len);

                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(_would_block) => continue,
            }
        }
    }
}

impl AsyncWrite for Inner {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {
            let mut guard = ready!(self.inner.poll_write_ready(cx))?;

            match guard.try_io(|inner| inner.get_ref().write(buf)) {
                Ok(result) => return Poll::Ready(result),
                Err(_would_block) => continue,
            }
        }
    }

    /// No-op for a channel device.
    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    /// No-op for a channel device.
    #[inline]
    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}
