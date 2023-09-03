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
        self.t as f64 / 1_000_000.
    }

    pub fn as_std_duration(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.t)
    }

    pub fn wait(&self) {
        spin_sleep::sleep(std::time::Duration::from_nanos(self.t))
    }
}

impl std::fmt::Display for Delay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", display_duration(self.as_std_duration(), ""))
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
