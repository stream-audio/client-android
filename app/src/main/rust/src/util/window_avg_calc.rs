use crate::error::Error;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct WindowAvgCalc {
    acc: f64,
    window_size: usize,
    window: VecDeque<f64>,
}

impl WindowAvgCalc {
    pub fn new(window_size: usize) -> Result<Self, Error> {
        if window_size == 0 {
            Err(Error::new_wrong_argument(
                "Window size must be greater than 0",
            ))
        } else {
            Ok(Self {
                acc: 0.,
                window_size,
                window: VecDeque::with_capacity(window_size),
            })
        }
    }

    pub fn push(&mut self, num: f64) {
        if self.window.len() >= self.window_size {
            self.acc -= self.window.pop_front().unwrap();
        }

        self.window.push_back(num);
        self.acc += num;
    }

    pub fn get_avg(&self) -> f64 {
        if !self.window.is_empty() {
            self.acc / self.window.len() as f64
        } else {
            0.
        }
    }
}
