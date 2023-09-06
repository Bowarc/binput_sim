#[derive(PartialEq, Debug)]
pub enum RunnerMessage {
    SetSequence(crate::scripting::ActionSequence),
    SequenceSet,

    StartSequence,
    SequenceStarted,

    StopSequence,
    SequenceStopped,

    CleanSequence,
    SequenceDeleted,

    ResetCurrentSequenceCursor,
    SequenceCursorResetted,

    CrusorUpdate(usize), // pos
    Goodbye,
}

pub struct RunnerHandle {
    joinhandle: std::thread::JoinHandle<()>,
    thread_channel: crate::threading::Channel<RunnerMessage>,
    name: String,
    key_sequence_running: bool,
}

pub struct RunnerThread {
    channel: crate::threading::Channel<RunnerMessage>,
    current_sequence: Option<crate::scripting::ActionSequence>,
    name: String,
    sequence_running: bool,
    requested_stop: bool,
    last_cursor_update_sent: usize,
}

pub struct RunnerState {
    current_sequence: super::ActionSequence,
    running: bool,
}

impl RunnerHandle {
    pub fn new(id: String) -> Self {
        let (channel1, channel2) = crate::threading::Channel::<RunnerMessage>::new_pair();

        let name = format!("RunnerThread id: {id}");
        let n = name.clone();
        let handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                debug!("A runner with id '{n}' has been created");
                let mut thread = RunnerThread::new(channel2, n);
                thread.run()
            })
            .unwrap();

        Self {
            thread_channel: channel1,
            joinhandle: handle,
            name,
            key_sequence_running: false,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn send(
        &mut self,
        msg: RunnerMessage,
    ) -> Result<(), std::sync::mpsc::SendError<RunnerMessage>> {
        // if msg == RunnerMessage::StartSequence {
        //     self.key_sequence_running = true;
        // }

        self.thread_channel.send(msg)
    }
    pub fn try_recv(&mut self) -> Result<RunnerMessage, std::sync::mpsc::TryRecvError> {
        let res = self.thread_channel.try_recv();

        if let Ok(msg) = &res {
            match msg {
                RunnerMessage::SequenceSet => {
                    debug!("Runner has succesfully set the requested sequence");
                    // self.sequence_sync = true
                }
                RunnerMessage::SequenceStarted => {
                    debug!("Runner has started its sequence");
                    self.key_sequence_running = true;
                }
                RunnerMessage::SequenceStopped => {
                    debug!("Runner's sequence has stopped");
                    self.key_sequence_running = false;
                }
                RunnerMessage::SequenceDeleted => {
                    debug!("Runner has deleted its sequence");
                    // self.sequence_sync = false
                }
                RunnerMessage::SequenceCursorResetted => {
                    debug!("Runner has reset its sequence cursor");
                }
                RunnerMessage::Goodbye => {
                    debug!("Runner has exited");
                }
                _ => {}
            }
        }

        res
    }
    pub fn is_runner_running(&self) -> bool {
        self.key_sequence_running
    }
}

impl RunnerThread {
    pub fn new(channel: crate::threading::Channel<RunnerMessage>, name: String) -> Self {
        Self {
            channel,
            name,
            current_sequence: None,
            sequence_running: false,
            requested_stop: false,

            last_cursor_update_sent: 0,
        }
    }

    fn handle_channel(&mut self) {
        match self.channel.try_recv() {
            Ok(msg) => {
                trace!("Thread received a new message: {msg:?}");

                match msg {
                    RunnerMessage::SetSequence(seq) => self.set_sequence_without_running(seq),
                    RunnerMessage::CleanSequence => self.delete_current_sequence(),
                    RunnerMessage::StartSequence => {
                        if self.current_sequence.is_some() {
                            self.run_current_sequence().unwrap();
                        } else {
                            error!("Runner {} tried to start a None sequence", self.name)
                        }
                    }
                    RunnerMessage::StopSequence => self.stop_current_sequence(),
                    _ => warn!("Unhandled message: {msg:?}"),
                }
            }
            Err(e) if e == std::sync::mpsc::TryRecvError::Empty => {
                // println!("Would block");
            }
            Err(e) => {
                error!("Unknown runner error: {e:?}");
                self.requested_stop = true;
            }
        }
    }

    fn set_sequence_without_running(&mut self, seq: super::ActionSequence) {
        self.current_sequence = Some(seq);
        self.channel.send(RunnerMessage::SequenceSet).unwrap();
    }

    fn run_current_sequence(&mut self) -> Result<(), ()> {
        if self.current_sequence.is_some() {
            self.sequence_running = true;
            self.channel.send(RunnerMessage::SequenceStarted).unwrap();
            Ok(())
        } else {
            Err(())
        }
    }

    fn run_new_sequence(&mut self, seq: super::ActionSequence) {
        self.current_sequence = Some(seq);
        self.channel.send(RunnerMessage::SequenceSet).unwrap();
        self.sequence_running = true;
        self.channel.send(RunnerMessage::SequenceStarted).unwrap();
    }

    fn stop_current_sequence(&mut self) {
        self.sequence_running = false;
        self.channel.send(RunnerMessage::SequenceStopped).unwrap();
    }

    fn delete_current_sequence(&mut self) {
        self.current_sequence = None;
        self.channel.send(RunnerMessage::SequenceDeleted).unwrap();
        self.sequence_running = false;
        self.channel.send(RunnerMessage::SequenceStopped).unwrap();
    }

    fn run_sequence(&mut self) {
        let Some(seq) =  &mut self.current_sequence else{
            self.stop_current_sequence();
            return;
        };

        if seq.requested_stop() {
            self.stop_current_sequence();
            return;
        }

        if let Err(e) = seq.run_one() {
            error!(
                "Runner {} encountered the error: {e:?}\nWhile running sequence {seq:#?}",
                self.name,
            );
            self.stop_current_sequence();
        }
    }

    fn update_tab(&mut self) {
        let Some(seq) =  &self.current_sequence else{
            return;
        };

        let cursor = seq.cursor();

        if cursor == self.last_cursor_update_sent {
            return;
        }

        if let Err(e) = self.channel.send(RunnerMessage::CrusorUpdate(seq.cursor())) {
            error!("Encoutered an error while sending CrusorUpdate to main thread: {e:?}");
            self.requested_stop = true
        }
        self.last_cursor_update_sent = cursor;
    }

    fn run(&mut self) {
        while !self.requested_stop {
            self.handle_channel();

            if self.sequence_running {
                self.update_tab();
            }

            if self.sequence_running {
                self.run_sequence();
            }
        }

        self.exit()
    }

    fn exit(&mut self) {
        self.delete_current_sequence();
        let _ = self.channel.send(RunnerMessage::Goodbye);
        debug!("Exiting thread {}", self.name);
    }
}
