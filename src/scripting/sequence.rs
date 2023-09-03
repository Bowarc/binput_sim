#[derive(Debug, PartialEq, Clone)]
pub struct KeySequence {
    seq: Vec<super::Action>,
    cursor: usize,
    requested_stop: bool,
}

impl KeySequence {
    pub fn new(seq: Vec<super::Action>) -> Self {
        Self {
            seq,
            cursor: 0,
            requested_stop: false,
        }
    }
    pub fn actions(&mut self) -> &mut Vec<super::Action> {
        &mut self.seq
    }
    pub fn cursor(&self) -> usize {
        self.cursor
    }
    pub fn requested_stop(&self) -> bool {
        self.requested_stop
    }

    pub fn run_one(&mut self) -> Result<(), crate::error::Error> {
        if self.requested_stop {
            // return Err(crate::error::Error::TestError(
            //     "Script has requested exec stop".to_string(),
            // ));

            return Ok(());
        }
        let current_action = self
            .seq
            .get(self.cursor)
            .ok_or(crate::error::Error::TestError(format!(
                "Could not query action at cursor {}",
                self.cursor
            )))?;

        match current_action {
            super::Action::Wait(d) => d.wait(),
            super::Action::KeyPress(key) => key.press(),
            super::Action::KeyRelease(key) => key.release(),
            super::Action::MouseMovement(mode, amount) => match mode {
                super::CursorMovementMode::Relative => {
                    inputbot::MouseCursor::move_rel(amount.0, amount.1)
                }
                super::CursorMovementMode::Absolute => {
                    inputbot::MouseCursor::move_abs(amount.0, amount.1)
                }
            },
            super::Action::ButtonPress(btn) => btn.press(),
            super::Action::ButtonRelease(btn) => btn.release(),
            super::Action::Scroll(dir, amount) => match dir {
                super::ScrollDirection::X => inputbot::MouseWheel::scroll_hor(*amount),
                super::ScrollDirection::Y => inputbot::MouseWheel::scroll_ver(*amount),
            },
            super::Action::Stop => self.requested_stop = true,
        }
        trace!("Succesfully ran action at cursor {}", self.cursor);

        if !self.requested_stop {
            self.cursor += 1;
        }

        Ok(())
    }
}
