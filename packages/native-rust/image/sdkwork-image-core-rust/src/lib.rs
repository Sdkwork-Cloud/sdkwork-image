use std::collections::{BTreeMap, BTreeSet};

use sdkwork_drive_product::{
    domain::space::DriveSpaceType,
    uploader::{PrepareUploaderUploadCommand, UploaderActor, UploaderRetention, UploaderTarget},
};

pub const IMAGE_WORKSPACE: &str = "sdkwork-image";
pub const IMAGE_DOMAIN: &str = "image";
pub const IMAGE_CAPABILITY: &str = "image";
pub const GENERATED_MEDIA_DEFAULT_CHUNK_SIZE_BYTES: i64 = 8 * 1024 * 1024;

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageJobStatus {
    Queued = 1,
    Rendering = 2,
    Ready = 3,
    Failed = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageVisibility {
    Private = 1,
    Tenant = 2,
    Public = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationRequest {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub prompt: String,
    pub resolution: String,
    pub style: String,
    pub visibility: ImageVisibility,
    pub status: ImageJobStatus,
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
    ProviderNativeImageGeneration,
}

impl ImageProviderOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAiImageGeneration => "openai.images.generate",
            Self::MidjourneyImageGeneration => "midjourney.images.generate",
            Self::NanoBananaImageGeneration => "nano_banana.images.generate",
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
    pub webhook_url: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageProviderDispatchPlan {
    pub provider_code: String,
    pub provider_operation: ImageProviderOperation,
    pub task_mode: ImageProviderTaskMode,
    pub claw_router_api_path: &'static str,
    pub claw_router_sdk_resource: &'static str,
    pub claw_router_sdk_method: &'static str,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub model: Option<String>,
    pub size: Option<String>,
    pub quality: Option<String>,
    pub response_format: Option<String>,
    pub output_count: i32,
    pub output_count_provider_parameter: Option<&'static str>,
    pub callback_url: Option<String>,
    pub idempotency_key: Option<String>,
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
    let prompt = require_trimmed(&command.prompt, "image generation prompt is required")?;
    let scene = require_trimmed(&command.scene, "image generation scene is required")?;
    validate_scene_code(scene)?;
    let provider_code = command
        .provider_code
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("openai")
        .to_ascii_lowercase();
    let output_count = command.output_count.unwrap_or(1);
    if !(1..=16).contains(&output_count) {
        return Err("image generation output_count must be between 1 and 16");
    }

    let model = normalized_optional_text(command.model.as_deref());
    let size = normalized_optional_text(command.resolution.as_deref());
    let quality = normalized_optional_text(command.style.as_deref());
    let negative_prompt = normalized_optional_text(command.negative_prompt.as_deref());
    let callback_url = normalized_optional_text(command.webhook_url.as_deref());
    let idempotency_key = normalized_optional_text(command.idempotency_key.as_deref());

    let (
        provider_operation,
        task_mode,
        claw_router_api_path,
        claw_router_sdk_resource,
        claw_router_sdk_method,
    ) = match provider_code.as_str() {
        "midjourney" => (
            ImageProviderOperation::MidjourneyImageGeneration,
            ImageProviderTaskMode::Task,
            "/midjourney/v1/images/generations",
            "images_midjourney",
            "create_v1_images_generation",
        ),
        "nano-banana" | "nano_banana" | "nanobanana" => (
            ImageProviderOperation::NanoBananaImageGeneration,
            ImageProviderTaskMode::Task,
            "/nano-banana/v1/images/generations",
            "images_nano_banana",
            "create_generations",
        ),
        "openai" | "gpt-image" | "gpt_image" => (
            ImageProviderOperation::OpenAiImageGeneration,
            ImageProviderTaskMode::Synchronous,
            "/v1/images/generations",
            "images",
            "create_generation",
        ),
        "gemini" | "kling" | "jimeng" | "volcengine" => (
            ImageProviderOperation::ProviderNativeImageGeneration,
            ImageProviderTaskMode::Task,
            "/v1/images/generations",
            "images",
            "create_generation",
        ),
        _ => (
            ImageProviderOperation::ProviderNativeImageGeneration,
            if callback_url.is_some() {
                ImageProviderTaskMode::Webhook
            } else {
                ImageProviderTaskMode::Task
            },
            "/v1/images/generations",
            "images",
            "create_generation",
        ),
    };

    Ok(ImageProviderDispatchPlan {
        provider_code: normalize_provider_code_for_storage(&provider_code),
        provider_operation,
        task_mode,
        claw_router_api_path,
        claw_router_sdk_resource,
        claw_router_sdk_method,
        prompt: prompt.to_string(),
        negative_prompt,
        model,
        size,
        quality,
        response_format: Some("url".to_string()),
        output_count,
        output_count_provider_parameter: Some("n"),
        callback_url,
        idempotency_key,
    })
}

pub fn normalize_openai_image_generation_outputs(
    provider_code: impl AsRef<str>,
    images: Vec<OpenAiGeneratedImage>,
) -> Result<Vec<GeneratedMediaOutput>, &'static str> {
    let provider_code = require_trimmed(
        provider_code.as_ref(),
        "image generation provider_code is required",
    )?;
    if images.is_empty() {
        return Err("image generation provider outputs are required");
    }

    let mut outputs = Vec::with_capacity(images.len());
    for (index, image) in images.into_iter().enumerate() {
        let mime_type = normalized_optional_text(image.mime_type.as_deref())
            .or_else(|| infer_mime_type_from_url(image.url.as_deref()))
            .or_else(|| Some("image/png".to_string()));
        let provider_url = normalized_optional_text(image.url.as_deref());
        let provider_uri = match normalized_optional_text(image.b64_json.as_deref()) {
            Some(b64_json) => {
                let mime = mime_type.as_deref().unwrap_or("image/png");
                Some(format!("data:{mime};base64,{b64_json}"))
            }
            None => Some(format!(
                "provider://{}/images/{}",
                normalize_provider_code_for_storage(provider_code),
                index
            )),
        };

        outputs.push(GeneratedMediaOutput {
            output_index: index as i32,
            kind: GeneratedMediaKind::Image,
            provider_asset_id: None,
            provider_uri,
            provider_url,
            file_name: Some(format!(
                "generated-{}.{}",
                index,
                file_extension_for_mime(mime_type.as_deref())
            )),
            mime_type,
            size_bytes: None,
            width: None,
            height: None,
            duration_seconds: None,
        });
    }

    Ok(outputs)
}

pub fn normalize_provider_task_generation_result(
    provider_code: impl AsRef<str>,
    task: ProviderTaskSnapshot,
) -> Result<NormalizedProviderGenerationResult, &'static str> {
    let provider_code = normalize_provider_code_for_storage(require_trimmed(
        provider_code.as_ref(),
        "image generation provider_code is required",
    )?);
    let provider_status = normalized_optional_text(task.status.as_deref());
    let provider_state = normalized_optional_text(task.state.as_deref());
    let normalized_state = provider_status
        .as_deref()
        .or(provider_state.as_deref())
        .unwrap_or("submitted");
    let status = normalize_provider_status(
        normalized_state,
        !task.images.is_empty(),
        task.error.is_some(),
    );
    let provider_terminal = matches!(
        status,
        ImageGenerationRuntimeStatus::Importing
            | ImageGenerationRuntimeStatus::Succeeded
            | ImageGenerationRuntimeStatus::Failed
            | ImageGenerationRuntimeStatus::Cancelled
            | ImageGenerationRuntimeStatus::Expired
    );
    let ready_for_drive_import = matches!(
        status,
        ImageGenerationRuntimeStatus::Importing | ImageGenerationRuntimeStatus::Succeeded
    ) && !task.images.is_empty();

    let outputs = task
        .images
        .into_iter()
        .enumerate()
        .map(|(index, asset)| GeneratedMediaOutput {
            output_index: index as i32,
            kind: GeneratedMediaKind::Image,
            provider_asset_id: normalized_optional_text(asset.id.as_deref()),
            provider_uri: normalized_optional_text(asset.uri.as_deref()).or_else(|| {
                Some(format!(
                    "provider://{}/tasks/{}/images/{}",
                    provider_code,
                    task.task_id
                        .as_deref()
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .unwrap_or("unknown"),
                    index
                ))
            }),
            provider_url: normalized_optional_text(asset.url.as_deref()),
            file_name: Some(format!(
                "generated-{}.{}",
                index,
                file_extension_for_mime(asset.mime_type.as_deref())
            )),
            mime_type: normalized_optional_text(asset.mime_type.as_deref())
                .or_else(|| infer_mime_type_from_url(asset.url.as_deref()))
                .or_else(|| Some("image/png".to_string())),
            size_bytes: None,
            width: i64_to_i32(asset.width),
            height: i64_to_i32(asset.height),
            duration_seconds: asset.duration_seconds,
        })
        .collect::<Vec<_>>();

    let (error_code, error_message) = match task.error {
        Some(error) => (
            normalized_optional_text(error.code.as_deref())
                .or_else(|| normalized_optional_text(error.error_type.as_deref())),
            normalized_optional_text(error.message.as_deref()),
        ),
        None => (None, None),
    };

    Ok(NormalizedProviderGenerationResult {
        provider_code,
        provider_task_id: normalized_optional_text(task.task_id.as_deref())
            .or_else(|| normalized_optional_text(task.id.as_deref())),
        provider_status,
        provider_state,
        status,
        provider_terminal,
        ready_for_drive_import,
        outputs,
        error_code,
        error_message,
    })
}

