use crate::metrics::Observer;
use prometheus::{
    core::{AtomicU64, Collector, Desc, GenericCounter},
    proto::MetricFamily,
    IntCounterVec, Opts,
};

#[derive(Clone)]
pub struct Rate(IntCounterVec);

impl Rate {
    pub fn new(opts: Opts, label_names: &[&str]) -> prometheus::Result<Self> {
        Ok(Self(IntCounterVec::new(opts, label_names)?))
    }

    pub fn observe(&self, labels: &[&str]) -> RateObserver {
        RateObserver(self.0.with_label_values(labels))
    }
}

impl Collector for Rate {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct RateObserver(GenericCounter<AtomicU64>);

impl<Out> Observer<Out> for RateObserver {
    fn start(&mut self) {
        self.0.inc();
    }

    fn stop(&mut self) {
        self.0.inc();
    }

    fn record(&mut self, _: &Out) {}
}
