pub mod user;

use axum::{
    http::{StatusCode, Uri},
    Router,
};

use crate::AppState;

use self::user::UserController;

trait Controller {
    fn get_controller_router() -> Router<AppState>;
}

pub fn app_router() -> Router<AppState> {
    Router::new()
        .merge(UserController::get_controller_router())
        .fallback(|uri: Uri| async move { (StatusCode::NOT_FOUND, format!("No route for {uri}")) })
}
