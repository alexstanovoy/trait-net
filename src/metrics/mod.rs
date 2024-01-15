mod future_ext;

#[cfg(feature = "prometheus")]
pub mod prometheus;

pub use future_ext::MetricsFutureExt;

pub trait Observer<Out> {
    fn start(&mut self);
    fn stop(&mut self);
    fn record(&mut self, output: &Out);
}

pub trait AsStatusLabel {
    fn as_status_label(&self) -> String;
}

impl<T, E: AsStatusLabel> AsStatusLabel for Result<T, E> {
    fn as_status_label(&self) -> String {
        match self {
            Ok(_) => "ok".to_owned(),
            Err(err) => err.as_status_label(),
        }
    }
}

impl<Out, O1> Observer<Out> for (O1,)
where
    O1: Observer<Out>,
{
    fn start(&mut self) {
        self.0.start();
    }

    fn stop(&mut self) {
        self.0.stop();
    }

    fn record(&mut self, output: &Out) {
        self.0.record(output);
    }
}

impl<Out, O1, O2> Observer<Out> for (O1, O2)
where
    O1: Observer<Out>,
    O2: Observer<Out>,
{
    fn start(&mut self) {
        self.0.start();
        self.1.start();
    }

    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
    }

    fn record(&mut self, output: &Out) {
        self.0.record(output);
        self.1.record(output)
    }
}

impl<Out, O1, O2, O3> Observer<Out> for (O1, O2, O3)
where
    O1: Observer<Out>,
    O2: Observer<Out>,
    O3: Observer<Out>,
{
    fn start(&mut self) {
        self.0.start();
        self.1.start();
        self.2.start();
    }

    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
        self.2.stop();
    }

    fn record(&mut self, output: &Out) {
        self.0.record(output);
        self.1.record(output);
        self.2.record(output);
    }
}

impl<Out, O1, O2, O3, O4> Observer<Out> for (O1, O2, O3, O4)
where
    O1: Observer<Out>,
    O2: Observer<Out>,
    O3: Observer<Out>,
    O4: Observer<Out>,
{
    fn start(&mut self) {
        self.0.start();
        self.1.start();
        self.2.start();
        self.3.start();
    }

    fn stop(&mut self) {
        self.0.stop();
        self.1.stop();
        self.2.stop();
        self.3.stop();
    }

    fn record(&mut self, output: &Out) {
        self.0.record(output);
        self.1.record(output);
        self.2.record(output);
        self.3.record(output);
    }
}
