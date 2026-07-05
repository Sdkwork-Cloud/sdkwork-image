use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationCommandWire {
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    pub scene: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_images: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationRefreshCommandWire {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_task_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationWire {
    pub generation_id: String,
    pub status: String,
    pub scene: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_task_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_space_id: Option<String>,
    pub drive_sync_status: String,
    pub output_asset_count: i32,
    pub outputs: Vec<ImageGenerationOutputWire>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationOutputWire {
    pub output_index: i32,
    pub media_kind: String,
    pub scene: String,
    pub sync_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_asset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_space_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
}
