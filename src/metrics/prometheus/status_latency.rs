use crate::metrics::{AsStatusLabel, Observer};
use prometheus::{
    core::{Collector, Desc},
    proto::MetricFamily,
    HistogramOpts, HistogramVec,
};
use std::{iter::once_with, time::Instant};

#[derive(Clone)]
pub struct StatusLatency(HistogramVec);

impl StatusLatency {
    pub fn new(opts: HistogramOpts, label_names: &[&str]) -> prometheus::Result<Self> {
        Self::new_with(opts, label_names, "status")
    }

    pub fn new_with(
        opts: HistogramOpts,
        label_names: &[&str],
        status_label_name: &str,
    ) -> prometheus::Result<Self> {
        let labels: Vec<_> = label_names
            .into_iter()
            .map(|s| *s)
            .chain(once_with(|| status_label_name))
            .collect();
        Ok(Self(HistogramVec::new(opts, &labels)?))
    }

    pub fn observe(&self, labels: &[&str]) -> StatusLatencyObserver {
        StatusLatencyObserver {
            hist: self.0.clone(),
            instant: None,
            labels: labels.into_iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl Collector for StatusLatency {
    fn desc(&self) -> Vec<&Desc> {
        self.0.desc()
    }

    fn collect(&self) -> Vec<MetricFamily> {
        self.0.collect()
    }
}

pub struct StatusLatencyObserver {
    hist: HistogramVec,
    instant: Option<Instant>,
    labels: Vec<String>,
}

impl<Out: AsStatusLabel> Observer<Out> for StatusLatencyObserver {
    fn on_first_poll(&mut self) {
        self.instant = Some(Instant::now());
    }

    fn on_poll_ready(&mut self, output: &Out) {
        if let Some(instant) = self.instant {
            let status_label_name = output.as_status_label();
            let labels: Vec<_> = self
                .labels
                .iter()
                .map(|s| s.as_str())
                .chain(once_with(|| status_label_name.as_str()))
                .collect();
            self.hist
                .with_label_values(&labels)
                .observe(instant.elapsed().as_secs_f64());
        }
    }
}
