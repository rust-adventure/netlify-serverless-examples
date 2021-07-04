use http::StatusCode;
use lamedh_http::{
    lambda::{lambda, Context},
    IntoResponse, Request, Response,
};

type Error =
    Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda(http)]
#[tokio::main]
async fn main(
    _: Request,
    _: Context,
) -> Result<impl IntoResponse, Error> {
    let response = Response::builder()
        .header(
            "Location",
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        )
        .status(StatusCode::TEMPORARY_REDIRECT)
        .body(());
    response.map_err(|_| "something went wrong!".into())
}
