use {
    futures_util::StreamExt,
    radio_common::{Fmt, Rfs},
    std::io,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let (_fmt_tx, mut fmt_rx) = radio_common::open::<Fmt, _>("/dev/umts_ipc0")?;
    let (_rfs_tx, mut rfs_rx) = radio_common::open::<Rfs, _>("/dev/umts_rfs0")?;

    loop {
        tokio::select! {
            message = fmt_rx.next() => {
                tracing::info!("fmt: {message:?}");
            },
            message = rfs_rx.next() => {
                tracing::info!("rfs: {message:?}");
            },
        };
    }
}
