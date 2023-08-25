pub struct Delay {
    pub t: spin_sleep::Nanoseconds,
}

impl Delay {
    pub fn new(wait_time_ms: f64) -> Self {
        let t: spin_sleep::Nanoseconds = (wait_time_ms * 10_000_000.) as u64;

        Self { t }
    }

    pub fn wait(&self) {
        spin_sleep::sleep(std::time::Duration::from_nanos(self.t))
    }
}
