mod future_ext;
#[cfg(feature = "prometheus")]
pub mod prometheus;

pub use future_ext::MetricsFutureExt;

pub trait ScopedObserver {
    fn start(&self);
    fn stop(&self);
}

impl<O1> ScopedObserver for (O1,)
where
    O1: ScopedObserver,
{
    fn start(&self) {
        self.0.start();
    }

    fn stop(&self) {
        self.0.stop();
    }
}

impl<O1, O2> ScopedObserver for (O1, O2)
where
    O1: ScopedObserver,
    O2: ScopedObserver,
{
    fn start(&self) {
        self.0.start();
        self.1.start();
    }

    fn stop(&self) {
        self.1.stop();
        self.0.stop();
    }
}

impl<O1, O2, O3> ScopedObserver for (O1, O2, O3)
where
    O1: ScopedObserver,
    O2: ScopedObserver,
    O3: ScopedObserver,
{
    fn start(&self) {
        self.0.start();
        self.1.start();
        self.2.start();
    }

    fn stop(&self) {
        self.2.stop();
        self.1.stop();
        self.0.stop();
    }
}

pub trait AsStatusLabel {
    fn as_status_label(&self) -> String;
}

pub trait StatusObserver {
    fn record<Output: AsStatusLabel>(&self, output: &Output);
}

impl<T, E: AsStatusLabel> AsStatusLabel for Result<T, E> {
    fn as_status_label(&self) -> String {
        match self {
            Ok(_) => "ok".to_owned(),
            Err(err) => err.as_status_label(),
        }
    }
}
