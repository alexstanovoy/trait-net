use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use trait_net::{client::ExecuteQuery, retry::policy::RetryOnError};

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

    async fn query(&self, request: PostRequest) -> Self::Response {
        println!("/post request");
        let response = self
            .http_client
            .post("http://httpbin.org/post")
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    async fn query_with_retry(&self, request: PostRequest) -> Self::Response {
        self.query_with_policy(request, RetryOnError::new(2, Duration::from_millis(100)))
            .await
    }
}

impl ExecuteQuery<UnavailableRequest> for HttpBinClient {
    type Response = Result<UnavailableResponse, Error>;

    async fn query(&self, request: UnavailableRequest) -> Self::Response {
        println!("/status/503 request");
        let _response = self
            .http_client
            .get("http://httpbin.org/status/503")
            .json(&request)
            .send()
            .await?
            .error_for_status()?;
        Ok(UnavailableResponse)
    }

    async fn query_with_retry(&self, request: UnavailableRequest) -> Self::Response {
        self.query_with_policy(request, RetryOnError::new(2, Duration::from_millis(100)))
            .await
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
    println!("Response: {:?}", response);

    let request = UnavailableRequest;
    let response = client.query_with_retry(request).await;
    println!("{:?}", response);
}
