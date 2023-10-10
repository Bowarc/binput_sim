#[derive(PartialEq, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Action {
    Wait(crate::time::Delay),
    KeyPress(inputbot::KeybdKey),
    KeyRelease(inputbot::KeybdKey),
    MouseMovement(CursorMovementMode, (i32, i32)), // mode, amount
    ButtonPress(inputbot::MouseButton),
    ButtonRelease(inputbot::MouseButton),
    Scroll(ScrollDirection, i32), // direction, amount
    KeySequence(String),
    // Condition(Action, Condition),
    // AbsoluteJump(usize),         // position
    // RelativeJump(usize), // jump length, negative for backwards
    Stop,
}

#[derive(Debug, PartialEq, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum ScrollDirection {
    X,
    Y,
}

#[derive(Debug, PartialEq, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum CursorMovementMode {
    Relative,
    Absolute,
}
