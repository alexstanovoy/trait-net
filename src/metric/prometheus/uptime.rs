use prometheus::{core::{Collector, Desc}, proto::MetricFamily, Gauge};
use std::{time::Instant, ops::Deref};

pub struct Uptime {
    gauge: Gauge,
    start_time: Instant,
}

impl Deref for Uptime {
    type Target = Gauge;

    fn deref(&self) -> &Self::Target {
        &self.gauge
    }
}

impl Uptime {
    pub fn start<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
    ) -> prometheus::Result<Self> {
        Ok(Self {
            gauge: Gauge::new(name, help)?,
            start_time: Instant::now(),
        })
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }
}

impl Collector for Uptime {
    fn desc(&self) -> Vec<&Desc> {
        self.gauge.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.gauge.set(self.start_time.elapsed().as_millis() as f64);
        self.gauge.collect()
    }
}
