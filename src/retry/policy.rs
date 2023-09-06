use super::{Decision, Policy};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug)]
pub struct RetryOnError {
    retry_attempts: usize,
    delay: Duration,
}

impl RetryOnError {
    pub fn new(retry_attempts: usize, delay: Duration) -> Self {
        Self {
            retry_attempts,
            delay,
        }
    }
}

impl<Response, Error> Policy<Result<Response, Error>, Result<Response, Error>> for RetryOnError {
    fn decide(&mut self, response: Result<Response, Error>) -> Decision<Result<Response, Error>> {
        if response.is_ok() {
            Decision::Break(response)
        } else if self.retry_attempts > 0 {
            self.retry_attempts -= 1;
            Decision::Retry(self.delay)
        } else {
            Decision::Break(response)
        }
    }
}
