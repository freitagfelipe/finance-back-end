use crate::{
    connection::Database,
    controller::user::dto::CreateUserDTO,
    errors::{Error, Result},
};

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(database: Database, dto: CreateUserDTO) -> Result<()> {
        let result = sqlx::query!(
            "SELECT COUNT(*) AS count FROM user u WHERE u.id = ?",
            dto.id
        )
        .fetch_one(&database.pool)
        .await;

        let Ok(record) = result else {
            return Err(Error::InternalServerError);
        };

        if record.count != 0 {
            return Err(Error::UserAlreadyExists);
        }

        let result = sqlx::query!("INSERT INTO user (id) VALUES (?)", dto.id)
            .execute(&database.pool)
            .await;

        if result.is_err() {
            return Err(Error::InternalServerError);
        }

        Ok(())
    }
}
