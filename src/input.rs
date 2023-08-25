pub enum Action {
    KeyPress,
    KeyRelease,
    Delay(crate::time::Delay),
}

pub struct KeySequence {
    inner: Vec<Action>,
}

impl KeySequence {
    pub fn new(seq: Vec<Action>) -> Self {
        Self::from(seq)
    }
}

impl From<Vec<Action>> for KeySequence {
    fn from(value: Vec<Action>) -> Self {
        Self { inner: value }
    }
}
