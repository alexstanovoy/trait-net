use crate::metrics::ScopedObserver;
use prometheus::{
    core::{AtomicU64, Collector, Desc, GenericCounter},
    proto::MetricFamily,
    IntCounterVec, Opts,
};

#[derive(Clone)]
pub struct Rate {
    started: IntCounterVec,
    ended: IntCounterVec,
}

impl Rate {
    pub fn new(opts: Opts, label_names: &[&str]) -> prometheus::Result<Self> {
        let mut started_opts = opts.clone();
        started_opts.name.push_str("_started");
        let mut ended_opts = opts;
        ended_opts.name.push_str("_ended");
        Ok(Self {
            started: IntCounterVec::new(started_opts, label_names)?,
            ended: IntCounterVec::new(ended_opts, label_names)?,
        })
    }

    pub fn observe(&self, labels: &[&str]) -> RateObserver {
        RateObserver {
            started: self.started.with_label_values(labels),
            ended: self.ended.with_label_values(labels),
        }
    }
}

impl Collector for Rate {
    fn desc(&self) -> Vec<&Desc> {
        self.started.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        let mut v = self.started.collect();
        v.extend(self.ended.collect());
        v
    }
}

pub struct RateObserver {
    started: GenericCounter<AtomicU64>,
    ended: GenericCounter<AtomicU64>,
}

impl ScopedObserver for RateObserver {
    fn start(&self) {
        self.started.inc();
    }

    fn stop(&self) {
        self.ended.inc();
    }
}
