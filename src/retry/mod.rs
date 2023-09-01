mod stream;

pub mod policy;

use core::time::Duration;
use stream::Stream;
use tokio::time::sleep;

pub enum Decision<Response> {
    Retry(Duration),
    Break(Response),
}

pub trait Policy<Response, TransformedResponse> {
    fn decide(&mut self, response: Response) -> Decision<TransformedResponse>;
}

pub(crate) async fn retry<S, P, Req, Res>(stream: S, mut policy: P, request: Req) -> Res
where
    S: Stream<Req>,
    P: Policy<<S as Stream<Req>>::Response, Res>,
    Req: Clone,
{
    loop {
        match policy.decide(stream.next(request.clone()).await) {
            Decision::Retry(delay) => sleep(delay).await,
            Decision::Break(response) => break response,
        }
    }
}
