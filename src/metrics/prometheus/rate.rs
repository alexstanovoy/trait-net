use crate::metrics::ScopedObserver;
use prometheus::{
    core::{AtomicI64, Collector, Desc, GenericGauge},
    proto::MetricFamily,
    IntGaugeVec, Opts,
};

#[derive(Clone)]
pub struct Rate(IntGaugeVec);

impl Rate {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        label_names: &[&str],
    ) -> prometheus::Result<Self> {
        Ok(Self(IntGaugeVec::new(Opts::new(name, help), label_names)?))
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

pub struct RateObserver(GenericGauge<AtomicI64>);

impl ScopedObserver for RateObserver {
    fn start(&self) {
        self.0.inc();
    }

    fn stop(&self) {
        self.0.dec();
    }
}
