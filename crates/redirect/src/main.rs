use http::StatusCode;

use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    _: Value,
    _: Context,
) -> Result<Value, Error> {
    Ok(json!({
        "headers": {
            "Location": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
        },
        "statusCode": StatusCode::TEMPORARY_REDIRECT.to_string()
    }))
}
