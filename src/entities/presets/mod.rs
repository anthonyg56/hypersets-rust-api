pub mod entity;

use crate::router::AppState;
use crate::utils::empty_string_as_none;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
    Extension, Json, Router,
};

use entity::{Hardware, PresetDBO};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use strum_macros::AsRefStr;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/presets/:preset_id",
            get(fetch_one_preset).delete(delete_a_preset),
        )
        .route(
            "/api/presets",
            get(fetch_multiple_presets).post(create_a_preset),
        )
}

#[derive(Deserialize)]
struct PresetsBody {
    preset_name: String,
    download_url: String,
    description: String,
    hardware_type: Hardware,
    photo_url: Option<String>,
    youtube_url: Option<String>,
}

#[derive(Deserialize)]
struct PresetQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    hardware: Option<Hardware>,
    game: Option<String>,
    sort: Option<SortOrder>,
}

#[derive(Deserialize, AsRefStr)]
enum SortOrder {
    MostPopular,
    MostDownloads,
    MostNew,
}

#[derive(Serialize)]
struct PresetResponse {
    error: bool,
    message: String,
    presets: Option<PresetDBO>,
}

#[derive(Serialize)]
struct MultiplePresetResponse {
    error: bool,
    message: String,
    presets: Option<Vec<PresetDBO>>,
}

async fn fetch_one_preset(
    Path(preset_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<PresetResponse>) {
    let preset_uuid = match Uuid::parse_str(&preset_id) {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };

    if preset_uuid == None {
        return (
            StatusCode::BAD_REQUEST,
            Json(PresetResponse {
                error: true,
                message: "Invalid Preset ID".to_string(),
                presets: None,
            }),
        );
    }

    let query = sqlx::query_as!(
        PresetDBO,
        r#"
            SELECT             
                description,
                preset_id,
                preset_name,
                created_on,
                last_updated_on,
                download_url,
                youtube_url,
                photo_url,
                hardware as "hardware!: Hardware",
                views,
                downloads
            FROM presets
            WHERE preset_id = $1
        "#,
        preset_uuid
    )
    .fetch_one(&state.pool);

    match query.await {
        Ok(preset) => (
            StatusCode::OK,
            Json(PresetResponse {
                error: false,
                message: "Successfully found presets".to_string(),
                presets: Some(preset),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PresetResponse {
                error: true,
                message: "There was an error".to_string(),
                presets: None,
            }),
        ),
    }
}

async fn fetch_multiple_presets(
    Query(params): Query<PresetQueryParams>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<MultiplePresetResponse>) {
    let sort_order: SortOrder = match params.sort {
        Some(SortOrder::MostNew) => SortOrder::MostNew,
        Some(SortOrder::MostPopular) => SortOrder::MostPopular,
        Some(SortOrder::MostDownloads) => SortOrder::MostDownloads,
        None => {
            return (
                StatusCode::OK,
                Json(MultiplePresetResponse {
                    error: true,
                    message: "Please provide a sort order".to_string(),
                    presets: None,
                }),
            )
        }
    };

    let query = sqlx::query_as!(
        PresetDBO,
        r#"
            SELECT             
                description,
                preset_id,
                preset_name,
                created_on,
                last_updated_on,
                download_url,
                youtube_url,
                photo_url,
                hardware as "hardware!: Hardware",
                views,
                downloads
            FROM presets
        "#,
    )
    .fetch_all(&state.pool);

    match query.await {
        Ok(preset) => (
            StatusCode::OK,
            Json(MultiplePresetResponse {
                error: false,
                message: "Successfully found presets".to_string(),
                presets: Some(preset),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MultiplePresetResponse {
                error: true,
                message: "There was an error".to_string(),
                presets: None,
            }),
        ),
    }
}

#[axum::debug_handler]
async fn create_a_preset(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<PresetsBody>,
) -> (StatusCode, Json<MultiplePresetResponse>) {
    let hardware: Hardware = payload.hardware_type.into();

    let query = sqlx::query_as!(
        PresetDBO,
        r#"
          INSERT INTO presets(preset_name, download_url, description, hardware, photo_url, youtube_url)
          VALUES($1, $2, $3, $4, $5, $6)
          RETURNING
            description,
            preset_id,
            preset_name,
            created_on,
            last_updated_on,
            download_url,
            youtube_url,
            photo_url,
            hardware as "hardware!: Hardware",
            views,
            downloads
        "#,
        payload.preset_name,
        payload.download_url,
        payload.description,
        hardware as Hardware,
        payload.photo_url,
        payload.youtube_url
    )
    .fetch_all(&state.pool);

    match query.await {
        Ok(presets) => (
            StatusCode::OK,
            Json(MultiplePresetResponse {
                error: false,
                message: "Successfully inserted preset".to_string(),
                presets: Some(presets),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MultiplePresetResponse {
                error: true,
                message: "There was an error".to_string(),
                presets: None,
            }),
        ),
    }
}

async fn delete_a_preset(
    Path(preset_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> (StatusCode, Json<PresetResponse>) {
    let preset_uuid = match Uuid::parse_str(&preset_id) {
        Ok(uuid) => Some(uuid),
        Err(_) => None,
    };

    if preset_uuid == None {
        return (
            StatusCode::BAD_REQUEST,
            Json(PresetResponse {
                error: true,
                message: "Invalid Preset ID".to_string(),
                presets: None,
            }),
        );
    }

    let query = sqlx::query("DELETE FROM presets WHERE preset_id = $1")
        .bind(preset_uuid)
        .fetch_optional(&state.pool);

    match query.await {
        Ok(_) => (
            StatusCode::OK,
            Json(PresetResponse {
                error: false,
                message: "Successfully removed preset".to_string(),
                presets: None,
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(PresetResponse {
                error: true,
                message: "There was an error".to_string(),
                presets: None,
            }),
        ),
    }
}