pub fn create_image_generation_request(
    tenant_id: impl Into<String>,
    organization_id: Option<&str>,
    prompt: impl Into<String>,
    resolution: impl Into<String>,
    style: impl Into<String>,
    visibility: ImageVisibility,
) -> ImageGenerationRequest {
    ImageGenerationRequest {
        tenant_id: tenant_id.into(),
        organization_id: organization_id.map(str::to_string),
        prompt: prompt.into(),
        resolution: resolution.into(),
        style: style.into(),
        visibility,
        status: ImageJobStatus::Queued,
    }
}

pub fn validate_image_generation_request(
    request: &ImageGenerationRequest,
) -> Result<(), &'static str> {
    if request.tenant_id.trim().is_empty() {
        return Err("image generation tenant is required");
    }

    if request.prompt.trim().is_empty() {
        return Err("image generation prompt is required");
    }

    if request.style.trim().is_empty() {
        return Err("image generation style is required");
    }

    let Some((width, height)) = request.resolution.split_once('x') else {
        return Err("image generation resolution must use WIDTHxHEIGHT");
    };

    if parse_positive_dimension(width).is_none() || parse_positive_dimension(height).is_none() {
        return Err("image generation resolution must use WIDTHxHEIGHT");
    }

    Ok(())
}

fn parse_positive_dimension(value: &str) -> Option<u32> {
    let dimension = value.trim().parse::<u32>().ok()?;
    (dimension > 0).then_some(dimension)
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

fn normalized_optional_text(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn normalize_provider_code_for_storage(value: &str) -> String {
    value.trim().replace('_', "-").to_ascii_lowercase()
}

fn normalize_provider_status(
    status: &str,
    has_outputs: bool,
    has_error: bool,
) -> ImageGenerationRuntimeStatus {
    let normalized = status.trim().to_ascii_lowercase();
    if has_error {
        return ImageGenerationRuntimeStatus::Failed;
    }
    if matches!(
        normalized.as_str(),
        "succeeded" | "success" | "completed" | "complete" | "done" | "finished"
    ) {
        return if has_outputs {
            ImageGenerationRuntimeStatus::Importing
        } else {
            ImageGenerationRuntimeStatus::Succeeded
        };
    }
    if matches!(
        normalized.as_str(),
        "failed" | "failure" | "error" | "rejected" | "blocked"
    ) {
        return ImageGenerationRuntimeStatus::Failed;
    }
    if matches!(normalized.as_str(), "cancelled" | "canceled") {
        return ImageGenerationRuntimeStatus::Cancelled;
    }
    if matches!(normalized.as_str(), "expired" | "timeout" | "timed_out") {
        return ImageGenerationRuntimeStatus::Expired;
    }
    if matches!(
        normalized.as_str(),
        "running" | "processing" | "rendering" | "in_progress"
    ) {
        return ImageGenerationRuntimeStatus::Rendering;
    }
    if matches!(normalized.as_str(), "queued" | "pending") {
        return ImageGenerationRuntimeStatus::Submitted;
    }
    ImageGenerationRuntimeStatus::Rendering
}

fn infer_mime_type_from_url(value: Option<&str>) -> Option<String> {
    let value = value?.trim().to_ascii_lowercase();
    if value.ends_with(".jpg") || value.ends_with(".jpeg") {
        return Some("image/jpeg".to_string());
    }
    if value.ends_with(".webp") {
        return Some("image/webp".to_string());
    }
    if value.ends_with(".gif") {
        return Some("image/gif".to_string());
    }
    if value.ends_with(".png") {
        return Some("image/png".to_string());
    }
    None
}

fn file_extension_for_mime(value: Option<&str>) -> &'static str {
    match value
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "image/jpeg" => "jpg",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "video/mp4" => "mp4",
        "audio/mpeg" => "mp3",
        _ => "png",
    }
}

