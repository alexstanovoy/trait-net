use std::{sync::{atomic::{Ordering, AtomicU64}, Arc}, time::Duration};
use super::{ScopedMetric, MetricsFutureExt, Observation};
use tokio::{spawn, time::sleep};

struct SimpleRate(Arc<AtomicU64>);
struct SimpleRateObservation(Arc<AtomicU64>);

impl ScopedMetric for SimpleRate {
    type Guard = SimpleRateObservation;

    fn observation(&self) -> Self::Guard {
        self.0.fetch_add(1, Ordering::Relaxed);
        SimpleRateObservation(self.0.clone())
    }
}

impl Observation for SimpleRateObservation {
    fn start(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl Drop for SimpleRateObservation {
    fn drop(&mut self) {
        println!("DROPPING");
        self.0.fetch_sub(1, Ordering::Relaxed);
    }
}

#[tokio::test]
async fn simple() {
    let metric = SimpleRate(Arc::new(AtomicU64::new(0)));
    let handle1 = spawn(sleep(Duration::from_millis(200)))
        .with_metric(&metric);

    assert_eq!(metric.0.load(Ordering::Relaxed), 1);
    let handle2 = spawn(sleep(Duration::from_millis(400)))
        .with_metric(&metric);
    assert_eq!(metric.0.load(Ordering::Relaxed), 2);
    handle1.await.unwrap();
    assert_eq!(metric.0.load(Ordering::Relaxed), 1);
    handle2.await.unwrap();
    assert_eq!(metric.0.load(Ordering::Relaxed), 0);
}

#[tokio::test]
async fn select_and_drop() {
    let metric = SimpleRate(Arc::new(AtomicU64::new(0)));
    tokio::select! {
        _ = spawn(sleep(Duration::from_millis(200))).with_metric(&metric) => unreachable!(),
        _ = spawn(sleep(Duration::from_millis(100))) => {},
    }
    // Drop is done asynchoronously, so we must wait for it
    spawn(sleep(Duration::from_millis(200))).await.unwrap();
    assert_eq!(metric.0.load(Ordering::Relaxed), 0);
}
