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
    Ok(
        json!({"body": "🦀🦀🦀🦀🦀🦀 Hello, Rust 🦀🦀🦀🦀🦀🦀"}),
    )
}
