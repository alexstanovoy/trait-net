mod future_ext;
// mod tuple;
#[cfg(test)]
mod tests;
#[cfg(feature = "prometheus")]
pub mod prometheus;

pub use future_ext::MetricsFutureExt;

pub trait ScopedMetric {
    type Guard: Observation;
    fn observation(&self, labels: &[&str]) -> Self::Guard;
}

pub trait Observation {
    fn start(&self);
}

impl<T1> Observation for (T1,)
where
    T1: Observation,
{
    fn start(&self) {
        self.0.start();
    }
}

impl<T1, T2> Observation for (T1, T2)
where
    T1: Observation,
    T2: Observation,
{
    fn start(&self) {
        self.0.start();
        self.1.start();
    }
}

impl<T1, T2, T3> Observation for (T1, T2, T3)
where
    T1: Observation,
    T2: Observation,
    T3: Observation,
{
    fn start(&self) {
        self.0.start();
        self.1.start();
        self.2.start();
    }
}
