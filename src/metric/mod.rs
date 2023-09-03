mod future_ext;
#[cfg(feature = "prometheus")]
pub mod prometheus;
#[cfg(test)]
mod tests;

pub use future_ext::MetricsFutureExt;

pub trait Observer {
    fn start(&mut self);
    fn stop(&mut self);
    fn record<Output>(&mut self, output: &Output);
}

impl<O1> Observer for (O1,)
where
    O1: Observer,
{
    fn start(&mut self) {
        self.0.start();
    }

    fn stop(&mut self) {
        self.0.stop();
    }

    fn record<T>(&mut self, output: &T) {
        self.0.record(output);
    }
}

impl<O1, O2> Observer for (O1, O2)
where
    O1: Observer,
    O2: Observer,
{
    fn start(&mut self) {
        self.0.start();
        self.1.start();
    }

    fn stop(&mut self) {
        self.1.stop();
        self.0.stop();
    }

    fn record<T>(&mut self, output: &T) {
        self.0.record(output);
        self.1.record(output);
    }
}

impl<O1, O2, O3> Observer for (O1, O2, O3)
where
    O1: Observer,
    O2: Observer,
    O3: Observer,
{
    fn start(&mut self) {
        self.0.start();
        self.1.start();
        self.2.start();
    }

    fn stop(&mut self) {
        self.2.stop();
        self.1.stop();
        self.0.stop();
    }

    fn record<T>(&mut self, output: &T) {
        self.0.record(output);
        self.1.record(output);
        self.2.record(output);
    }
}