fn i64_to_i32(value: Option<i64>) -> Option<i32> {
    value.and_then(|value| i32::try_from(value).ok())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageGenerationActor {
    Anonymous { anonymous_id: String },
    User { user_id: String },
    System { operator_id: String },
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

    fn default_extension(&self, mime_type: Option<&str>) -> &'static str {
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
                Self::Document => "bin",
                Self::Other => "bin",
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveGeneratedMediaContext {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub generation_id: String,
    pub provider_code: String,
    pub model: Option<String>,
    pub scene: String,
    pub actor: ImageGenerationActor,
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
pub struct MediaAiProvenance {
    pub provenance: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub generation_task_id: Option<String>,
    pub moderation_status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveBackedMediaResource {
    pub id: String,
    pub kind: String,
    pub source: String,
    pub uri: String,
    pub url: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub object_blob_id: Option<String>,
    pub ai: MediaAiProvenance,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriveGeneratedMediaImportPlan {
    pub generation_id: String,
    pub output_index: i32,
    pub scene: String,
    pub provider_code: String,
    pub provider_asset_id: Option<String>,
    pub provider_uri: Option<String>,
    pub provider_url: Option<String>,
    pub drive_space_type: String,
    pub drive_owner_subject_type: String,
    pub drive_owner_subject_id: String,
    pub drive_actor_type: String,
    pub drive_actor_id: String,
    pub drive_space_id: String,
    pub drive_parent_node_id: Option<String>,
    pub drive_node_id: String,
    pub drive_uri: String,
    pub drive_upload_profile_code: String,
    pub drive_upload_task_id: String,
    pub media_resource: DriveBackedMediaResource,
}

pub fn plan_drive_import_for_generated_outputs(
    context: DriveGeneratedMediaContext,
    outputs: Vec<GeneratedMediaOutput>,
) -> Result<Vec<DriveGeneratedMediaImportPlan>, &'static str> {
    require_trimmed(&context.tenant_id, "generated media tenant is required")?;
    let generation_id = require_trimmed(
        &context.generation_id,
        "generated media generation_id is required",
    )?;
    let provider_code = require_trimmed(
        &context.provider_code,
        "generated media provider_code is required",
    )?;
    let scene = require_trimmed(&context.scene, "generated media scene is required")?;
    if outputs.is_empty() {
        return Err("generated media outputs are required");
    }

    let mut output_indexes = BTreeSet::new();
    for output in &outputs {
        if output.output_index < 0 {
            return Err("generated media output_index must be non-negative");
        }
        if !output_indexes.insert(output.output_index) {
            return Err("generated media output_index must be unique");
        }
    }

    let owner = resolve_drive_owner(&context.actor)?;
    let drive_space_type = DriveSpaceType::AiGenerated.as_str().to_string();
    let owner_suffix = stable_identifier_suffix(&owner.owner_subject_id);
    let drive_space_id = format!(
        "space-ai-generated-{}-{}",
        owner.owner_subject_type, owner_suffix
    );

    let mut plans = Vec::with_capacity(outputs.len());
    for output in outputs {
        let mime_type = output
            .mime_type
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let file_name = output
            .file_name
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| {
                format!(
                    "{}-{}.{}",
                    stable_identifier_suffix(generation_id),
                    output.output_index,
                    output.kind.default_extension(mime_type)
                )
            });
        let drive_node_id = format!(
            "node-ai-generated-{}-{}",
            stable_identifier_suffix(generation_id),
            output.output_index
        );
        let drive_uri = format!("drive://spaces/{drive_space_id}/nodes/{drive_node_id}");
        let drive_upload_task_id = format!(
            "image-generation-{}-{}",
            stable_identifier_suffix(generation_id),
            output.output_index
        );
        let mut metadata = BTreeMap::new();
        metadata.insert("spaceType".to_string(), drive_space_type.clone());
        metadata.insert("spaceId".to_string(), drive_space_id.clone());
        metadata.insert("nodeId".to_string(), drive_node_id.clone());
        metadata.insert("scene".to_string(), scene.to_string());
        metadata.insert("provider".to_string(), provider_code.to_string());
        metadata.insert("generationId".to_string(), generation_id.to_string());
        metadata.insert("outputIndex".to_string(), output.output_index.to_string());
        if let Some(organization_id) = context
            .organization_id
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            metadata.insert("organizationId".to_string(), organization_id.to_string());
        }
        if let Some(provider_asset_id) = output
            .provider_asset_id
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            metadata.insert("providerAssetId".to_string(), provider_asset_id.to_string());
        }
        if let Some(provider_uri) = output
            .provider_uri
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            metadata.insert("providerUri".to_string(), provider_uri.to_string());
        }

        let media_resource = DriveBackedMediaResource {
            id: drive_node_id.clone(),
            kind: output.kind.as_media_kind().to_string(),
            source: "drive".to_string(),
            uri: drive_uri.clone(),
            url: None,
            file_name: Some(file_name),
            mime_type: mime_type.map(str::to_string),
            size_bytes: output.size_bytes.map(|value| value.to_string()),
            width: output.width,
            height: output.height,
            duration_seconds: output.duration_seconds,
            object_blob_id: None,
            ai: MediaAiProvenance {
                provenance: "generated".to_string(),
                provider: Some(provider_code.to_string()),
                model: context
                    .model
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string),
                generation_task_id: Some(generation_id.to_string()),
                moderation_status: "unknown".to_string(),
            },
            metadata,
        };

        plans.push(DriveGeneratedMediaImportPlan {
            generation_id: generation_id.to_string(),
            output_index: output.output_index,
            scene: scene.to_string(),
            provider_code: provider_code.to_string(),
            provider_asset_id: output.provider_asset_id,
            provider_uri: output.provider_uri,
            provider_url: output.provider_url,
            drive_space_type: drive_space_type.clone(),
            drive_owner_subject_type: owner.owner_subject_type.clone(),
            drive_owner_subject_id: owner.owner_subject_id.clone(),
            drive_actor_type: owner.actor_type.clone(),
            drive_actor_id: owner.actor_id.clone(),
            drive_space_id: drive_space_id.clone(),
            drive_parent_node_id: None,
            drive_node_id,
            drive_uri,
            drive_upload_profile_code: output.kind.as_drive_upload_profile_code().to_string(),
            drive_upload_task_id,
            media_resource,
        });
    }

    Ok(plans)
}

