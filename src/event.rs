#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseAxis {
    X,
    Y,
    Wheel,
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
    MouseMoveRel {
        axis: MouseAxis,
        value: f32,
    },
}
