use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    IntCounter, Opts,
};

#[derive(Clone)]
pub struct Constant(IntCounter);

impl Constant {
    pub fn new(opts: Opts, constant: u64) -> prometheus::Result<Self> {
        let counter = IntCounter::with_opts(opts)?;
        counter.inc_by(constant);
        Ok(Self(counter))
    }
}

impl Collector for Constant {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}
