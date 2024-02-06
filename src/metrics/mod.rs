mod future_ext;

#[cfg(feature = "prometheus")]
pub mod prometheus;

pub use future_ext::{MeteredFuture, MetricsFutureExt};

pub trait Observer<Out> {
    fn on_first_poll(&mut self) {}
    fn on_poll_ready(&mut self, _output: &Out) {}
    fn on_drop(&mut self) {}
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
    fn on_first_poll(&mut self) {
        self.0.on_first_poll();
    }

    fn on_poll_ready(&mut self, output: &Out) {
        self.0.on_poll_ready(output);
    }

    fn on_drop(&mut self) {
        self.0.on_drop();
    }
}

impl<Out, O1, O2> Observer<Out> for (O1, O2)
where
    O1: Observer<Out>,
    O2: Observer<Out>,
{
    fn on_first_poll(&mut self) {
        self.0.on_first_poll();
        self.1.on_first_poll();
    }

    fn on_poll_ready(&mut self, output: &Out) {
        self.0.on_poll_ready(output);
        self.1.on_poll_ready(output)
    }

    fn on_drop(&mut self) {
        self.0.on_drop();
        self.1.on_drop();
    }
}

impl<Out, O1, O2, O3> Observer<Out> for (O1, O2, O3)
where
    O1: Observer<Out>,
    O2: Observer<Out>,
    O3: Observer<Out>,
{
    fn on_first_poll(&mut self) {
        self.0.on_first_poll();
        self.1.on_first_poll();
        self.2.on_first_poll();
    }

    fn on_poll_ready(&mut self, output: &Out) {
        self.0.on_poll_ready(output);
        self.1.on_poll_ready(output);
        self.2.on_poll_ready(output);
    }

    fn on_drop(&mut self) {
        self.0.on_drop();
        self.1.on_drop();
        self.2.on_drop();
    }
}

impl<Out, O1, O2, O3, O4> Observer<Out> for (O1, O2, O3, O4)
where
    O1: Observer<Out>,
    O2: Observer<Out>,
    O3: Observer<Out>,
    O4: Observer<Out>,
{
    fn on_first_poll(&mut self) {
        self.0.on_first_poll();
        self.1.on_first_poll();
        self.2.on_first_poll();
        self.3.on_first_poll();
    }

    fn on_poll_ready(&mut self, output: &Out) {
        self.0.on_poll_ready(output);
        self.1.on_poll_ready(output);
        self.2.on_poll_ready(output);
        self.3.on_poll_ready(output);
    }

    fn on_drop(&mut self) {
        self.0.on_drop();
        self.1.on_drop();
        self.2.on_drop();
        self.3.on_drop();
    }
}
