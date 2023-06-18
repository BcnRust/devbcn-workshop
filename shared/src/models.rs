use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, FromRow)]
pub struct Film {
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16,
    pub poster: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
