use crate::metrics::ScopedObserver;
use prometheus::{Histogram, HistogramOpts, HistogramVec, core::{Collector, Desc}, proto::MetricFamily};
use std::{cell::OnceCell, time::Instant};

#[derive(Clone)]
pub struct Latency(HistogramVec);

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
            timer: OnceCell::new(),
        }
    }
}

impl Collector for Latency {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct LatencyObserver {
    hist: Histogram,
    timer: OnceCell<Instant>,
}

impl ScopedObserver for LatencyObserver {
    fn start(&self) {
        self.timer.get_or_init(|| Instant::now());
    }

    fn stop(&self) {
        self.timer
            .get()
            .map(|t| self.hist.observe(t.elapsed().as_secs_f64()));
    }
}
