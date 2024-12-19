use crate::event::{ButtonState, InputEvent, MouseAxis, MouseButton};
use crate::source::InputSource;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;

const DEV_MICE_PATH: &str = "/dev/input/mice";

const LMB_FLAG: u8 = 1;
const RMB_FLAG: u8 = 2;
const MMB_FLAG: u8 = 4;

#[derive(Default)]
pub struct MiceFileSource {
    scaling: f32,
    button_state: u8,
}

impl MiceFileSource {
    pub fn new() -> Self {
        Self {
            scaling: 1.0,
            button_state: 0,
        }
    }
}
macro_rules! handle_axis {
    ($delta_buff:expr, $scaling:expr, $axis_event:expr, $channel:expr) => {
        let delta = ($delta_buff as i8) as f32 * $scaling;
        if delta != 0.0f32 {
            let event = InputEvent::MouseMoveRel {
                axis: $axis_event,
                value: delta,
            };
            $channel.send(event).await.expect("Failed to send event");
        }
    };
}

macro_rules! handle_button {
    ($button_flag:expr, $button_state:expr, $stored_state:expr, $button_event:expr, $channel:expr) => {
        let state = $button_state & $button_flag;
        if state != $stored_state & $button_flag {
            let event = InputEvent::MouseButton {
                button: $button_event,
                state: if state != 0 {
                    ButtonState::Down
                } else {
                    ButtonState::Up
                },
            };
            $channel.send(event).await.expect("Failed to send event");
            $stored_state ^= $button_flag;
        }
    };
}

impl InputSource for MiceFileSource {
    async fn start_source(&mut self, channel: Sender<InputEvent>) {
        let mut file = File::open(DEV_MICE_PATH)
            .await
            .expect("Failed to open file");
        let mut buff = vec![0; 4];

        while let Ok(bytes_read) = file.read(&mut buff).await {
            if bytes_read < 3 {
                println!("Failed to read from file");
                break;
            }

            handle_button!(
                LMB_FLAG,
                buff[0],
                self.button_state,
                MouseButton::Left,
                channel
            );
            handle_button!(
                RMB_FLAG,
                buff[0],
                self.button_state,
                MouseButton::Right,
                channel
            );
            handle_button!(
                MMB_FLAG,
                buff[0],
                self.button_state,
                MouseButton::Middle,
                channel
            );

            handle_axis!(buff[1], self.scaling, MouseAxis::X, channel);
            handle_axis!(buff[2], self.scaling, MouseAxis::Y, channel);
        }
    }
}
