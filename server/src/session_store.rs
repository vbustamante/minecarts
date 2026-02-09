use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use uuid::Uuid;

use crate::models::Session;

#[derive(Clone)]
pub struct SessionStore {
    conn: ConnectionManager,
}

impl SessionStore {
    pub async fn connect(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let conn = client.get_connection_manager().await?;
        Ok(Self { conn })
    }

    fn key(session_id: Uuid) -> String {
        format!("session:{session_id}")
    }

    pub async fn get(&self, session_id: Uuid) -> Result<Option<Session>, redis::RedisError> {
        let mut conn = self.conn.clone();
        let json: Option<String> = conn.get(Self::key(session_id)).await?;
        Ok(json.and_then(|j| serde_json::from_str(&j).ok()))
    }

    pub async fn set(&self, session_id: Uuid, session: &Session) -> Result<(), redis::RedisError> {
        let mut conn = self.conn.clone();
        let json = serde_json::to_string(session).expect("Session serialization cannot fail");
        conn.set(Self::key(session_id), json).await
    }

    pub async fn delete(&self, session_id: Uuid) -> Result<(), redis::RedisError> {
        let mut conn = self.conn.clone();
        conn.del(Self::key(session_id)).await
    }
}
