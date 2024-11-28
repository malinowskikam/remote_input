use tokio::sync::mpsc::Sender;
use crate::event::InputEvent;
use crate::source::InputSource;

pub struct VecSource {
    events: Vec<InputEvent>,
}

impl VecSource {
    pub fn new(events: Vec<InputEvent>) -> Self {
        Self { events }
    }
}

impl InputSource for VecSource {
    async fn start_source(&mut self, channel: Sender<InputEvent>) {
        for event in &self.events {
            channel.send(event.clone().into()).await.expect("Send Error");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{ButtonState, MouseButton};
    use super::*;

    #[tokio::test]
    async fn test_slice_source() {
        let mut events = vec![];
        events.push(InputEvent::MouseButton { button: MouseButton::Left, state: ButtonState::Down });
        events.push(InputEvent::MouseMoveRel { x: 0.1, y: 0.1 });
        events.push(InputEvent::MouseButton { button: MouseButton::Left, state: ButtonState::Up });
        
        let mut source = VecSource::new(events.clone());
        let (tx, mut rx) = tokio::sync::mpsc::channel(3);
        
        source.start_source(tx).await;
        
        let mut output = vec![];
        while let Some(event) = rx.recv().await {
            output.push(event);
        }
        
        assert_eq!(events, output);
    }
}