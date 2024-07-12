use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Comments {
    comment_id: u32,
    preset_id: Uuid,
    ip_addr: IpAddr,
    text: String,
    created_on: DateTime<Utc>,
}

impl Comments {
    async fn submit_comment(&self, &conn: &Pool<Postgres>) {
        let query = sqlx::query_as!(
            Comments,
            "
        INSERT INTO comments(preset_id, ip_addr, text, created_on)
        VALUES($1, $2, $3, $4)
      ",
            &self.preset_id,
            &self.ip_addr.to_string(),
            &self.text,
            &self.created_on
        )
        .fetch_optional(&conn);

        match query.await {
            Ok(comment) => ,
            Err(error) => ,
        }
    }

    async fn delete_comment(&self) {}
}
