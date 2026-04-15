use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TickRate {
    Hz(u32),
    Interval(Duration),
}

impl TickRate {
    pub fn interval(self) -> Duration {
        match self {
            Self::Hz(hz) => {
                let hz = hz.max(1);
                Duration::from_nanos(1_000_000_000 / u64::from(hz))
            }
            Self::Interval(d) => d,
        }
    }
}

pub trait Clock: Send {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
    fn wait_until_next_tick(&mut self, rate: Option<TickRate>);
}

pub struct SystemClock {
    last_tick: Option<Instant>,
}

impl SystemClock {
    pub fn new() -> Self {
        Self { last_tick: None }
    }
}

impl Default for SystemClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for SystemClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now()
    }

    fn wait_until_next_tick(&mut self, rate: Option<TickRate>) {
        let Some(rate) = rate else {
            self.last_tick = Some(Instant::now());
            return;
        };
        let target = rate.interval();
        let now = Instant::now();
        if let Some(last) = self.last_tick {
            let elapsed = now.saturating_duration_since(last);
            if elapsed < target {
                std::thread::sleep(target - elapsed);
            }
        }
        self.last_tick = Some(Instant::now());
    }
}

pub struct ManualClock {
    now: chrono::DateTime<chrono::Utc>,
    pending_advance: Duration,
}

impl ManualClock {
    pub fn new(start: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            now: start,
            pending_advance: Duration::ZERO,
        }
    }

    pub fn advance(&mut self, by: Duration) {
        self.pending_advance += by;
    }
}

impl Default for ManualClock {
    fn default() -> Self {
        Self::new(chrono::DateTime::UNIX_EPOCH)
    }
}

impl Clock for ManualClock {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        self.now
    }

    fn wait_until_next_tick(&mut self, rate: Option<TickRate>) {
        let step = match rate {
            Some(r) => r.interval(),
            None => Duration::ZERO,
        };
        let total = step + std::mem::take(&mut self.pending_advance);
        if let Ok(delta) = chrono::Duration::from_std(total) {
            self.now += delta;
        }
    }
}
