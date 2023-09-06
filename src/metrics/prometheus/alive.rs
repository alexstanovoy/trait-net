use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    IntCounter,
};

#[derive(Clone)]
pub struct Alive(IntCounter);

impl Alive {
    pub fn start<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
    ) -> prometheus::Result<Self> {
        let counter = IntCounter::new(name, help)?;
        counter.inc();
        Ok(Self(counter))
    }
}

impl Collector for Alive {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}
