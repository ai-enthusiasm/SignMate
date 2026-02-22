use crate::core::entity::vocab::Vocab;
use crate::infrastructure::database::connection::DbConnection;
use crate::infrastructure::database::vocab_repo::SurrealVocabRepo;
use tauri::State;

/// Returns a paginated list of vocabulary with optional filters
#[tauri::command]
pub async fn get_vocab_list(
  db: State<'_, DbConnection>,
  page: u32,
  filter_topic: Option<String>,
  filter_region: Option<String>,
  search_query: Option<String>,
) -> Result<Vec<Vocab>, String> {
  let repo = SurrealVocabRepo::new(db.inner().clone());
  repo
    .get_vocab_list(page, filter_topic, filter_region, search_query)
    .await
}

/// Returns details for a single vocabulary item
#[tauri::command]
pub async fn get_vocab_detail(db: State<'_, DbConnection>, id: String) -> Result<Vocab, String> {
  let repo = SurrealVocabRepo::new(db.inner().clone());
  repo.get_vocab_detail(id).await
}

/// Returns related videos from the same topic
#[tauri::command]
pub async fn get_related_videos(
  db: State<'_, DbConnection>,
  topic: String,
  current_id: String,
) -> Result<Vec<Vocab>, String> {
  let repo = SurrealVocabRepo::new(db.inner().clone());
  repo.get_related_videos(topic, current_id).await
}

/// Toggle bookmark for a vocabulary item
#[tauri::command]
pub async fn toggle_bookmark(db: State<'_, DbConnection>, id: String) -> Result<bool, String> {
  let repo = SurrealVocabRepo::new(db.inner().clone());
  repo.toggle_bookmark(id).await
}
