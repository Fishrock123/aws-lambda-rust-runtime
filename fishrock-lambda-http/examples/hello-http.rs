use fishrock_lambda_http as lambda_http;

use lambda_http::{
    handler,
    lambda_runtime::{self, Context},
    IntoResponse, Request, RequestExt, Response,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(match event.query_string_parameters().get("first_name") {
        Some(first_name) => format!("Hello, {}!", first_name).into_response(),
        _ => Response::builder()
            .status(400)
            .body("Empty first name".into())
            .expect("failed to render response"),
    })
}