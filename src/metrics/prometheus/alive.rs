use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    IntCounter, Opts,
};

#[derive(Clone)]
pub struct Alive(IntCounter);

impl Alive {
    pub fn new(opts: Opts) -> prometheus::Result<Self> {
        let counter = IntCounter::with_opts(opts)?;
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
