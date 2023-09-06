use crate::metrics::{AsStatusLabel, StatusObserver};
use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    IntGaugeVec, Opts,
};
use std::iter::once_with;

#[derive(Clone)]
pub struct Status(IntGaugeVec);

impl Status {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
        label_names: &[&str],
        status_label_name: &str,
    ) -> prometheus::Result<Self> {
        let labels: Vec<_> = label_names
            .into_iter()
            .map(|s| *s)
            .chain(once_with(|| status_label_name))
            .collect();
        Ok(Self(IntGaugeVec::new(Opts::new(name, help), &labels)?))
    }

    pub fn observe(&self, labels: &[&str]) -> StatusObserverGuard {
        StatusObserverGuard {
            gauge: self.0.clone(),
            labels: labels.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Collector for Status {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct StatusObserverGuard {
    gauge: IntGaugeVec,
    labels: Vec<String>,
}

impl StatusObserver for StatusObserverGuard {
    fn record<Output: AsStatusLabel>(&self, output: &Output) {
        let status_label_name = output.as_status_label();
        let labels: Vec<_> = self
            .labels
            .iter()
            .map(|s| s.as_str())
            .chain(once_with(|| status_label_name.as_str()))
            .collect();
        self.gauge.with_label_values(&labels).inc();
    }
}
