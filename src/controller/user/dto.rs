use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDTO {
    pub id: i64,
}
