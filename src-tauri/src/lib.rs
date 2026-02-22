use tauri::Manager;

pub mod applications;
pub mod core;
pub mod infrastructure;
pub mod ipc;

use infrastructure::database::connection;

use ipc::progress_commands::{get_daily_cards, get_deck_stats, update_card_progress};
use ipc::vocab_commands::{get_related_videos, get_vocab_detail, get_vocab_list, toggle_bookmark};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      // Get app data directory for database storage
      let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .to_string_lossy()
        .to_string();

      // Initialize database in a background task
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        match connection::init_db(&app_data_dir).await {
          Ok(db) => {
            handle.manage(db);
            println!("✅ SurrealDB connected successfully!");
          }
          Err(e) => {
            eprintln!("❌ Failed to initialize database: {}", e);
          }
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      // Module A: Dictionary & Learning
      get_vocab_list,
      get_vocab_detail,
      get_related_videos,
      toggle_bookmark,
      // Module B: Practice & Anki
      get_daily_cards,
      update_card_progress,
      get_deck_stats,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
