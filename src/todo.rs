use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: OffsetDateTime,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            title,
            completed: false,
            created_at: OffsetDateTime::now_utc() - time::Duration::hours(3),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "ID: {}\nTitle: {}\nCompleted: {}\nCreated At: {:?}",
            self.id, self.title, self.completed, self.created_at
        )
    }
}