pub fn build_drive_uploader_command_for_generated_output(
    plan: &DriveGeneratedMediaImportPlan,
    tenant_id: impl AsRef<str>,
    organization_id: Option<&str>,
    operator_id: impl AsRef<str>,
    now_epoch_ms: i64,
) -> Result<PrepareUploaderUploadCommand, &'static str> {
    let tenant_id = require_trimmed(tenant_id.as_ref(), "generated media tenant is required")?;
    let operator_id = require_trimmed(
        operator_id.as_ref(),
        "generated media drive operator_id is required",
    )?;
    if now_epoch_ms <= 0 {
        return Err("generated media drive now_epoch_ms must be greater than 0");
    }
    let scene = require_trimmed(&plan.scene, "generated media scene is required")?;
    let upload_id = plan
        .drive_node_id
        .strip_prefix("node-")
        .unwrap_or(&plan.drive_node_id)
        .to_string();
    let file_name = plan
        .media_resource
        .file_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or("generated media drive file_name is required")?
        .to_string();
    let content_type = plan
        .media_resource
        .mime_type
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
        .unwrap_or_else(|| {
            default_content_type_for_profile(&plan.drive_upload_profile_code).to_string()
        });

    Ok(PrepareUploaderUploadCommand {
        id: upload_id,
        task_id: plan.drive_upload_task_id.clone(),
        tenant_id: tenant_id.to_string(),
        organization_id: organization_id
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string),
        actor: uploader_actor_for_drive_plan(plan)?,
        app_id: IMAGE_WORKSPACE.to_string(),
        app_resource_type: "ai_generation_output".to_string(),
        app_resource_id: format!("{}:{}", plan.generation_id, plan.output_index),
        scene: Some(scene.to_string()),
        source: Some("ai_generated".to_string()),
        upload_profile_code: plan.drive_upload_profile_code.clone(),
        file_fingerprint: format!(
            "sdkwork-image:ai-generated:{}:{}",
            stable_identifier_suffix(&plan.generation_id),
            plan.output_index
        ),
        original_file_name: file_name,
        content_type,
        content_length: parse_content_length(plan.media_resource.size_bytes.as_deref())?,
        chunk_size_bytes: GENERATED_MEDIA_DEFAULT_CHUNK_SIZE_BYTES,
        target: UploaderTarget::AiGeneratedSpace {
            parent_node_id: plan.drive_parent_node_id.clone(),
        },
        retention: UploaderRetention::LongTerm,
        operator_id: operator_id.to_string(),
        now_epoch_ms,
    })
}

