#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
// #[serde(from = "Vec<super::Action>")]
pub struct ActionSequence {
    seq: Vec<super::Action>,
    #[serde(skip_serializing, skip_deserializing)]
    cursor: usize,
    #[serde(skip_serializing, skip_deserializing)]
    requested_stop: bool,
    #[serde(skip_serializing, skip_deserializing)]
    currently_waiting: bool,
}

impl ActionSequence {
    pub fn new(seq: Vec<super::Action>) -> Self {
        Self {
            seq,
            cursor: 0,
            requested_stop: false,
            currently_waiting: false,
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

        let current_action =
            self.seq
                .get_mut(self.cursor)
                .ok_or(crate::error::Error::TestError(format!(
                    "Could not query action at cursor {}",
                    self.cursor
                )))?;

        if self.currently_waiting {
            if let super::Action::Wait(d) = current_action {
                if d.is_finished() {
                    self.currently_waiting = false;
                } else {
                    return Ok(());
                }
            } else {
                error!("The sequence is currently waiting but the latest action is not a Wait");
                self.currently_waiting = false;
            }
        } else {
            match current_action {
                super::Action::Wait(d) => {
                    d.start_wait();
                    self.currently_waiting = true;
                    return Ok(());
                }
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
                super::Action::Stop => {
                    super::utils::release_all_kbkeys();
                    super::utils::release_all_mouse_btns();
                    self.requested_stop = true
                }
                super::Action::KeySequence(s) => inputbot::KeySequence(s).send(),
            }
        }

        trace!("Succesfully ran action at cursor {}", self.cursor);

        if !self.requested_stop {
            self.cursor += 1;
        }

        Ok(())
    }
}

impl From<Vec<super::Action>> for ActionSequence {
    fn from(value: Vec<super::Action>) -> Self {
        ActionSequence::new(value)
    }
}
