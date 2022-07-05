use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyResponse,
    },
};
use lambda_runtime::{service_fn, Error, LambdaEvent};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct DadJoke {
    id: String,
    joke: String,
}
async fn handler(
    _: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let client = reqwest::Client::new();

    let dadjoke: DadJoke = client
        .get("https://icanhazdadjoke.com/")
        .header("Accept", "application/json")
        .header("User-Agent", "Rust Adventure Serverless Examples (https://github.com/rust-adventure/netlify-serverless-examples)")
        .send()
        .await?
        .json()
        .await?;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(
            serde_json::to_string(&dadjoke).unwrap(),
        )),
        is_base64_encoded: Some(false),
    })
}
