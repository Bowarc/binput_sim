#[derive(PartialEq, Debug)]
pub enum RunnerMessage {
    NewSequence(crate::scripting::KeySequence),
    CleanSequence,
    CrusorUpdate(usize), // pos
    Goodbye,
}

pub struct RunnerHandle {
    pub joinhandle: std::thread::JoinHandle<()>,
    pub thread_channel: crate::threading::Channel<RunnerMessage>,
    pub name: String,
    pub key_sequence_running: bool,
}

pub struct RunnerThread {
    channel: crate::threading::Channel<RunnerMessage>,
    current_sequence: Option<crate::scripting::KeySequence>,
    name: String,
    sequence_running: bool,
    requested_stop: bool,
}

impl RunnerHandle {
    pub fn new(id: String) -> Self {
        let (channel1, channel2) = crate::threading::Channel::<RunnerMessage>::new_pair();

        let name = format!("RunnerThread id: {id}");
        let n = name.clone();
        let handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                println!("A runner with id '{n}' has been created");
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
}

impl RunnerThread {
    pub fn new(channel: crate::threading::Channel<RunnerMessage>, name: String) -> Self {
        Self {
            channel,
            name,
            current_sequence: None,
            sequence_running: false,
            requested_stop: false,
        }
    }

    fn handle_channel(&mut self) {
        match self.channel.try_recv() {
            Ok(msg) => {
                println!("Thread received a new message: {msg:?}");

                match msg {
                    RunnerMessage::NewSequence(seq) => self.run_new_sequence(seq),
                    RunnerMessage::CleanSequence => self.delete_current_sequence(),

                    _ => println!("Unhandled message: {msg:?}"),
                }
            }
            Err(e) if e == std::sync::mpsc::TryRecvError::Empty => {
                // println!("Would block");
            }
            Err(e) => {
                println!("Unknown error: {e:?}");
                self.requested_stop = true;
            }
        }
    }

    fn set_sequence_without_running(&mut self, seq: super::KeySequence) {
        self.current_sequence = Some(seq);
    }

    fn run_new_sequence(&mut self, seq: super::KeySequence) {
        self.current_sequence = Some(seq);
        self.sequence_running = true;
    }

    fn stop_current_sequence(&mut self) {
        self.sequence_running = false;
    }

    fn delete_current_sequence(&mut self) {
        self.current_sequence = None;
        self.sequence_running = false;
    }

    fn run_sequence(&mut self) {
        let Some(seq) =  &mut self.current_sequence else{
            self.stop_current_sequence();
            return;
        };

        if seq.requested_stop {
            self.stop_current_sequence();
            return;
        }

        if let Err(e) = seq.run_one() {
            println!(
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

        if let Err(e) = self.channel.send(RunnerMessage::CrusorUpdate(seq.cursor())) {
            println!("Encoutered an error while sending CrusorUpdate to main thread: {e:?}");
            self.requested_stop = true
        }
    }

    fn run(&mut self) {
        while !self.requested_stop {
            self.handle_channel();

            if self.sequence_running {
                self.run_sequence();
            }

            if self.sequence_running {
                self.update_tab();
            }
        }

        self.exit()
    }

    fn exit(&mut self) {
        self.delete_current_sequence();
        let _ = self.channel.send(RunnerMessage::Goodbye);
        println!("Exiting thread {}", self.name);
    }
}