struct DriveOwner {
    owner_subject_type: String,
    owner_subject_id: String,
    actor_type: String,
    actor_id: String,
}

fn resolve_drive_owner(actor: &ImageGenerationActor) -> Result<DriveOwner, &'static str> {
    match actor {
        ImageGenerationActor::User { user_id } => {
            let user_id = require_trimmed(user_id, "generated media user_id is required")?;
            Ok(DriveOwner {
                owner_subject_type: "user".to_string(),
                owner_subject_id: user_id.to_string(),
                actor_type: "user".to_string(),
                actor_id: user_id.to_string(),
            })
        }
        ImageGenerationActor::Anonymous { anonymous_id } => {
            let anonymous_id =
                require_trimmed(anonymous_id, "generated media anonymous_id is required")?;
            Ok(DriveOwner {
                owner_subject_type: "app".to_string(),
                owner_subject_id: format!("app:{IMAGE_WORKSPACE}:anonymous"),
                actor_type: "anonymous".to_string(),
                actor_id: anonymous_id.to_string(),
            })
        }
        ImageGenerationActor::System { operator_id } => {
            let operator_id =
                require_trimmed(operator_id, "generated media operator_id is required")?;
            Ok(DriveOwner {
                owner_subject_type: "app".to_string(),
                owner_subject_id: format!("app:{IMAGE_WORKSPACE}:system"),
                actor_type: "system".to_string(),
                actor_id: operator_id.to_string(),
            })
        }
    }
}

