use std::collections::BTreeSet;

use crate::ImageGenerationProviderError;

#[derive(Clone, Debug, Eq, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(transparent)]
pub struct ImageVendorId(String);

impl ImageVendorId {
    pub fn new(value: impl Into<String>) -> Result<Self, ImageGenerationProviderError> {
        let value = normalize_provider_code_for_storage(&value.into());
        if value.is_empty() {
            return Err(ImageGenerationProviderError::InvalidRequest(
                "vendor is required".to_string(),
            ));
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            return Err(ImageGenerationProviderError::InvalidRequest(
                "vendor must use lowercase letters, digits, or hyphens".to_string(),
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ImageVendorId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageGenerationModelSelection {
    Named(String),
    VendorDefault,
}

impl ImageGenerationModelSelection {
    pub fn named(value: impl Into<String>) -> Result<Self, ImageGenerationProviderError> {
        let value = value.into().trim().to_string();
        if value.is_empty() {
            return Err(ImageGenerationProviderError::InvalidRequest(
                "model is required".to_string(),
            ));
        }
        Ok(Self::Named(value))
    }

    pub fn as_named(&self) -> Option<&str> {
        match self {
            Self::Named(value) => Some(value),
            Self::VendorDefault => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageGenerationGeometry {
    Dimensions { width: u32, height: u32 },
    AspectRatio { width: u32, height: u32 },
    Preset(String),
}

impl ImageGenerationGeometry {
    pub fn provider_value(&self) -> String {
        match self {
            Self::Dimensions { width, height } => format!("{width}x{height}"),
            Self::AspectRatio { width, height } => format!("{width}:{height}"),
            Self::Preset(value) => value.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationReference {
    pub uri: String,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ImageGenerationVendorParameters {
    pub schema: String,
    pub values: serde_json::Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationCommand {
    pub vendor: ImageVendorId,
    pub model: ImageGenerationModelSelection,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub scene: String,
    pub geometry: Option<ImageGenerationGeometry>,
    pub quality: Option<String>,
    pub style: Option<String>,
    pub output_count: i32,
    pub reference_images: Vec<ImageGenerationReference>,
    pub callback_url: Option<String>,
    pub idempotency_key: Option<String>,
    pub vendor_parameters: Option<ImageGenerationVendorParameters>,
}

impl TryFrom<&ImageGenerationCreateCommand> for ImageGenerationCommand {
    type Error = ImageGenerationProviderError;

    fn try_from(command: &ImageGenerationCreateCommand) -> Result<Self, Self::Error> {
        let vendor = command
            .provider_code
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("openai");
        let model = match command.model.as_deref().map(str::trim) {
            Some(value) if !value.is_empty() => ImageGenerationModelSelection::named(value)?,
            _ => ImageGenerationModelSelection::VendorDefault,
        };
        Ok(Self {
            vendor: ImageVendorId::new(vendor)?,
            model,
            prompt: command.prompt.clone(),
            negative_prompt: command.negative_prompt.clone(),
            scene: command.scene.clone(),
            geometry: command
                .resolution
                .as_ref()
                .map(|value| ImageGenerationGeometry::Preset(value.clone())),
            quality: command.style.clone(),
            style: command.style.clone(),
            output_count: command.output_count.unwrap_or(1),
            reference_images: command
                .reference_images
                .iter()
                .cloned()
                .map(|uri| ImageGenerationReference { uri })
                .collect(),
            callback_url: command.webhook_url.clone(),
            idempotency_key: command.idempotency_key.clone(),
            vendor_parameters: None,
        })
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageJobStatus {
    Queued = 1,
    Rendering = 2,
    Ready = 3,
    Failed = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageGenerationRuntimeStatus {
    Queued,
    Dispatching,
    Submitted,
    Rendering,
    Importing,
    Succeeded,
    Failed,
    CancelRequested,
    Cancelled,
    Expired,
}

impl ImageGenerationRuntimeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Dispatching => "dispatching",
            Self::Submitted => "submitted",
            Self::Rendering => "rendering",
            Self::Importing => "importing",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::CancelRequested => "cancel_requested",
            Self::Cancelled => "cancelled",
            Self::Expired => "expired",
        }
    }

    pub fn terminal(&self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Cancelled | Self::Expired
        )
    }

    pub fn as_job_status(&self) -> ImageJobStatus {
        match self {
            Self::Queued | Self::Dispatching | Self::Submitted => ImageJobStatus::Queued,
            Self::Rendering | Self::Importing | Self::CancelRequested => ImageJobStatus::Rendering,
            Self::Succeeded => ImageJobStatus::Ready,
            Self::Failed | Self::Cancelled | Self::Expired => ImageJobStatus::Failed,
        }
    }

    pub fn as_job_status_code(&self) -> i32 {
        self.as_job_status() as i32
    }

    pub fn as_drive_sync_status(&self) -> &'static str {
        match self {
            Self::Importing => "importing",
            Self::Succeeded => "imported",
            Self::Failed | Self::Cancelled | Self::Expired => "failed",
            Self::Queued
            | Self::Dispatching
            | Self::Submitted
            | Self::Rendering
            | Self::CancelRequested => "pending",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageProviderTaskMode {
    Synchronous,
    Task,
    Webhook,
}

impl ImageProviderTaskMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Synchronous => "sync",
            Self::Task => "task",
            Self::Webhook => "webhook",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageProviderOperation {
    OpenAiImageGeneration,
    MidjourneyImageGeneration,
    NanoBananaImageGeneration,
    ViduReferenceToImageGeneration,
    ProviderNativeImageGeneration,
}

impl ImageProviderOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAiImageGeneration => "openai.images.generate",
            Self::MidjourneyImageGeneration => "midjourney.images.generate",
            Self::NanoBananaImageGeneration => "nano_banana.images.generate",
            Self::ViduReferenceToImageGeneration => "vidu.images.reference_to_image",
            Self::ProviderNativeImageGeneration => "provider_native.images.generate",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationCreateCommand {
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub scene: String,
    pub provider_code: Option<String>,
    pub model: Option<String>,
    pub resolution: Option<String>,
    pub style: Option<String>,
    pub output_count: Option<i32>,
    pub reference_images: Vec<String>,
    pub webhook_url: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageProviderDispatchPlan {
    pub provider_id: String,
    pub provider_code: String,
    pub provider_operation: ImageProviderOperation,
    pub task_mode: ImageProviderTaskMode,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub model: Option<String>,
    pub size: Option<String>,
    pub quality: Option<String>,
    pub response_format: Option<String>,
    pub output_count: i32,
    pub output_count_provider_parameter: Option<&'static str>,
    pub reference_images: Vec<String>,
    pub callback_url: Option<String>,
    pub idempotency_key: Option<String>,
    pub vendor_parameters: Option<ImageGenerationVendorParameters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiGeneratedImage {
    pub url: Option<String>,
    pub b64_json: Option<String>,
    pub mime_type: Option<String>,
    pub revised_prompt: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderGeneratedMediaAsset {
    pub id: Option<String>,
    pub uri: Option<String>,
    pub url: Option<String>,
    pub mime_type: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub duration_seconds: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderTaskErrorSnapshot {
    pub code: Option<String>,
    pub message: Option<String>,
    pub error_type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderTaskSnapshot {
    pub task_id: Option<String>,
    pub id: Option<String>,
    pub status: Option<String>,
    pub state: Option<String>,
    pub model: Option<String>,
    pub images: Vec<ProviderGeneratedMediaAsset>,
    pub error: Option<ProviderTaskErrorSnapshot>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeneratedMediaKind {
    Image,
    Video,
    Audio,
    Music,
    Voice,
    Document,
    Other,
}

impl GeneratedMediaKind {
    pub fn as_media_kind(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio | Self::Music => "audio",
            Self::Voice => "voice",
            Self::Document => "document",
            Self::Other => "other",
        }
    }

    pub fn as_drive_upload_profile_code(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio | Self::Music | Self::Voice => "audio",
            Self::Document => "document",
            Self::Other => "generic",
        }
    }

    pub fn default_extension(&self, mime_type: Option<&str>) -> &'static str {
        match mime_type.unwrap_or_default() {
            "image/jpeg" => "jpg",
            "image/webp" => "webp",
            "video/mp4" => "mp4",
            "audio/mpeg" => "mp3",
            "audio/wav" => "wav",
            "application/pdf" => "pdf",
            _ => match self {
                Self::Image => "png",
                Self::Video => "mp4",
                Self::Audio | Self::Music | Self::Voice => "mp3",
                Self::Document | Self::Other => "bin",
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedMediaOutput {
    pub output_index: i32,
    pub kind: GeneratedMediaKind,
    pub provider_asset_id: Option<String>,
    pub provider_uri: Option<String>,
    pub provider_url: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedProviderGenerationResult {
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub provider_state: Option<String>,
    pub status: ImageGenerationRuntimeStatus,
    pub provider_terminal: bool,
    pub ready_for_drive_import: bool,
    pub outputs: Vec<GeneratedMediaOutput>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

pub fn plan_image_generation_provider_dispatch(
    command: &ImageGenerationCreateCommand,
) -> Result<ImageProviderDispatchPlan, &'static str> {
    let command = ImageGenerationCommand::try_from(command)
        .map_err(|_| "invalid image generation command")?;
    plan_unified_image_generation_provider_dispatch(&command)
}

pub fn plan_unified_image_generation_provider_dispatch(
    command: &ImageGenerationCommand,
) -> Result<ImageProviderDispatchPlan, &'static str> {
    let prompt = require_trimmed(&command.prompt, "image generation prompt is required")?;
    let scene = require_trimmed(&command.scene, "image generation scene is required")?;
    validate_scene_code(scene)?;
    let output_count = command.output_count;
    if !(1..=16).contains(&output_count) {
        return Err("image generation output_count must be between 1 and 16");
    }

    let provider_code = normalize_provider_code_for_storage(command.vendor.as_str());
    let model = command.model.as_named().map(str::to_string);
    let size = command
        .geometry
        .as_ref()
        .map(ImageGenerationGeometry::provider_value);
    let quality = normalized_optional_text(command.quality.as_deref())
        .or_else(|| normalized_optional_text(command.style.as_deref()));
    let negative_prompt = normalized_optional_text(command.negative_prompt.as_deref());
    let callback_url = normalized_optional_text(command.callback_url.as_deref());
    let idempotency_key = normalized_optional_text(command.idempotency_key.as_deref());
    let reference_values = command
        .reference_images
        .iter()
        .map(|reference| reference.uri.clone())
        .collect::<Vec<_>>();
    let reference_images = normalize_reference_images(&reference_values)?;

    let (provider_operation, task_mode, output_count_provider_parameter) =
        match provider_code.as_str() {
            "midjourney" => (
                ImageProviderOperation::MidjourneyImageGeneration,
                ImageProviderTaskMode::Task,
                None,
            ),
            "nano-banana" | "nanobanana" => (
                ImageProviderOperation::NanoBananaImageGeneration,
                ImageProviderTaskMode::Task,
                None,
            ),
            "vidu" => {
                if model.is_none() {
                    return Err("vidu image generation model is required");
                }
                if reference_images.is_empty() {
                    return Err(
                        "vidu reference image generation requires at least one reference image",
                    );
                }
                (
                    ImageProviderOperation::ViduReferenceToImageGeneration,
                    ImageProviderTaskMode::Task,
                    None,
                )
            }
            "openai" | "gpt-image" => (
                ImageProviderOperation::OpenAiImageGeneration,
                ImageProviderTaskMode::Synchronous,
                Some("n"),
            ),
            _ => return Err("image generation vendor is not supported by a registered provider"),
        };

    if !reference_images.is_empty()
        && !matches!(
            provider_operation,
            ImageProviderOperation::NanoBananaImageGeneration
                | ImageProviderOperation::ViduReferenceToImageGeneration
        )
    {
        return Err("image generation provider does not support reference_images");
    }

    Ok(ImageProviderDispatchPlan {
        provider_id: String::new(),
        provider_code,
        provider_operation,
        task_mode,
        prompt: prompt.to_string(),
        negative_prompt,
        model,
        size,
        quality,
        response_format: Some("url".to_string()),
        output_count,
        output_count_provider_parameter,
        reference_images,
        callback_url,
        idempotency_key,
        vendor_parameters: command.vendor_parameters.clone(),
    })
}

fn normalize_reference_images(values: &[String]) -> Result<Vec<String>, &'static str> {
    let mut normalized = Vec::new();
    let mut seen = BTreeSet::new();
    for value in values {
        let value = value.trim();
        if value.is_empty() {
            continue;
        }
        if value.len() > 2048 {
            return Err("image generation reference_image must be at most 2048 characters");
        }
        if !seen.insert(value.to_string()) {
            continue;
        }
        normalized.push(value.to_string());
        if normalized.len() > 16 {
            return Err("image generation reference_images must be at most 16 items");
        }
    }
    Ok(normalized)
}

fn validate_scene_code(scene: &str) -> Result<(), &'static str> {
    if scene.len() > 128 {
        return Err("image generation scene must be at most 128 characters");
    }
    if scene.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'@' | b'-')
    }) {
        Ok(())
    } else {
        Err("image generation scene must use visible code characters")
    }
}

pub(crate) fn require_trimmed<'a>(
    value: &'a str,
    error: &'static str,
) -> Result<&'a str, &'static str> {
    let value = value.trim();
    if value.is_empty() {
        Err(error)
    } else {
        Ok(value)
    }
}

pub(crate) fn normalized_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

pub(crate) fn normalize_provider_code_for_storage(value: &str) -> String {
    value.trim().replace('_', "-").to_ascii_lowercase()
}
