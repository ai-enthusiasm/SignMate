use crate::core::entity::vocab::Vocab;
use serde::{Deserialize, Serialize};
use surrealdb_types::{RecordId, SurrealValue};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct SurrealVocab {
  pub id: Option<RecordId>,
  pub word: String,
  pub topic: String,
  pub region: String,
  pub video_path: String,
  pub description: String,
}

impl SurrealVocab {
  pub fn into_core(self) -> Vocab {
    Vocab {
      // Convert RecordId to String (e.g. "vocabulary:tkwse1j5o0anqjxonvzx")
      id: self
        .id
        .map(|id: RecordId| format!("{:?}", id))
        .unwrap_or_default(),
      word: self.word,
      topic: self.topic,
      region: self.region,
      video_path: self.video_path,
      description: self.description,
    }
  }
}
