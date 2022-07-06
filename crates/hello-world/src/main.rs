use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse, encodings::Body,
    event::apigw::ApiGatewayProxyRequest,
};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(
    _: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(
            "🦀🦀🦀🦀🦀🦀 Hello, Rust 🦀🦀🦀🦀🦀🦀"
                .to_string(),
        )),
        is_base64_encoded: Some(false),
    })
}
