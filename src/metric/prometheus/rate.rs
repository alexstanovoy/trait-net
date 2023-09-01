use crate::metric::{ScopedMetric, Observation};
use prometheus::{core::{AtomicI64, GenericGauge}, IntGaugeVec, Opts};

pub struct Rate(IntGaugeVec);

impl Rate {
    pub fn new(name: &str, help: &str, label_names: &[&str]) -> prometheus::Result<Self> {
        Ok(Self(IntGaugeVec::new(Opts::new(name, help), label_names)?))
    }
}

impl ScopedMetric for Rate {
    type Guard = RateObserver;

    fn observation(&self, labels: &[&str]) -> Self::Guard {
        RateObserver(self.0.with_label_values(labels))
    }
}

pub struct RateObserver(GenericGauge<AtomicI64>);

impl Observation for RateObserver {
    fn start(&self) {
        self.0.inc()
    }
}

impl Drop for RateObserver {
    fn drop(&mut self) {
        self.0.dec()
    }
}
