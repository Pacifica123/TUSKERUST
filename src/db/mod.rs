use sqlx::PgPool;

pub struct DbClient {
    pub conn: PgPool,
}

impl DbClient {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }
}

pub mod users;
pub mod taskboards;