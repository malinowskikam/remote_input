use crate::event::{ButtonState, InputEvent, MouseButton};
use crate::source::InputSource;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;

const DEV_MICE_PATH: &str = "/dev/input/mice";

const LMB_FLAG: u8 = 1;
const RMB_FLAG: u8 = 2;
const MMB_FLAG: u8 = 4;

pub struct DevMiceSource {}

impl DevMiceSource {
    pub fn new() -> Self {
        Self {}
    }
}

macro_rules! handle_button_state {
    ($button_state:expr, $buff:expr, $flag:expr, $button:expr, $channel:expr) => {
        if $button_state & $flag != $buff[0] & $flag {
            let state = if $buff[0] & $flag != 0 {
                ButtonState::Down
            } else {
                ButtonState::Up
            };
            $channel
                .send(InputEvent::MouseButton {
                    button: $button,
                    state,
                })
                .await
                .expect("Failed to send event");
            $button_state ^= $flag;
        }
    };
}

impl InputSource for DevMiceSource {
    async fn start_source(&mut self, channel: Sender<InputEvent>) {
        let mut file = File::open(DEV_MICE_PATH)
            .await
            .expect("Failed to open file");
        let mut buff = vec![0; 3];

        let mut button_state = 0u8;

        while let Ok(bytes_read) = file.read(&mut buff).await {
            if bytes_read != 3 {
                break;
            }

            let dx = buff[1] as i8;
            let dy = buff[2] as i8;

            if dx != 0 || dy != 0 {
                channel
                    .send(InputEvent::MouseMoveRel {
                        x: dx as f32,
                        y: dy as f32,
                    })
                    .await
                    .expect("Failed to send event");
            }

            handle_button_state!(button_state, buff, LMB_FLAG, MouseButton::Left, channel);
            handle_button_state!(button_state, buff, RMB_FLAG, MouseButton::Right, channel);
            handle_button_state!(button_state, buff, MMB_FLAG, MouseButton::Middle, channel);
        }
    }
}
