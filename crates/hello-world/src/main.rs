use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(
    _: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<Value, Error> {
    Ok(
        json!({"body": "🦀🦀🦀🦀🦀🦀 Hello, Rust 🦀🦀🦀🦀🦀🦀"}),
    )
}
