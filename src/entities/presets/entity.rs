use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum_macros::AsRefStr;
use uuid::Uuid;

// Preset Table Database Object/Structure
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PresetDBO {
    pub preset_id: Uuid,
    pub preset_name: String,
    pub created_on: DateTime<Utc>,
    pub last_updated_on: Option<DateTime<Utc>>,
    pub download_url: String,
    pub description: String,
    pub youtube_url: Option<String>,
    pub photo_url: Option<String>,
    pub hardware: Hardware,
    pub views: i32,
    pub downloads: i32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, AsRefStr)]
#[sqlx(type_name = "hardware_type")]
pub enum Hardware {
    Microphone,
    Keyboard,
    Headset,
    Mouse,
    Misc,
}

impl FromStr for Hardware {
    type Err = UnknownHardwareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Microphone" | "microphone" => Ok(Hardware::Microphone),
            "Keyboard" | "keyboard" => Ok(Hardware::Keyboard),
            "Headset" | "headset" => Ok(Hardware::Headset),
            "Misc" | "misc" => Ok(Hardware::Misc),
            _ => return Err(UnknownHardwareError),
        }
    }
}

#[derive(Debug)]
pub struct UnknownHardwareError;

impl Display for UnknownHardwareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown Hardware Type Provided")
    }
}
