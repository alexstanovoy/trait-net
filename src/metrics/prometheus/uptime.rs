use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    Gauge,
};
use std::{sync::Arc, time::Instant};

#[derive(Clone)]
pub struct Uptime(Arc<UptimeData>);

#[derive(Clone)]
struct UptimeData {
    gauge: Gauge,
    start_time: Instant,
}

impl Uptime {
    pub fn start<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
    ) -> prometheus::Result<Self> {
        Ok(Self(Arc::new(UptimeData {
            gauge: Gauge::new(name, help)?,
            start_time: Instant::now(),
        })))
    }
}

impl Collector for Uptime {
    fn desc(&self) -> Vec<&Desc> {
        self.0.gauge.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0
            .gauge
            .set(self.0.start_time.elapsed().as_millis() as f64);
        self.0.gauge.collect()
    }
}
