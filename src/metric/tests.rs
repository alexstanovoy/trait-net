// use super::{Metric, MetricsFutureExt, Observer};
// use std::{
//     sync::{
//         atomic::{AtomicU64, Ordering},
//         Arc,
//     },
//     time::Duration,
// };
// use tokio::{spawn, time::sleep};

// struct SimpleRate(Arc<AtomicU64>);
// struct SimpleRateObservation(Arc<AtomicU64>);

// impl Metric for SimpleRate {
//     type ObservationGuard = SimpleRateObservation;

//     fn observe(&self, _labels: &[&str]) -> Self::ObservationGuard {
//         self.0.fetch_add(1, Ordering::Relaxed);
//         SimpleRateObservation(self.0.clone())
//     }
// }

// impl Observer for SimpleRateObservation {
//     fn watch(&self) {
//         self.0.fetch_add(1, Ordering::Relaxed);
//     }

//     fn record<Output>(&self, _output: &Output) {}
// }

// impl Drop for SimpleRateObservation {
//     fn drop(&mut self) {
//         self.0.fetch_sub(1, Ordering::Relaxed);
//     }
// }

// #[tokio::test]
// async fn simple() {
//     let metric = SimpleRate(Arc::new(AtomicU64::new(0)));

//     let obs = metric.observe(&[]);
//     let handle1 = spawn(sleep(Duration::from_millis(200))).monitor(obs);
//     assert_eq!(metric.0.load(Ordering::Relaxed), 1);

//     let obs = metric.observe(&[]);
//     let handle2 = spawn(sleep(Duration::from_millis(400))).monitor(obs);
//     assert_eq!(metric.0.load(Ordering::Relaxed), 2);
//     handle1.await.unwrap();
//     assert_eq!(metric.0.load(Ordering::Relaxed), 1);
//     handle2.await.unwrap();
//     assert_eq!(metric.0.load(Ordering::Relaxed), 0);
// }

// #[tokio::test]
// async fn select_and_drop() {
//     let metric = SimpleRate(Arc::new(AtomicU64::new(0)));
//     let obs = metric.observe(&[]);
//     tokio::select! {
//         _ = spawn(sleep(Duration::from_millis(200))).monitor(obs) => unreachable!(),
//         _ = spawn(sleep(Duration::from_millis(100))) => {},
//     }
//     // Drop is done asynchoronously, so we must wait for it
//     spawn(sleep(Duration::from_millis(200))).await.unwrap();
//     assert_eq!(metric.0.load(Ordering::Relaxed), 0);
// }
