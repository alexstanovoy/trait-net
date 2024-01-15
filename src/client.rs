use crate::retry::Policy as RetryPolicy;
use std::future::Future;

pub trait ExecuteQuery<Request> {
    type Response;

    fn query(&self, request: Request) -> impl Future<Output = Self::Response> + Send;

    fn query_with_retry(&self, request: Request) -> impl Future<Output = Self::Response> + Send
    where
        Request: Clone,
        Self::Response: Send;

    fn query_with_policy<Policy>(
        &self,
        request: Request,
        policy: Policy,
    ) -> impl Future<Output = Self::Response> + Send
    where
        Request: Clone + Send,
        Policy: RetryPolicy<Self::Response> + Send,
        Self::Response: Send,
        Self: Sync,
    {
        async move { policy.retry(|req| self.query(req), (request,)).await }
    }
}
