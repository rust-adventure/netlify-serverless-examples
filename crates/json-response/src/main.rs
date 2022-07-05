use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    body: Option<Name>,
}

async fn handler(
    event: LambdaEvent<Event>,
) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let first_name = event
        .body
        .unwrap_or(Name {
            first_name: String::from("world"),
        })
        .first_name;

    Ok(json!({
        "message": format!("Hello, {}!", first_name)
    }))
}
