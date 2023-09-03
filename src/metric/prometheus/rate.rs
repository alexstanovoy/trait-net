use crate::metric::Observer;
use std::ops::Deref;
use prometheus::{
    core::{AtomicI64, GenericGauge},
    IntGaugeVec, Opts,
};

pub struct Rate(IntGaugeVec);

impl Deref for Rate {
    type Target = IntGaugeVec;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rate {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        label_names: &[&str],
    ) -> prometheus::Result<Self> {
        Ok(Self(IntGaugeVec::new(Opts::new(name, help), label_names)?))
    }

    pub fn observe(&mut self, labels: &[&str]) -> RateObserver {
        RateObserver(self.with_label_values(labels))
    }
}

pub struct RateObserver(GenericGauge<AtomicI64>);

impl Observer for RateObserver {
    fn start(&mut self) {
        self.0.inc();
    }

    fn stop(&mut self) {
        self.0.dec();
    }

    fn record<Output>(&mut self, _output: &Output) {}
}
