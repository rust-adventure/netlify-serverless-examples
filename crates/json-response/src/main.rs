use aws_lambda_events::{
    apigw::ApiGatewayProxyResponse, encodings::Body,
};
use http::HeaderMap;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(handler)).await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Name {
    first_name: String,
}

#[derive(Debug, Deserialize)]
struct Event {
    body: String,
}

async fn handler(
    event: LambdaEvent<Event>,
) -> Result<ApiGatewayProxyResponse, Error> {
    let (event, _context) = event.into_parts();
    let name = serde_json::from_str(&event.body);
    let first_name = name
        .unwrap_or(Name {
            first_name: String::from("world"),
        })
        .first_name;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!(
            "Hello, {}!",
            first_name
        ))),
        is_base64_encoded: Some(false),
    })
}
