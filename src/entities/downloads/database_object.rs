use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub download_id: i32,
    pub preset_id: Uuid,
    pub ip_addr: IpAddr,
    pub created_at: String,
}

impl Download {
    fn download_preset(&self, preset_id: Uuid) {}
}
