// use netlify_lambda::{lambda, Context};
// use serde_json::{json, Value};

// type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

// #[lambda]
// #[tokio::main]
// // Note that Request is replaced by Value here, allowing us to
// // accept and access the body value as JSON via serde
// async fn main(event: Value, _: Context) -> Result<Value, Error> {
//     dbg!(&event);
//     let first_name = event["firstName"].as_str().unwrap_or("world");
//     dbg!(first_name);
//     Ok(json!({ "message": format!("Hello, {}!", first_name) }))
// }

use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request, RequestExt,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Name {
    first_name: String,
}

#[lambda(http)]
#[tokio::main]
// Note that Request is replaced by Value here, allowing us to
// accept and access the body value as JSON via serde
async fn main(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let event: Name = serde_json::from_slice(event.body()).unwrap_or(Name {
        first_name: String::from("world"),
    });
    Ok(json!({
        "message": format!("Hello, {}!", event.first_name)
    }))
}
