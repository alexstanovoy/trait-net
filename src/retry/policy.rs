use super::{Decision, Policy};
use core::{ops::Range, time::Duration};

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

pub struct RetryOnError {
    counter: Range<usize>,
    delay: Duration,
}

impl RetryOnError {
    pub fn new(retry_count: usize, delay: Duration) -> Self {
        Self {
            counter: 0..retry_count,
            delay,
        }
    }
}

impl<Response, Error> Policy<Result<Response, Error>, Result<Response, Error>> for RetryOnError {
    fn decide(&mut self, response: Result<Response, Error>) -> Decision<Result<Response, Error>> {
        if response.is_ok() {
            Decision::Break(response)
        } else if self.counter.next().is_some() {
            Decision::Retry(self.delay)
        } else {
            Decision::Break(response)
        }
    }
}
