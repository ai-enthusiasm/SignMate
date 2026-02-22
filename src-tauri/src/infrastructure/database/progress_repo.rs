use super::connection::DbConnection;
use super::progress_dto::SurrealUserProgress;
use crate::core::algorithm::anki::calculate_next_review;
use crate::core::entity::progress::{DeckStats, UserProgress};

pub struct SurrealProgressRepo {
  pub db: DbConnection,
}

impl SurrealProgressRepo {
  pub fn new(db: DbConnection) -> Self {
    Self { db }
  }

  /// Get vocabulary items that are due for review today
  pub async fn get_daily_cards(&self) -> Result<Vec<UserProgress>, String> {
    let db = self.db.lock().await;
    let now = std::time::SystemTime::now()
      .duration_since(std::time::UNIX_EPOCH)
      .unwrap()
      .as_secs() as i64;

    let query = format!(
      "SELECT * FROM user_progress WHERE next_review_at <= {} AND status != 'Bookmarked' ORDER BY next_review_at ASC",
      now
    );
    let mut result = db
      .query(&query)
      .await
      .map_err(|e| format!("Query failed: {}", e))?;
    let cards: Vec<SurrealUserProgress> = result
      .take(0)
      .map_err(|e| format!("Failed to parse cards: {}", e))?;
    Ok(cards.into_iter().map(|p| p.into_core()).collect())
  }

  /// Update a card's progress using the Anki SM-2 algorithm
  pub async fn update_card_progress(&self, vocab_id: String, quality: u8) -> Result<i64, String> {
    let db = self.db.lock().await;

    // Get existing progress or create new
    let query = format!(
      "SELECT * FROM user_progress WHERE vocab_id = '{}' AND status != 'Bookmarked'",
      vocab_id
    );
    let mut result = db
      .query(&query)
      .await
      .map_err(|e| format!("Query failed: {}", e))?;
    let existing: Vec<SurrealUserProgress> =
      result.take(0).map_err(|e| format!("Parse error: {}", e))?;

    let mut progress = if let Some(p) = existing.into_iter().next() {
      p.into_core()
    } else {
      UserProgress {
        id: String::new(),
        vocab_id: vocab_id.clone(),
        status: "New".to_string(),
        interval: 0,
        ease_factor: 2.5,
        next_review_at: 0,
      }
    };

    // Run the Anki SM-2 algorithm
    calculate_next_review(&mut progress, quality);

    // Save back to database
    db.query(&format!(
      "UPDATE user_progress SET status = '{}', interval = {}, ease_factor = {}, next_review_at = {} WHERE vocab_id = '{}' AND status != 'Bookmarked'",
      progress.status, progress.interval, progress.ease_factor, progress.next_review_at, vocab_id
    ))
    .await
    .map_err(|e| format!("Failed to update progress: {}", e))?;

    Ok(progress.next_review_at)
  }

  /// Get statistics for the dashboard
  pub async fn get_deck_stats(&self) -> Result<DeckStats, String> {
    let db = self.db.lock().await;

    let mut result = db.query(
      "SELECT count() AS total FROM user_progress WHERE status = 'New' GROUP ALL;
       SELECT count() AS total FROM user_progress WHERE status = 'Review' OR status = 'Learning' GROUP ALL;
       SELECT count() AS total FROM user_progress WHERE status = 'Mastered' GROUP ALL;"
    )
    .await
    .map_err(|e| format!("Query failed: {}", e))?;

    let new_cards: Vec<serde_json::Value> = result.take(0).unwrap_or_default();
    let review_cards: Vec<serde_json::Value> = result.take(1).unwrap_or_default();
    let learned_cards: Vec<serde_json::Value> = result.take(2).unwrap_or_default();

    let new = new_cards
      .first()
      .and_then(|v| v.get("total"))
      .and_then(|v| v.as_u64())
      .unwrap_or(0) as u32;
    let review = review_cards
      .first()
      .and_then(|v| v.get("total"))
      .and_then(|v| v.as_u64())
      .unwrap_or(0) as u32;
    let learned = learned_cards
      .first()
      .and_then(|v| v.get("total"))
      .and_then(|v| v.as_u64())
      .unwrap_or(0) as u32;

    Ok(DeckStats {
      new,
      review,
      learned,
    })
  }
}
