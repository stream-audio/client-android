use crate::error::Error;
use std::collections::VecDeque;
use std::default::Default;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct WindowAvgCalc {
    acc: Duration,
    window_size: usize,
    window: VecDeque<Duration>,
}

impl WindowAvgCalc {
    pub fn new(window_size: usize) -> Result<Self, Error> {
        if window_size == 0 {
            Err(Error::new_wrong_argument(
                "Window size must be greater than 0",
            ))
        } else {
            Ok(Self {
                acc: Default::default(),
                window_size,
                window: VecDeque::with_capacity(window_size),
            })
        }
    }

    pub fn push(&mut self, num: Duration) {
        if self.window.len() >= self.window_size {
            self.acc -= self.window.pop_front().unwrap();
        }

        self.window.push_back(num);
        self.acc += num;
    }

    pub fn get_avg(&self) -> Duration {
        if !self.window.is_empty() {
            self.acc / self.window.len() as u32
        } else {
            Default::default()
        }
    }
}
