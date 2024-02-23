pub mod connection;
mod controller;
mod model;

use connection::Database;
pub use controller::app_router;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
}
