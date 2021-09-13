use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
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
    event: Event,
    _: Context,
) -> Result<Value, Error> {
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
