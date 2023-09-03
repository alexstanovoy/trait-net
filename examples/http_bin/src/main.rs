use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin, time::Duration};
use trait_net::{client::ExecuteQuery, retry::policy::RetryOnError, retry::tokio_retry};

pub struct HttpBinClient {
    http_client: Client,
}

impl HttpBinClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct PostRequest {
    name: String,
    surname: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostResponse {
    data: String,
}

pub struct RateMetric {}

#[derive(Clone, Debug, Serialize)]
pub struct UnavailableRequest;

#[derive(Clone, Debug, Deserialize)]
pub struct UnavailableResponse;

impl ExecuteQuery<PostRequest> for HttpBinClient {
    type Response = Result<PostResponse, Error>;

    #[must_use]
    fn query<'life0, 'async_trait>(
        &'life0 self,
        request: PostRequest,
    ) -> Pin<Box<dyn Future<Output = Self::Response> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let response = self
                .http_client
                .post("https://httpbin.org/post")
                .json(&request)
                .send()
                .await?
                .json()
                .await?;
            Ok(response)
        })
    }

    fn query_with_retry<'life0, 'async_trait>(
        &'life0 self,
        request: PostRequest,
    ) -> Pin<Box<dyn Future<Output = Self::Response> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(tokio_retry(
            |req| self.query(req),
            RetryOnError::new(2, Duration::from_millis(100)),
            (request,),
        ))
    }
}

impl ExecuteQuery<UnavailableRequest> for HttpBinClient {
    type Response = Result<UnavailableResponse, Error>;

    #[must_use]
    fn query<'life0, 'async_trait>(
        &'life0 self,
        request: UnavailableRequest,
    ) -> Pin<Box<dyn Future<Output = Self::Response> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let response = self
                .http_client
                .get("https://httpbin.org/status/503")
                .json(&request)
                .send()
                .await?
                .error_for_status()?;
            Ok(UnavailableResponse)
        })
    }

    fn query_with_retry<'life0, 'async_trait>(
        &'life0 self,
        request: UnavailableRequest,
    ) -> Pin<Box<dyn Future<Output = Self::Response> + Send + 'async_trait>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(tokio_retry(
            |req| self.query(req),
            RetryOnError::new(2, Duration::from_secs(1)),
            (request,),
        ))
    }
}

#[tokio::main]
async fn main() {
    let client = HttpBinClient::new();

    let request = PostRequest {
        name: "Foo".to_owned(),
        surname: "Bar".to_owned(),
    };
    let response = client.query(request).await;
    println!("{:?}", response);

    let request = UnavailableRequest;
    let response = client.query_with_retry(request).await;
    println!("{:?}", response);
}
