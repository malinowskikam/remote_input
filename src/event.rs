#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ButtonState {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InputEvent {
    MouseButton {
        button: MouseButton,
        state: ButtonState,
    },
    MouseMoveAbs {
        x: f32,
        y: f32,
    },
    MouseMoveRel {
        x: f32,
        y: f32,
    },
}
