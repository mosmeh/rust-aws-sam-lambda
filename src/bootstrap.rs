use netlify_lambda_http::{
    lambda::{self, Context},
    Request, RequestExt,
};
use serde::Deserialize;
use serde_json::{json, Value};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(netlify_lambda_http::handler(handler)).await
}

#[derive(Deserialize)]
struct Params {
    name: String,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            name: "world".to_owned(),
        }
    }
}

async fn handler(req: Request, _: Context) -> Result<Value, Error> {
    let params: Params = req.payload().unwrap_or(None).unwrap_or_default();
    Ok(json!({ "message": format!("Hello, {}!", params.name) }))
}
