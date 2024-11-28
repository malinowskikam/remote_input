use crate::event::InputEvent;
use tokio::sync::mpsc::Receiver;

#[allow(async_fn_in_trait)]
pub trait InputSink {
    async fn start_sink(&mut self, channel: Receiver<InputEvent>);
}
