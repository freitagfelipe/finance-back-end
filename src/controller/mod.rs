use axum::{
    http::{StatusCode, Uri},
    Router,
};

trait Controller {
    fn get_controller_router() -> Router;
}

pub fn app_router() -> Router {
    Router::new()
        .fallback(|uri: Uri| async move { (StatusCode::NOT_FOUND, format!("No route for {uri}")) })
}
