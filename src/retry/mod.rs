pub mod policy;
pub mod stream;

use std::{future::Future, time::Duration};

pub use stream::Stream;

pub enum Decision {
    Retry(Duration),
    Break,
}

pub trait Policy<Response> {
    fn decide(&mut self, response: &Response) -> Decision;

    fn retry<Request, S>(
        mut self,
        stream: S,
        request: Request,
    ) -> impl Future<Output = S::Response> + Send
    where
        Response: Send,
        Request: Clone + Send,
        S: Stream<Request, Response = Response> + Send,
        <S as Stream<Request>>::Function: Send,
        Self: Sized + Send,
    {
        async move {
            loop {
                let response = stream.next(request.clone()).await;
                match self.decide(&response) {
                    Decision::Retry(delay) => tokio::time::sleep(delay).await,
                    Decision::Break => break response,
                }
            }
        }
    }
}
