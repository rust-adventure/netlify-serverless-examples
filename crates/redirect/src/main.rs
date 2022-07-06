use aws_lambda_events::apigw::{
    ApiGatewayProxyRequest, ApiGatewayProxyResponse,
};
use http::{HeaderMap, HeaderValue, StatusCode};

use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(
    _: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Location",
        HeaderValue::from_static(
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        ),
    );
    Ok(ApiGatewayProxyResponse {
        status_code: StatusCode::TEMPORARY_REDIRECT
            .as_u16()
            .into(),
        headers: headers,
        multi_value_headers: HeaderMap::new(),
        body: None,
        is_base64_encoded: Some(false),
    })
}
