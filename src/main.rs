use std::{env, error::Error};

use axum::Router;
use finance_back_end::{self, connection::Database, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;

    let listener =
        TcpListener::bind(env::var("URL").expect("Could not find URL enviroment var")).await?;

    let pool = sqlx::mysql::MySqlPool::connect(
        &env::var("DATABASE_URL").expect("Could not find DB_URI enviroment var"),
    )
    .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let router = Router::new()
        .merge(finance_back_end::app_router())
        .with_state(AppState {
            database: Database { pool },
        });

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
