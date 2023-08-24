use core::{ops::Range, time::Duration};
use super::{Decision, Policy};

pub struct Once;

impl Once {
    pub fn new() -> Self {
        Self
    }
}

impl<Response> Policy<Response, Response> for Once {
    fn decide(&mut self, response: Response) -> Decision<Response> {
        Decision::Break(response)
    }
}

pub struct Fixed {
    counter: Range<usize>,
    delay: Duration,
}

impl Fixed {
    pub fn new(retry_count: usize, delay: Duration) -> Self {
        Self {
            counter: 0..retry_count,
            delay,
        }
    }
}

impl<Response> Policy<Response, Response> for Fixed {
    fn decide(&mut self, response: Response) -> Decision<Response> {
        if self.counter.next().is_some() {
            Decision::Retry(self.delay)
        } else {
            Decision::Break(response)
        }
    }
}
