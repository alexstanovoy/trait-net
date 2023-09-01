use super::Observation;

pub struct Guard2<T1, T2>(T1, T2);
pub struct Guard3<T1, T2, T3>(T1, T2, T3);

impl<T1, T2> Guard2<T1, T2> {
    pub(super) fn new(t1: T1, t2: T2) -> Self {
        Guard2(t1, t2)
    }
}

impl<T1, T2, T3> Guard3<T1, T2, T3> {
    pub(super) fn new(t1: T1, t2: T2, t3: T3) -> Self {
        Guard3(t1, t2, t3)
    }
}

impl<T1, T2> Observation for Guard2<T1, T2>
where
    T1: Observation,
    T2: Observation,
{
    fn start(&self) {
        self.0.start();
        self.1.start();
    }
}

impl<T1, T2, T3> Observation for Guard3<T1, T2, T3>
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
