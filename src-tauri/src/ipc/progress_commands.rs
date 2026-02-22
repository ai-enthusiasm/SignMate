use crate::core::entity::progress::{DeckStats, UserProgress};
use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::progress_repo::SurrealProgressRepo;
use tauri::State;

/// Returns words due for review today (Anki daily cards)
#[tauri::command]
pub async fn get_daily_cards(db: State<'_, DbConnection>) -> Result<Vec<UserProgress>, String> {
  let repo = SurrealProgressRepo::new(db.inner().clone());
  repo.get_daily_cards().await
}

/// Updates Anki stats after practice. quality: 0 (Fail) to 5 (Perfect)
#[tauri::command]
pub async fn update_card_progress(
  db: State<'_, DbConnection>,
  vocab_id: String,
  quality: u8,
) -> Result<i64, String> {
  let repo = SurrealProgressRepo::new(db.inner().clone());
  repo.update_card_progress(vocab_id, quality).await
}

/// Returns deck statistics for the dashboard
#[tauri::command]
pub async fn get_deck_stats(db: State<'_, DbConnection>) -> Result<DeckStats, String> {
  let repo = SurrealProgressRepo::new(db.inner().clone());
  repo.get_deck_stats().await
}
