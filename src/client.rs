use async_trait::async_trait;
use crate::retry::{retry, Policy as RetryPolicy};

#[async_trait]
pub trait ExecuteQuery<Request> {
    type Response;

    async fn query(&self, request: Request) -> Self::Response;
    
    async fn query_with_retry<Policy>(&self, request: Request, policy: Policy) -> Self::Response
    where
        Request: Clone + Send + Sync + 'async_trait,
        Policy: RetryPolicy<Self::Response, Self::Response> + Send,
        Self::Response: Send,
    {
        retry(|req| self.query(req), policy, (request,)).await
    }
}
