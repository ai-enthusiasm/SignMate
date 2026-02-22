use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProgress {
  pub id: String,
  pub vocab_id: String,
  pub status: String,
  pub interval: u32,
  pub ease_factor: f32,
  pub next_review_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckStats {
  pub new: u32,
  pub review: u32,
  pub learned: u32,
}
