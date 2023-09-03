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

// impl Action {
//     pub fn exec(&self) {
//         match self {
//             Action::Wait(d) => d.wait(),
//             Action::KeyPress(key) => {
//                 if !key.is_pressed() {
//                     key.press()
//                 }
//             }
//             Action::KeyRelease(key) => {
//                 if key.is_pressed() {
//                     key.release();
//                 }
//             }
//             Action::MouseMovement(mode, amount) => match mode {
//                 CursorMovementMode::Relative => inputbot::MouseCursor::move_rel(amount.0, amount.1),
//                 CursorMovementMode::Absolute => inputbot::MouseCursor::move_abs(amount.0, amount.1),
//             },
//             Action::ButtonPress(btn) => {
//                 if !btn.is_pressed() {
//                     btn.press()
//                 }
//             }
//             Action::ButtonRelease(btn) => {
//                 if btn.is_pressed() {
//                     btn.release()
//                 }
//             }
//             Action::Scroll(dir, amount) => match dir {
//                 ScrollDirection::X => inputbot::MouseWheel::scroll_hor(*amount),
//                 ScrollDirection::Y => inputbot::MouseWheel::scroll_ver(*amount),
//             },
//             Action::Stop => {}
//         }
//     }
// }
