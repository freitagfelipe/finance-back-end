pub mod dto;

use crate::{controller::Controller, errors::Result, repository::user::UserRepository, AppState};
use axum::{
    extract::{Json, State},
    routing, Router,
};

use self::dto::CreateUserDTO;

pub struct UserController;

impl Controller for UserController {
    fn get_controller_router() -> Router<AppState> {
        Router::new().route("/user", routing::post(create_user))
    }
}

async fn create_user(
    State(app_state): State<AppState>,
    Json(dto): Json<CreateUserDTO>,
) -> Result<()> {
    UserRepository::create_user(app_state.database, dto).await
}
