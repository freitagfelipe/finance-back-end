use std::{env, error::Error};

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("person");

    format!("Hello, {name}!")
}

async fn handler_greeting(Path(name): Path<String>) -> impl IntoResponse {
    format!("Welcome {name}!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let listener = TcpListener::bind(
        env::var("URL").expect("Could not find URL enviroment var")
    ).await?;

    let router = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/hello", get(handler_hello))
        .route("/greeting/:name", get(handler_greeting));

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
