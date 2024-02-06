use crate::metrics::Observer;
use prometheus::{
    core::{AtomicU64, Collector, Desc, GenericCounter},
    proto::MetricFamily,
    IntCounterVec, Opts,
};

#[derive(Clone)]
pub struct StartedCount(IntCounterVec);

impl StartedCount {
    pub fn new(opts: Opts, label_names: &[&str]) -> prometheus::Result<Self> {
        Ok(Self(IntCounterVec::new(opts, label_names)?))
    }

    pub fn observe(&self, labels: &[&str]) -> StartedCountObserver {
        StartedCountObserver(self.0.with_label_values(labels))
    }
}

impl Collector for StartedCount {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct StartedCountObserver(GenericCounter<AtomicU64>);

impl<Out> Observer<Out> for StartedCountObserver {
    fn on_first_poll(&mut self) {
        self.0.inc();
    }
}
