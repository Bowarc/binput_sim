mod action;
pub mod runner;

pub use action::*;

#[derive(Debug, PartialEq, Clone)]
pub struct KeySequence {
    seq: Vec<Action>,
    cursor: usize,
    requested_stop: bool,
}

impl KeySequence {
    pub fn new(seq: Vec<Action>) -> Self {
        Self {
            seq,
            cursor: 0,
            requested_stop: false,
        }
    }
    pub fn actions(&mut self) -> &mut Vec<Action> {
        &mut self.seq
    }
    pub fn cursor(&self) -> usize {
        self.cursor
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
            Action::Wait(d) => d.wait(),
            Action::KeyPress(key) => key.press(),
            Action::KeyRelease(key) => key.release(),
            Action::MouseMovement(mode, amount) => match mode {
                CursorMovementMode::Relative => inputbot::MouseCursor::move_rel(amount.0, amount.1),
                CursorMovementMode::Absolute => inputbot::MouseCursor::move_abs(amount.0, amount.1),
            },
            Action::ButtonPress(btn) => btn.press(),
            Action::ButtonRelease(btn) => btn.release(),
            Action::Scroll(dir, amount) => match dir {
                ScrollDirection::X => inputbot::MouseWheel::scroll_hor(*amount),
                ScrollDirection::Y => inputbot::MouseWheel::scroll_ver(*amount),
            },
            Action::Stop => self.requested_stop = true,
        }
        trace!("Succesfully ran action at cursor {}", self.cursor);

        if !self.requested_stop {
            self.cursor += 1;
        }

        Ok(())
    }
}
