use crate::metrics::Observer;
use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    Histogram, HistogramOpts, HistogramVec,
};
use std::time::Instant;

#[derive(Clone)]
pub struct Latency(HistogramVec);

impl Latency {
    pub fn new(opts: HistogramOpts, label_names: &[&str]) -> prometheus::Result<Self> {
        Ok(Self(HistogramVec::new(opts, label_names)?))
    }

    pub fn observe(&self, labels: &[&str]) -> LatencyObserver {
        LatencyObserver {
            hist: self.0.with_label_values(labels),
            instant: None,
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
    instant: Option<Instant>,
}

impl<Out> Observer<Out> for LatencyObserver {
    fn start(&mut self) {
        self.instant = Some(Instant::now());
    }

    fn stop(&mut self) {
        self.instant
            .map(|i| self.hist.observe(i.elapsed().as_secs_f64()));
    }

    fn record(&mut self, output: &Out) {}
}
