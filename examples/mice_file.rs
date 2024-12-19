#[cfg(all(feature = "dev_mice", target_os = "linux"))]
#[tokio::main]
async fn main() {
    use log::info;
    use remote_input::sink::{ConsoleSink, InputSink};
    use remote_input::source::{MiceFileSource, InputSource};
    use tokio::sync::mpsc::channel;

    const TIMEOUT: u64 = 10000;

    env_logger::init();
    info!("Starting");

    let (tx, rx) = channel(100);

    info!("Starting Sink");
    let mut sink = ConsoleSink::new();
    let sink_handle = tokio::spawn(async move {
        sink.start_sink(rx).await;
    });

    info!("Starting source");
    let mut source = MiceFileSource::new();
    let source_handle = tokio::spawn(async move {
        source.start_source(tx).await;
    });

    info!(
        "Waiting for source and sink to finish, timeout: {}ms",
        TIMEOUT
    );
    let stop_handle = tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(TIMEOUT)).await;
        source_handle.abort();
        sink_handle.abort();
        info!("Pipeline stopped");
    });

    stop_handle.await.expect("Could not stop pipeline");
}

#[cfg(not(all(feature = "dev_mice", target_os = "linux")))]
fn main() {
    use log::error;
    env_logger::init();
    error!("This example works only on Linux and requires the 'dev_mice' feature");
}
