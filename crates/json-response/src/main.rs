use netlify_lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Name {
    first_name: String,
}

#[lambda(http)]
#[tokio::main]
async fn main(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let event: Name = serde_json::from_slice(event.body()).unwrap_or(Name {
        first_name: String::from("world"),
    });
    Ok(json!({
        "message": format!("Hello, {}!", event.first_name)
    }))
}
