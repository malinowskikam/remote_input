use log::info;
use crate::event::InputEvent;
use crate::source::InputSource;
use tokio::sync::mpsc::Sender;

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
        info!("Starting vec source");
        for event in &self.events {
            channel.send(*event).await.expect("Send Error");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{ButtonState, MouseAxis, MouseButton};

    #[tokio::test]
    async fn test_slice_source() {
        let mut events = vec![];
        events.push(InputEvent::MouseButton {
            button: MouseButton::Left,
            state: ButtonState::Down,
        });
        events.push(InputEvent::MouseMoveRel { axis: MouseAxis::X, value: 0.1 });
        events.push(InputEvent::MouseButton {
            button: MouseButton::Left,
            state: ButtonState::Up,
        });

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
