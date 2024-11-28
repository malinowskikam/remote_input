use remote_input::event::InputEvent;
use remote_input::sink::{ConsoleSink, InputSink};
use remote_input::source::{/*VecSource, */ DevMiceSource, InputSource};
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    let mut events = vec![];
    events.push(InputEvent::MouseMoveRel { x: 0.1, y: 0.0 });
    events.push(InputEvent::MouseMoveRel { x: 0.1, y: 0.1 });
    events.push(InputEvent::MouseMoveRel { x: 0.0, y: 0.1 });

    let (tx, rx) = channel(100);

    // let mut source = VecSource::new(events);
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
