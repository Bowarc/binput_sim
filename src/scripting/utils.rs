pub fn release_all_kbkeys() {
    use strum::IntoEnumIterator as _;
    for kbkey in inputbot::KeybdKey::iter() {
        if kbkey.is_pressed() {
            kbkey.release()
        }
    }
}

pub fn release_all_mouse_btns() {
    use strum::IntoEnumIterator as _;
    for btn in inputbot::MouseButton::iter() {
        if btn.is_pressed() {
            btn.release()
        }
    }
}
