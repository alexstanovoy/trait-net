use crate::metrics::ScopedObserver;
use prometheus::{
    core::{AtomicF64, Collector, Desc, GenericGauge},
    proto::MetricFamily,
    GaugeVec, Opts,
};
use std::{cell::OnceCell, time::Instant};

#[derive(Clone)]
pub struct Latency(GaugeVec);

impl Latency {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        label_names: &[&str],
    ) -> prometheus::Result<Self> {
        Ok(Self(GaugeVec::new(Opts::new(name, help), label_names)?))
    }

    pub fn observe(&self, labels: &[&str]) -> LatencyObserver {
        LatencyObserver {
            gauge: self.0.with_label_values(labels),
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
    gauge: GenericGauge<AtomicF64>,
    timer: OnceCell<Instant>,
}

impl ScopedObserver for LatencyObserver {
    fn start(&self) {
        self.timer.get_or_init(|| Instant::now());
    }

    fn stop(&self) {
        self.timer
            .get()
            .map(|t| self.gauge.add(t.elapsed().as_secs_f64()));
    }
}
