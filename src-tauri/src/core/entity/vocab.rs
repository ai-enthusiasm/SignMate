use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vocab {
  pub id: String,
  pub word: String,
  pub topic: String,
  pub region: String,
  pub video_path: String,
  pub description: String,
}
