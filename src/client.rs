use crate::retry::Policy as RetryPolicy;
use async_trait::async_trait;

#[async_trait]
pub trait ExecuteQuery<Request> {
    type Response;

    async fn query(&self, request: Request) -> Self::Response;

    async fn query_with_retry(&self, request: Request) -> Self::Response
    where
        Request: Clone + Send + Sync + 'async_trait,
        Self::Response: Send;

    #[cfg(not(feature = "tokio"))]
    async fn query_with_policy<Policy>(&self, request: Request, policy: Policy) -> Self::Response
    where
        Request: Clone + Send + Sync + 'async_trait,
        Policy: RetryPolicy<Self::Response, Self::Response> + Send,
        Self::Response: Send;

    #[cfg(feature = "tokio")]
    async fn query_with_policy<Policy>(&self, request: Request, policy: Policy) -> Self::Response
    where
        Request: Clone + Send + Sync + 'async_trait,
        Policy: RetryPolicy<Self::Response, Self::Response> + Send,
        Self::Response: Send,
    {
        crate::retry::tokio_retry(|req| self.query(req), policy, (request,)).await
    }
}
