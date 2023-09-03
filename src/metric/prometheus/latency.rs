use crate::metric::Observer;
use prometheus::{Histogram, HistogramOpts, HistogramVec};
use std::{ops::Deref, time::Instant};

pub struct Latency(HistogramVec);

impl Deref for Latency {
    type Target = HistogramVec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Latency {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        label_names: &[&str],
    ) -> prometheus::Result<Self> {
        Ok(Self(HistogramVec::new(
            HistogramOpts::new(name, help),
            label_names,
        )?))
    }

    pub fn observe(&self, labels: &[&str]) -> LatencyObserver {
        LatencyObserver {
            hist: self.0.with_label_values(labels),
            timer: None,
        }
    }
}

pub struct LatencyObserver {
    hist: Histogram,
    timer: Option<Instant>,
}

impl Observer for LatencyObserver {
    fn start(&mut self) {
        self.timer = Some(Instant::now());
    }

    fn stop(&mut self) {
        self.hist.observe(
            self.timer
                .expect("start wasn't called before stop")
                .elapsed()
                .as_millis() as f64,
        )
    }

    fn record<Output>(&mut self, _output: &Output) {}
}
