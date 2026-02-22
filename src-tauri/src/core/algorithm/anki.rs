use crate::core::entity::progress::UserProgress;

/// Anki SM-2 Algorithm
/// quality: 0 (complete failure) to 5 (perfect recall)
/// Returns updated UserProgress with new interval, ease_factor, and next_review_at
pub fn calculate_next_review(progress: &mut UserProgress, quality: u8) {
  let quality = quality.min(5);

  if quality < 3 {
    // Failed: reset to beginning
    progress.interval = 1;
    progress.status = "Learning".to_string();
  } else {
    // Passed: increase interval
    match progress.interval {
      0 | 1 => progress.interval = 1,
      2 => progress.interval = 6,
      _ => {
        progress.interval = (progress.interval as f32 * progress.ease_factor).round() as u32;
      }
    }
    progress.status = if progress.interval >= 21 {
      "Mastered".to_string()
    } else {
      "Review".to_string()
    };
  }

  // Update ease factor (never below 1.3)
  let q = quality as f32;
  progress.ease_factor += 0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02);
  if progress.ease_factor < 1.3 {
    progress.ease_factor = 1.3;
  }

  // Calculate next review timestamp (current time + interval in days)
  let now = std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64;
  progress.next_review_at = now + (progress.interval as i64 * 86400);
}
