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
}

pub struct RunnerThread {
    channel: crate::threading::Channel<RunnerMessage>,
    current_sequence: Option<crate::scripting::KeySequence>,
    name: String,
    running: bool,
}

impl RunnerHandle {
    pub fn new() -> Self {
        let (channel1, channel2) = crate::threading::Channel::<RunnerMessage>::new_pair();

        let name = String::from("RunnerThread id: ?");
        let n = name.clone();
        let handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                let mut thread = RunnerThread::new(channel2, n);
                thread.run()
            })
            .unwrap();

        Self {
            thread_channel: channel1,
            joinhandle: handle,
            name,
        }
    }
}

impl RunnerThread {
    pub fn new(channel: crate::threading::Channel<RunnerMessage>, name: String) -> Self {
        Self {
            channel,
            name,
            current_sequence: None,
            running: true,
        }
    }

    fn handle_channel(&mut self) {
        match self.channel.try_recv() {
            Ok(msg) => {
                println!("Thread received a new message: {msg:?}");

                match msg {
                    RunnerMessage::NewSequence(seq) => self.current_sequence = Some(seq),
                    RunnerMessage::CleanSequence => self.current_sequence = None,

                    _ => println!("Unhandled message: {msg:?}"),
                }
            }
            Err(e) if e == std::sync::mpsc::TryRecvError::Empty => {
                // println!("Would block");
            }
            Err(e) => {
                println!("Unknown error: {e:?}");
                self.running = false;
            }
        }
    }

    fn run_sequence(&mut self) {
        let Some(seq) =  &mut self.current_sequence else{
            return;
        };
        if let Err(e) = self.channel.send(RunnerMessage::CrusorUpdate(seq.cursor())) {
            println!("Encoutered an error while sending CrusorUpdate to main thread: {e:?}");
            self.running = false
        }

        if let Err(e) = seq.run_one() {
            println!(
                "Runner {} encountered the error: {e:?}\nWhile running sequence {seq:#?}",
                self.name,
            );
            self.running = false;
        }
    }

    fn run(&mut self) {
        while self.running {
            self.handle_channel();
            self.run_sequence();
        }

        self.exit()
    }

    fn exit(&mut self) {
        let _ = self.channel.send(RunnerMessage::Goodbye);
        println!("Exiting thread {}", self.name);
    }
}
