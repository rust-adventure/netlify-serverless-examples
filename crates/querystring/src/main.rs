use aws_lambda_events::{
    apigw::{
        ApiGatewayProxyRequest, ApiGatewayProxyResponse,
    },
    encodings::Body,
};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (event, _context) = event.into_parts();
    let response = format!(
        "hello {}",
        event
            .query_string_parameters
            .first("name")
            .unwrap_or("stranger")
    );
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(response)),
        is_base64_encoded: Some(false),
    })
}
