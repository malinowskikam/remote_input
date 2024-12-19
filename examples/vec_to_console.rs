use log::info;
use remote_input::event::{ButtonState, InputEvent, MouseAxis, MouseButton};
use remote_input::sink::{ConsoleSink, InputSink};
use remote_input::source::{InputSource, VecSource};
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    const TIMEOUT: u64 = 3000;

    env_logger::init();
    info!("Starting");

    let (tx, rx) = channel(100);

    info!("Starting Sink");
    let mut sink = ConsoleSink::new();
    let sink_handle = tokio::spawn(tokio::time::timeout(
        tokio::time::Duration::from_secs(TIMEOUT),
        async move { sink.start_sink(rx).await },
    ));

    info!("Starting source");
    let mut source = VecSource::new(vec![
        InputEvent::MouseMoveRel {
            axis: MouseAxis::X,
            value: 1.0,
        },
        InputEvent::MouseMoveRel {
            axis: MouseAxis::Y,
            value: 1.0,
        },
        InputEvent::MouseMoveRel {
            axis: MouseAxis::Wheel,
            value: 1.0,
        },
        InputEvent::MouseButton {
            button: MouseButton::Left,
            state: ButtonState::Down,
        },
        InputEvent::MouseButton {
            button: MouseButton::Left,
            state: ButtonState::Up,
        },
        InputEvent::MouseButton {
            button: MouseButton::Right,
            state: ButtonState::Down,
        },
        InputEvent::MouseButton {
            button: MouseButton::Right,
            state: ButtonState::Up,
        },
        InputEvent::MouseButton {
            button: MouseButton::Middle,
            state: ButtonState::Down,
        },
        InputEvent::MouseButton {
            button: MouseButton::Middle,
            state: ButtonState::Up,
        },
    ]);
    let source_handle = tokio::spawn(tokio::time::timeout(
        tokio::time::Duration::from_secs(TIMEOUT),
        async move { source.start_source(tx).await },
    ));

    source_handle.await.unwrap().unwrap();
    sink_handle.await.unwrap().unwrap();

    info!("Pipeline stopped");
}
