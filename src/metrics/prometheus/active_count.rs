use crate::metrics::Observer;
use prometheus::{
    core::{AtomicI64, Collector, Desc, GenericGauge},
    proto::MetricFamily,
    IntGaugeVec, Opts,
};

#[derive(Clone)]
pub struct ActiveCount(IntGaugeVec);

impl ActiveCount {
    pub fn new(opts: Opts, label_names: &[&str]) -> prometheus::Result<Self> {
        Ok(Self(IntGaugeVec::new(opts, label_names)?))
    }

    pub fn observe(&self, labels: &[&str]) -> ActiveCountObserver {
        ActiveCountObserver(self.0.with_label_values(labels))
    }
}

impl Collector for ActiveCount {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct ActiveCountObserver(GenericGauge<AtomicI64>);

impl<Out> Observer<Out> for ActiveCountObserver {
    fn on_first_poll(&mut self) {
        self.0.inc();
    }

    fn on_poll_ready(&mut self, _: &Out) {
        self.0.dec();
    }
}
