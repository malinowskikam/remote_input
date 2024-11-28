use crate::event::InputEvent;
use tokio::sync::mpsc::Sender;

#[allow(async_fn_in_trait)]
pub trait InputSource {
    async fn start_source(&mut self, channel: Sender<InputEvent>);
}