fn uploader_actor_for_drive_plan(
    plan: &DriveGeneratedMediaImportPlan,
) -> Result<UploaderActor, &'static str> {
    match plan.drive_actor_type.as_str() {
        "user" => Ok(UploaderActor::User {
            user_id: require_trimmed(
                &plan.drive_actor_id,
                "generated media drive user_id is required",
            )?
            .to_string(),
        }),
        "anonymous" => Ok(UploaderActor::Anonymous {
            anonymous_id: require_trimmed(
                &plan.drive_actor_id,
                "generated media drive anonymous_id is required",
            )?
            .to_string(),
        }),
        "system" => Ok(UploaderActor::System {
            operator_id: require_trimmed(
                &plan.drive_actor_id,
                "generated media drive system operator_id is required",
            )?
            .to_string(),
        }),
        _ => Err("generated media drive actor_type is not supported"),
    }
}

fn default_content_type_for_profile(upload_profile_code: &str) -> &'static str {
    match upload_profile_code {
        "image" => "image/png",
        "video" => "video/mp4",
        "audio" => "audio/mpeg",
        "document" => "application/octet-stream",
        _ => "application/octet-stream",
    }
}

fn parse_content_length(value: Option<&str>) -> Result<i64, &'static str> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(0);
    };
    let parsed = value
        .parse::<i64>()
        .map_err(|_| "generated media size_bytes must be a non-negative integer")?;
    if parsed < 0 {
        Err("generated media size_bytes must be a non-negative integer")
    } else {
        Ok(parsed)
    }
}

fn require_trimmed<'a>(value: &'a str, error: &'static str) -> Result<&'a str, &'static str> {
    let value = value.trim();
    if value.is_empty() {
        Err(error)
    } else {
        Ok(value)
    }
}

fn stable_identifier_suffix(value: &str) -> String {
    let normalized = value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    let suffix = normalized
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let suffix = suffix.chars().take(80).collect::<String>();
    if suffix.is_empty() {
        "unknown".to_string()
    } else {
        suffix
    }
}
