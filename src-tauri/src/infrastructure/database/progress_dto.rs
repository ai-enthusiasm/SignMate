use crate::core::entity::progress::UserProgress;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct SurrealUserProgress {
  pub id: Option<RecordId>,
  pub vocab_id: String,
  pub status: String,
  pub interval: u32,
  pub ease_factor: f32,
  pub next_review_at: i64,
}

impl SurrealUserProgress {
  pub fn into_core(self) -> UserProgress {
    UserProgress {
      id: self
        .id
        .map(|id: RecordId| format!("{:?}", id))
        .unwrap_or_default(),
      vocab_id: self.vocab_id,
      status: self.status,
      interval: self.interval,
      ease_factor: self.ease_factor,
      next_review_at: self.next_review_at,
    }
  }
}
