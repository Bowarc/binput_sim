pub struct Channel<T> {
    sender: std::sync::mpsc::Sender<T>,
    receiver: std::sync::mpsc::Receiver<T>,
}

impl<T: std::cmp::PartialEq> Channel<T> {
    pub fn new_pair() -> (Channel<T>, Channel<T>) {
        let (sender1, receiver1) = std::sync::mpsc::channel::<T>();
        let (sender2, receiver2) = std::sync::mpsc::channel::<T>();

        let com1 = Channel {
            sender: sender1,
            receiver: receiver2,
        };
        let com2 = Channel {
            sender: sender2,
            receiver: receiver1,
        };
        (com1, com2)
    }

    pub fn wait_for(&self, waited_message: T) {
        loop {
            let message = self.receiver.recv().unwrap();
            if message == waited_message {
                break;
            }
        }
    }
    pub fn wait_for_or_timeout(
        &self,
        waited_message: T,
        timeout: std::time::Duration,
    ) -> Result<(), std::sync::mpsc::RecvTimeoutError> {
        let start_time = std::time::Instant::now();

        let internal_timeout = timeout / 100;
        while start_time.elapsed() < timeout {
            // we map the internal_timeout to be very small to be able to quit as soon as the timeout is done
            // + having a dynamic internal_timeout is adding to the consistency
            match self.recv_timeout(internal_timeout) {
                Ok(message) => {
                    if message == waited_message {
                        return Ok(());
                    }
                }
                Err(err) => return Err(err),
            }
        }
        Err(std::sync::mpsc::RecvTimeoutError::Timeout)
    }
    pub fn send(&self, t: T) -> Result<(), std::sync::mpsc::SendError<T>> {
        self.sender.send(t)
    }
    pub fn iter(&self) -> std::sync::mpsc::Iter<'_, T> {
        self.receiver.iter()
    }
    pub fn try_iter(&self) -> std::sync::mpsc::TryIter<'_, T> {
        self.receiver.try_iter()
    }
    pub fn recv(&self) -> Result<T, std::sync::mpsc::RecvError> {
        self.receiver.recv()
    }
    pub fn try_recv(&self) -> Result<T, std::sync::mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<T, std::sync::mpsc::RecvTimeoutError> {
        self.receiver.recv_timeout(timeout)
    }
}
