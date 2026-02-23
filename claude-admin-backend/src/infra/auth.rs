use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Session token store with TTL-based expiry.
#[derive(Clone)]
pub struct TokenStore {
    tokens: Arc<Mutex<HashMap<String, TokenEntry>>>,
    ttl: Duration,
}

struct TokenEntry {
    created_at: Instant,
}

impl TokenStore {
    pub fn new(ttl_hours: u64) -> Self {
        Self {
            tokens: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_hours * 3600),
        }
    }

    /// Create a new session token and store it.
    pub fn create_session(&self) -> (String, chrono::DateTime<chrono::Utc>) {
        let token = generate_token();
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(self.ttl.as_secs() as i64);

        let mut tokens = self.tokens.lock().unwrap();
        tokens.insert(
            token.clone(),
            TokenEntry {
                created_at: Instant::now(),
            },
        );
        (token, expires_at)
    }

    /// Validate a session token. Returns true if valid and not expired.
    pub fn validate(&self, token: &str) -> bool {
        let tokens = self.tokens.lock().unwrap();
        if let Some(entry) = tokens.get(token) {
            entry.created_at.elapsed() < self.ttl
        } else {
            false
        }
    }

    /// Purge expired tokens.
    pub fn purge_expired(&self) -> usize {
        let mut tokens = self.tokens.lock().unwrap();
        let before = tokens.len();
        tokens.retain(|_, entry| entry.created_at.elapsed() < self.ttl);
        before - tokens.len()
    }
}

fn generate_token() -> String {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    // Simple token generation using timestamp + random-ish bits
    format!(
        "ses_{:x}_{:x}",
        seed,
        seed.wrapping_mul(6364136223846793005).wrapping_add(1)
    )
}

/// Spawn background task to purge expired tokens every 5 minutes.
pub fn spawn_token_purge_task(store: TokenStore) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(300));
        loop {
            interval.tick().await;
            let purged = store.purge_expired();
            if purged > 0 {
                tracing::debug!("Purged {} expired session tokens", purged);
            }
        }
    });
}
