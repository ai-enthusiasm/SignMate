use super::connection::DbConnection;
use super::vocab_dto::SurrealVocab;
use crate::core::entity::vocab::Vocab;

/// The real implementation of VocabularyRepository that talks to SurrealDB
pub struct SurrealVocabRepo {
  pub db: DbConnection,
}

impl SurrealVocabRepo {
  pub fn new(db: DbConnection) -> Self {
    Self { db }
  }

  /// Get paginated vocabulary list with optional topic/region filters
  pub async fn get_vocab_list(
    &self,
    page: u32,
    filter_topic: Option<String>,
    filter_region: Option<String>,
    search_query: Option<String>,
  ) -> Result<Vec<Vocab>, String> {
    let db = self.db.lock().await;
    let limit = 20;
    let offset = page * limit;

    // Build dynamic query based on filters
    let mut conditions = Vec::new();
    if let Some(ref topic) = filter_topic {
      conditions.push(format!("topic = '{}'", topic));
    }
    if let Some(ref region) = filter_region {
      conditions.push(format!("region = '{}'", region));
    }
    if let Some(ref query) = search_query {
      conditions.push(format!("word CONTAINS '{}'", query));
    }

    let where_clause = if conditions.is_empty() {
      String::new()
    } else {
      format!("WHERE {}", conditions.join(" AND "))
    };

    let query = format!(
      "SELECT * FROM vocabulary {} LIMIT {} START {}",
      where_clause, limit, offset
    );

    let mut result = db
      .query(&query)
      .await
      .map_err(|e| format!("Query failed: {}", e))?;

    let surreal_vocabs: Vec<SurrealVocab> = result
      .take(0)
      .map_err(|e| format!("Failed to parse vocab list: {}", e))?;

    Ok(
      surreal_vocabs
        .into_iter()
        .map(|sv| sv.into_core())
        .collect(),
    )
  }

  /// Get a single vocabulary item by ID
  pub async fn get_vocab_detail(&self, id: String) -> Result<Vocab, String> {
    let db = self.db.lock().await;

    let query = format!("SELECT * FROM vocabulary:{}", id);
    let mut result = db
      .query(&query)
      .await
      .map_err(|e| format!("Failed to get vocab: {}", e))?;

    let vocabs: Vec<SurrealVocab> = result
      .take(0)
      .map_err(|e| format!("Failed to parse vocab: {}", e))?;

    let vocab = vocabs
      .into_iter()
      .next()
      .ok_or_else(|| format!("Vocabulary with id '{}' not found", id))?;

    Ok(vocab.into_core())
  }

  /// Get related videos from the same topic, excluding the current one
  pub async fn get_related_videos(
    &self,
    topic: String,
    current_id: String,
  ) -> Result<Vec<Vocab>, String> {
    let db = self.db.lock().await;

    let query = format!(
      "SELECT * FROM vocabulary WHERE topic = '{}' AND id != vocabulary:{} LIMIT 5",
      topic, current_id
    );

    let mut result = db
      .query(&query)
      .await
      .map_err(|e| format!("Query failed: {}", e))?;

    let surreal_vocabs: Vec<SurrealVocab> = result
      .take(0)
      .map_err(|e| format!("Failed to parse related videos: {}", e))?;

    Ok(
      surreal_vocabs
        .into_iter()
        .map(|sv| sv.into_core())
        .collect(),
    )
  }

  /// Toggle a bookmark for a vocabulary item (saves to user_progress with status "Bookmarked")
  pub async fn toggle_bookmark(&self, id: String) -> Result<bool, String> {
    let db = self.db.lock().await;

    // Check if bookmark exists
    let query = format!(
      "SELECT * FROM user_progress WHERE vocab_id = '{}' AND status = 'Bookmarked'",
      id
    );
    let mut result = db
      .query(&query)
      .await
      .map_err(|e| format!("Query failed: {}", e))?;

    let existing: Vec<serde_json::Value> =
      result.take(0).map_err(|e| format!("Parse error: {}", e))?;

    if existing.is_empty() {
      // Add bookmark
      db.query(&format!(
        "CREATE user_progress SET vocab_id = '{}', status = 'Bookmarked', interval = 0, ease_factor = 2.5, next_review_at = 0",
        id
      ))
      .await
      .map_err(|e| format!("Failed to add bookmark: {}", e))?;
      Ok(true) // bookmarked
    } else {
      // Remove bookmark
      db.query(&format!(
        "DELETE FROM user_progress WHERE vocab_id = '{}' AND status = 'Bookmarked'",
        id
      ))
      .await
      .map_err(|e| format!("Failed to remove bookmark: {}", e))?;
      Ok(false) // unbookmarked
    }
  }
}
