use super::{Alive, Rate, Uptime};
use crate::metrics::MetricsFutureExt;
use prometheus::{core::Collector, opts};
use std::time::Duration;
use tokio::time::sleep;

#[test]
fn alive() {
    let alive = Alive::new(opts!("alive", "Total uptime")).unwrap();
    assert_eq!(
        alive.collect()[0].get_metric()[0].get_counter().get_value(),
        1.0
    );
}

#[tokio::test]
async fn uptime() {
    let uptime = Uptime::new(opts!("alive", "Total uptime")).unwrap();
    assert!((0.0..0.1).contains(&uptime.collect()[0].get_metric()[0].get_gauge().get_value()));
    sleep(Duration::from_millis(100)).await;
    assert!((0.1..0.2).contains(&uptime.collect()[0].get_metric()[0].get_gauge().get_value()));
    sleep(Duration::from_millis(200)).await;
    assert!((0.3..0.4).contains(&uptime.collect()[0].get_metric()[0].get_gauge().get_value()));
}

#[tokio::test]
async fn rate() {
    let rate = Rate::new(opts!("rate", "Request rate"), &[]).unwrap();
    let o = rate.observe(&[]);
    assert_eq!(
        rate.collect()[0].get_metric()[0].get_counter().get_value(),
        0.0
    );
    assert_eq!(
        rate.collect()[1].get_metric()[0].get_counter().get_value(),
        0.0
    );
    tokio::spawn({
        let rate = rate.clone();
        async move {
            assert_eq!(
                rate.collect()[0].get_metric()[0].get_counter().get_value(),
                1.0
            );
            assert_eq!(
                rate.collect()[1].get_metric()[0].get_counter().get_value(),
                0.0
            );
            sleep(Duration::from_millis(100)).await;
        }
        .monitor(o)
    })
    .await
    .unwrap();
    assert_eq!(
        rate.collect()[0].get_metric()[0].get_counter().get_value(),
        1.0
    );
    assert_eq!(
        rate.collect()[1].get_metric()[0].get_counter().get_value(),
        1.0
    );
}
