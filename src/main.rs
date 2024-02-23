use std::{env, error::Error};

use axum::Router;
use finance_back_end::{self, connection::Database};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let listener =
        TcpListener::bind(env::var("URL").expect("Could not find URL enviroment var")).await?;

    let pool = sqlx::mysql::MySqlPool::connect(
        &env::var("DB_URI").expect("Could not find DB_URI enviroment var"),
    )
    .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let router = Router::new()
        .with_state(Database { pool })
        .merge(finance_back_end::app_router());

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
