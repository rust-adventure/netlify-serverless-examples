use aws_lambda_events::{
    encodings::Body,
    event::apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyResponse,
    },
};
use lambda_runtime::{handler_fn, Context, Error};
use reqwest::header::HeaderMap;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct DadJoke {
    id: String,
    joke: String,
}
async fn handler(
    _: ApiGatewayProxyRequest,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let client = reqwest::Client::new();

    let dadjoke: DadJoke = client
        .get("https://www.rust-lang.org")
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
        body: Some(Body::Text(dadjoke.joke)),
        is_base64_encoded: Some(false),
    })
}
