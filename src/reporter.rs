use crate::num_format::NumFormat;
use std::time::Instant;

pub struct Reporter {
    thread_id: usize,
    pub index: usize,
    reported_index: usize,
    first: Instant,
    last: Instant,
    report_interval_seconds: f64,
    fixed_interval: Option<usize>,
    format: NumFormat,
}

impl Reporter {
    pub fn new(thread_id: usize, report_interval_seconds: f64, format: NumFormat) -> Self {
        let instant = Instant::now();
        Self {
            thread_id,
            index: 0,
            reported_index: 0,
            report_interval_seconds,
            first: instant,
            last: instant,
            fixed_interval: None,
            format,
        }
    }

    pub fn total_elapsed(&self) -> f64 {
        self.first.elapsed().as_millis() as f64
    }

    pub fn record_step(&mut self) {
        self.index += 1;
        if self.should_report() {
            let ms = self.last.elapsed().as_millis() as f64 + 1.0;
            let letter = self.format.letter();
            let delta = (self.index - self.reported_index) as f64;
            println!(
                "Thread #{}: iteration {:.2}{letter} ({:.2} {letter}/s)\r",
                self.thread_id,
                (self.index as f64) / self.format.factor(),
                (delta * 1000.0 / (self.format.factor() * ms))
            );
            self.last = Instant::now();
            self.reported_index = self.index;
            self.fixed_interval =
                Some((delta * self.report_interval_seconds * 1000.0 / ms) as usize);
        }
    }

    pub fn should_report(&self) -> bool {
        if let Some(interval) = self.fixed_interval {
            self.index - self.reported_index >= interval
        } else {
            (self.last.elapsed().as_millis() as f64) / 1000.0 >= self.report_interval_seconds
        }
    }
}
