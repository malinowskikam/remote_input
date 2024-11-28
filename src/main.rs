use remote_input::sink::{ConsoleSink, InputSink};
use remote_input::source::{DevMiceSource, InputSource};
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let (tx, rx) = channel(100);

    let mut source = DevMiceSource::new();
    let source_handle = tokio::spawn(async move { source.start_source(tx).await });

    let mut sink = ConsoleSink::new();
    let sink_handle = tokio::spawn(async move { sink.start_sink(rx).await });

    let stop_handle = tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(9999)).await;
        source_handle.abort();
        sink_handle.abort();
    });

    stop_handle.await.expect("Could not stop pipeline");
}
