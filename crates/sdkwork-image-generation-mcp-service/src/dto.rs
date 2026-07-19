use rmcp::schemars::JsonSchema;
use sdkwork_image_generation_service::{
    GeneratedMediaKind, ImageGenerationCommand, ImageGenerationGeometry,
    ImageGenerationModelSelection, ImageGenerationReference, ImageGenerationVendorParameters,
    ImageProviderSubmission, ImageVendorId, NormalizedProviderGenerationResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::McpToolError;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VendorParametersInput {
    pub schema: String,
    pub values: Value,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ImageGeometryInput {
    Dimensions { width: u32, height: u32 },
    AspectRatio { width: u32, height: u32 },
    Preset { value: String },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateImageInput {
    pub vendor: String,
    #[serde(default)]
    pub model: Option<String>,
    pub prompt: String,
    #[serde(default)]
    pub negative_prompt: Option<String>,
    #[serde(default = "default_scene")]
    pub scene: String,
    #[serde(default)]
    pub geometry: Option<ImageGeometryInput>,
    #[serde(default)]
    pub quality: Option<String>,
    #[serde(default)]
    pub style: Option<String>,
    #[serde(default = "default_output_count")]
    pub output_count: i32,
    #[serde(default)]
    pub reference_images: Vec<String>,
    #[serde(default)]
    pub callback_url: Option<String>,
    #[serde(default)]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub vendor_parameters: Option<VendorParametersInput>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTaskInput {
    pub task_handle: String,
}

#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageGenerationResult {
    pub vendor: String,
    pub task_handle: Option<String>,
    pub status: String,
    pub terminal: bool,
    pub outputs: Vec<ImageOutput>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOutput {
    pub output_index: i32,
    pub media_kind: String,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
}

impl TryFrom<GenerateImageInput> for ImageGenerationCommand {
    type Error = McpToolError;

    fn try_from(input: GenerateImageInput) -> Result<Self, Self::Error> {
        let model = match input.model.as_deref().map(str::trim) {
            Some(value) if !value.is_empty() => {
                ImageGenerationModelSelection::named(value).map_err(McpToolError::from)?
            }
            _ => ImageGenerationModelSelection::VendorDefault,
        };
        let geometry = input.geometry.map(|geometry| match geometry {
            ImageGeometryInput::Dimensions { width, height } => {
                ImageGenerationGeometry::Dimensions { width, height }
            }
            ImageGeometryInput::AspectRatio { width, height } => {
                ImageGenerationGeometry::AspectRatio { width, height }
            }
            ImageGeometryInput::Preset { value } => ImageGenerationGeometry::Preset(value),
        });
        Ok(Self {
            vendor: ImageVendorId::new(input.vendor).map_err(McpToolError::from)?,
            model,
            prompt: input.prompt,
            negative_prompt: input.negative_prompt,
            scene: input.scene,
            geometry,
            quality: input.quality,
            style: input.style,
            output_count: input.output_count,
            reference_images: input
                .reference_images
                .into_iter()
                .map(|uri| ImageGenerationReference { uri })
                .collect(),
            callback_url: input.callback_url,
            idempotency_key: input.idempotency_key,
            vendor_parameters: input.vendor_parameters.map(|parameters| {
                ImageGenerationVendorParameters {
                    schema: parameters.schema,
                    values: parameters.values,
                }
            }),
        })
    }
}

impl ImageGenerationResult {
    pub(crate) fn from_submission(
        submission: &ImageProviderSubmission,
        task_handle: Option<String>,
    ) -> Self {
        Self::from_normalized(&submission.result, task_handle)
    }

    pub(crate) fn from_normalized(
        result: &NormalizedProviderGenerationResult,
        task_handle: Option<String>,
    ) -> Self {
        Self {
            vendor: result.provider_code.clone(),
            task_handle,
            status: result.status.as_str().to_string(),
            terminal: result.provider_terminal,
            outputs: result
                .outputs
                .iter()
                .map(|output| ImageOutput {
                    output_index: output.output_index,
                    media_kind: media_kind(output.kind),
                    url: output.provider_url.clone(),
                    file_name: output.file_name.clone(),
                    mime_type: output.mime_type.clone(),
                    size_bytes: output.size_bytes,
                    width: output.width,
                    height: output.height,
                    duration_seconds: output.duration_seconds,
                })
                .collect(),
            error_code: result.error_code.clone(),
            error_message: result.error_message.clone(),
        }
    }
}

fn media_kind(kind: GeneratedMediaKind) -> String {
    match kind {
        GeneratedMediaKind::Image => "image",
        GeneratedMediaKind::Video => "video",
        GeneratedMediaKind::Audio => "audio",
        GeneratedMediaKind::Music => "music",
        GeneratedMediaKind::Voice => "voice",
        GeneratedMediaKind::Document => "document",
        GeneratedMediaKind::Other => "other",
    }
    .to_string()
}

fn default_scene() -> String {
    "agent.image.generation".to_string()
}

fn default_output_count() -> i32 {
    1
}
