use crate::metrics::{AsStatusLabel, Observer};
use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    IntGaugeVec, Opts,
};
use std::iter::once_with;

#[derive(Clone)]
pub struct Status(IntGaugeVec);

impl Status {
    pub fn new(opts: Opts, label_names: &[&str]) -> prometheus::Result<Self> {
        Self::new_with(opts, label_names, "status")
    }

    pub fn new_with(
        opts: Opts,
        label_names: &[&str],
        status_label_name: &str,
    ) -> prometheus::Result<Self> {
        let labels: Vec<_> = label_names
            .into_iter()
            .map(|s| *s)
            .chain(once_with(|| status_label_name))
            .collect();
        Ok(Self(IntGaugeVec::new(opts, &labels)?))
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

impl<Out: AsStatusLabel> Observer<Out> for StatusObserverGuard {
    fn on_poll_ready(&mut self, output: &Out) {
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
