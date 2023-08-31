#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Delay {
    pub t: spin_sleep::Nanoseconds,
}

impl Delay {
    pub fn new(wait_time_s: f64) -> Self {
        let t: spin_sleep::Nanoseconds = (wait_time_s * 1_000_000_000.) as u64;

        Self { t }
    }
    pub fn as_millis(&self) -> f64 {
        self.t as f64 / 10_000_000.
    }

    pub fn wait(&self) {
        spin_sleep::sleep(std::time::Duration::from_nanos(self.t))
    }
}
