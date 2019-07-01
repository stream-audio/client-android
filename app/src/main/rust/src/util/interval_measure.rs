use std::time::{Duration, Instant};

#[allow(dead_code)]
#[derive(Debug)]
pub struct IntervalMeasure {
    prev_time: Option<Instant>,
    min_interval: Duration,
    max_interval: Duration,
    avg: AvgIntervalCalc,
}

#[allow(dead_code)]
impl IntervalMeasure {
    pub fn new() -> Self {
        Self {
            prev_time: None,
            min_interval: Duration::from_secs(10000),
            max_interval: Duration::from_secs(0),
            avg: AvgIntervalCalc::new(),
        }
    }

    pub fn new_event(&mut self) -> bool {
        let mut is_changed = false;
        if let Some(prev_time) = &self.prev_time {
            let duration = prev_time.elapsed();
            if duration < self.min_interval {
                self.min_interval = duration;
                is_changed = true;
            }
            if duration > self.max_interval {
                self.max_interval = duration;
                is_changed = true;
            }
            self.avg.add(duration);
        }
        self.prev_time = Some(Instant::now());

        is_changed
    }

    pub fn get_avg_interval(&self) -> Duration {
        self.avg.get_avg()
    }
}

#[derive(Debug)]
struct AvgIntervalCalc {
    acc: Duration,
    qty: u32,
}

impl std::fmt::Display for IntervalMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "min: {} ms., max: {} ms. avg: {} ms.",
            self.min_interval.as_micros() as f64 / 1000.,
            self.max_interval.as_millis(),
            self.get_avg_interval().as_millis()
        )
    }
}

impl AvgIntervalCalc {
    fn new() -> Self {
        Self {
            acc: Duration::from_secs(0),
            qty: 0,
        }
    }

    pub fn add(&mut self, interval: Duration) {
        self.acc += interval;
        self.qty += 1;
    }

    pub fn get_avg(&self) -> Duration {
        if self.qty > 0 {
            self.acc / self.qty
        } else {
            Duration::from_secs(0)
        }
    }
}
