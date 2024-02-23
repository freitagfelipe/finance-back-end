#[derive(Clone)]
pub struct Database {
    pub pool: sqlx::mysql::MySqlPool,
}
