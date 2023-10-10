#[derive(Debug, Copy, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
}

#[derive(PartialEq, Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Delay {
    #[serde(skip_serializing, skip_deserializing)]
    waiting_instant: Option<std::time::Instant>,

    pub v: f64,
    pub unit: TimeUnit,
}

impl Delay {
    pub fn new(wait_time_s: f64) -> Self {
        // let t: spin_sleep::Nanoseconds = (wait_time_s * 1000_000_000.) as u64;

        Self {
            v: wait_time_s,
            unit: TimeUnit::Seconds,
            waiting_instant: None,
        }
    }
    pub fn as_millis(&self) -> f64 {
        self.unit.to_millis(self.v)
    }

    pub fn as_std_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.unit.to_nanos(self.v) as u64)
    }

    pub fn start_wait(&mut self) {
        self.waiting_instant = Some(std::time::Instant::now());
    }

    pub fn is_finished(&self) -> bool {
        if let Some(instant) = self.waiting_instant {
            instant.elapsed().as_nanos() >= self.unit.to_nanos(self.v) as u64 as u128
        } else {
            false
        }
    }

    pub fn wait(&self) {
        spin_sleep::sleep(std::time::Duration::from_nanos(
            self.unit.to_nanos(self.v) as u64
        ))
    }
}

impl From<(f64, TimeUnit)> for Delay {
    fn from(value: (f64, TimeUnit)) -> Self {
        Delay {
            waiting_instant: None,
            v: value.0,
            unit: value.1,
        }
    }
}

impl std::fmt::Display for Delay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_duration(self.as_std_duration()))
    }
}

impl TimeUnit {
    pub fn to_nanos(self, v: f64) -> f64 {
        match self {
            TimeUnit::Nanoseconds => v,
            TimeUnit::Microseconds => v * 1e+3,
            TimeUnit::Milliseconds => v * 1e+6,
            TimeUnit::Seconds => v * 1e+9,
        }
    }

    pub fn to_micros(self, v: f64) -> f64 {
        match self {
            TimeUnit::Nanoseconds => v / 1e+3,
            TimeUnit::Microseconds => v,
            TimeUnit::Milliseconds => v * 1e+3,
            TimeUnit::Seconds => v * 1e+6,
        }
    }

    pub fn to_millis(self, v: f64) -> f64 {
        match self {
            TimeUnit::Nanoseconds => v / 1e+6,
            TimeUnit::Microseconds => v / 1e+3,
            TimeUnit::Milliseconds => v,
            TimeUnit::Seconds => v * 1e+3,
        }
    }

    pub fn to_seconds(self, v: f64) -> f64 {
        match self {
            TimeUnit::Nanoseconds => v / 1e+9,
            TimeUnit::Microseconds => v / 1e+6,
            TimeUnit::Milliseconds => v / 1e+3,
            TimeUnit::Seconds => v,
        }
    }
}

impl std::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimeUnit::Nanoseconds => "ns",
                TimeUnit::Microseconds => "µs",
                TimeUnit::Milliseconds => "ms",
                TimeUnit::Seconds => "s",
            }
        )
    }
}

#[derive(Debug, Clone)]
/// Measure the time between the .start and .stop functions, can be read later
pub enum Stopwatch {
    // Ps i used an enum as it best fits the use to me, + it's globally smaller as it re-uses the memory if the other state for the curent one
    Running {
        start_time: std::time::Instant,
    },
    Paused {
        paused_since: std::time::Instant,
        runtime: std::time::Duration,
    },
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::Paused {
            paused_since: std::time::Instant::now(),
            runtime: std::time::Duration::from_secs(0),
        }
    }
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn start_new() -> Self {
        Self::Running {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn is_running(&self) -> bool {
        matches![self, Stopwatch::Running { .. }]
    }
    pub fn is_stopped(&self) -> bool {
        !self.is_running()
    }
    pub fn start(&mut self) {
        *self = Stopwatch::start_new();
    }
    pub fn stop(&mut self) {
        if let Self::Running { start_time } = self {
            *self = Stopwatch::Paused {
                paused_since: std::time::Instant::now(),
                runtime: start_time.elapsed(),
            }
        }
    }
    pub fn read(&self) -> std::time::Duration {
        match self {
            Stopwatch::Running { start_time } => start_time.elapsed(),
            Stopwatch::Paused { runtime, .. } => *runtime,
        }
    }
}

impl std::fmt::Display for Stopwatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_duration(self.read()))
    }
}

pub fn display_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos();

    if secs == 0 {
        if nanos < 1_000 {
            return format!("{}ns", nanos);
        } else if nanos < 1_000_000 {
            return format!("{:.2}µs", nanos as f64 / 1_000.0);
        } else {
            return format!("{:.2}ms", nanos as f64 / 1_000_000.0);
        }
    }

    if secs < 60 {
        format!("{secs}s")
    } else if secs < 3_600 {
        let minutes = secs / 60;
        let seconds = secs % 60;
        format!("{minutes}m {seconds}s")
    } else if secs < 86_400 {
        let hours = secs / 3_600;
        let minutes = (secs % 3_600) / 60;
        format!("{hours}h {minutes}m")
    } else {
        let days = secs / 86_400;
        format!("{days}days")
    }
}
