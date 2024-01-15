use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    Gauge, Opts,
};
use std::{sync::Arc, time::Instant};

#[derive(Clone)]
pub struct Elapsed(Arc<ElapsedData>);

#[derive(Clone)]
struct ElapsedData {
    gauge: Gauge,
    start_time: Instant,
}

impl Elapsed {
    pub fn new(opts: Opts) -> prometheus::Result<Self> {
        Ok(Self(Arc::new(ElapsedData {
            gauge: Gauge::with_opts(opts)?,
            start_time: Instant::now(),
        })))
    }
}

impl Collector for Elapsed {
    fn desc(&self) -> Vec<&Desc> {
        self.0.gauge.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0
            .gauge
            .set(self.0.start_time.elapsed().as_secs_f64() as f64);
        self.0.gauge.collect()
    }
}
