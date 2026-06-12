use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A single message in a conversation. Re-exported from the legacy
/// `t3claw::history::ConversationMessage`; the monolith now re-exports
/// this type so both names refer to the same struct.
#[derive(Debug, Clone)]
pub struct ConversationMessage {
    pub id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}
