#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Delay {
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

impl std::fmt::Display for Delay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_duration(self.as_std_duration(), ""))
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

pub fn display_duration(d: std::time::Duration, separator: &str) -> String {
    let mut value: f64 = d.as_nanos() as f64;
    // debug!("d:{:?}", d);
    // if nanos == 0 {}
    // debug!("nbr: {}", nbr);

    let units: &[&str] = &["ns", "µs", "ms", "s"];
    let mut name_index = 0;

    while value >= 1_000. {
        if name_index < units.len() - 1 {
            value /= 1_000.;
            name_index += 1
        } else {
            break;
        }
    }

    format!("{:.2}{}{}", value, separator, units[name_index])
}
