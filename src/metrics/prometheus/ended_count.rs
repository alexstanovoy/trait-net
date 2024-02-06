use crate::metrics::Observer;
use prometheus::{
    core::{AtomicU64, Collector, Desc, GenericCounter},
    proto::MetricFamily,
    IntCounterVec, Opts,
};

#[derive(Clone)]
pub struct EndedCount(IntCounterVec);

impl EndedCount {
    pub fn new(opts: Opts, label_names: &[&str]) -> prometheus::Result<Self> {
        Ok(Self(IntCounterVec::new(opts, label_names)?))
    }

    pub fn observe(&self, labels: &[&str]) -> EndedCountObserver {
        EndedCountObserver(self.0.with_label_values(labels))
    }
}

impl Collector for EndedCount {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct EndedCountObserver(GenericCounter<AtomicU64>);

impl<Out> Observer<Out> for EndedCountObserver {
    fn on_poll_ready(&mut self, _: &Out) {
        self.0.inc();
    }
}
