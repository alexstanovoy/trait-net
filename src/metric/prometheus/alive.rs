use prometheus::IntCounter;
use std::ops::Deref;

pub struct Alive(IntCounter);

impl Deref for Alive {
    type Target = IntCounter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Alive {
    pub fn start<S1: Into<String>, S2: Into<String>>(
        name: S1,
        help: S2,
    ) -> prometheus::Result<Self> {
        let counter = IntCounter::new(name, help)?;
        counter.inc();
        Ok(Self(counter))
    }
}
