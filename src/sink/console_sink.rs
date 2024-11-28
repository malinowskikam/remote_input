use tokio::sync::mpsc::Receiver;
use crate::event::InputEvent;
use crate::sink::InputSink;

pub struct ConsoleSink {}

impl ConsoleSink {
    pub fn new() -> Self { Self {} }
}

impl InputSink for ConsoleSink {
    async fn start_sink(&mut self, mut channel: Receiver<InputEvent>) {
        while let Some(event) = channel.recv().await {
            println!("Console sink: {:?}", event);            
        }
    }
}