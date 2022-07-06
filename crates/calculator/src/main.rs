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
    let (event, _) = event.into_parts();
    let a = event
        .query_string_parameters
        .first("a")
        .unwrap_or("2");
    let b = event
        .query_string_parameters
        .first("b")
        .unwrap_or("2");

    match (a.parse::<isize>(), b.parse::<isize>()) {
        (Ok(num_a), Ok(num_b)) => {
            Ok(ApiGatewayProxyResponse {
                status_code: 200,
                headers: HeaderMap::new(),
                multi_value_headers: HeaderMap::new(),
                body: Some(Body::Text(
                    format!("{} + {} = {}", a, b, num_a + num_b)
                        .to_string(),
                )),
                is_base64_encoded: Some(false),
            })
        }
        (Ok(_), Err(_)) => Ok(ApiGatewayProxyResponse {
            status_code: 200,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                format!(
                    "failed to parse number b from `{}`",
                    b
                )
                .to_string(),
            )),
            is_base64_encoded: Some(false),
        }),
        (Err(_), Ok(_)) => Ok(ApiGatewayProxyResponse {
            status_code: 200,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                format!(
                    "failed to parse number a from `{}`",
                    a
                )
                .to_string(),
            )),
            is_base64_encoded: Some(false),
        }),
        (Err(_), Err(_)) => Ok(ApiGatewayProxyResponse {
            status_code: 200,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(
                format!(
                    "failed to parse number a or b from `{}`, `{}`",
                    a,b
                )
                .to_string(),
            )),
            is_base64_encoded: Some(false),
        }),
    }
}
