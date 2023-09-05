#[derive(PartialEq, Debug, Clone)]
pub enum Action {
    Wait(crate::time::Delay),
    KeyPress(inputbot::KeybdKey),
    KeyRelease(inputbot::KeybdKey),
    MouseMovement(CursorMovementMode, (i32, i32)), // mode, amount
    ButtonPress(inputbot::MouseButton),
    ButtonRelease(inputbot::MouseButton),
    Scroll(ScrollDirection, i32), // direction, amount
    // Condition(Action, Condition),
    // AbsoluteJump(usize),         // position
    // RelativeJump(usize), // jump length, negative for backwards
    Stop,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ScrollDirection {
    X,
    Y,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CursorMovementMode {
    Relative,
    Absolute,
}
