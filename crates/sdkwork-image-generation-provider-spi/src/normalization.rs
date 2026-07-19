use crate::model::{
    normalize_provider_code_for_storage, normalized_optional_text, require_trimmed,
};
use crate::{
    GeneratedMediaKind, GeneratedMediaOutput, ImageGenerationRuntimeStatus,
    NormalizedProviderGenerationResult, OpenAiGeneratedImage, ProviderTaskSnapshot,
};

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

    images
        .into_iter()
        .enumerate()
        .map(|(index, image)| {
            let mime_type = normalized_optional_text(image.mime_type.as_deref())
                .or_else(|| infer_mime_type_from_url(image.url.as_deref()))
                .or_else(|| Some("image/png".to_string()));
            let provider_url = normalized_optional_text(image.url.as_deref());
            let provider_uri = normalized_optional_text(image.b64_json.as_deref())
                .map(|base64| {
                    format!(
                        "data:{};base64,{base64}",
                        mime_type.as_deref().unwrap_or("image/png")
                    )
                })
                .or_else(|| {
                    Some(format!(
                        "provider://{}/images/{index}",
                        normalize_provider_code_for_storage(provider_code)
                    ))
                });
            Ok(GeneratedMediaOutput {
                output_index: index as i32,
                kind: GeneratedMediaKind::Image,
                provider_asset_id: None,
                provider_uri,
                provider_url,
                file_name: Some(format!(
                    "generated-{index}.{}",
                    file_extension_for_mime(mime_type.as_deref())
                )),
                mime_type,
                size_bytes: None,
                width: None,
                height: None,
                duration_seconds: None,
            })
        })
        .collect()
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
    let status_text = provider_status
        .as_deref()
        .or(provider_state.as_deref())
        .unwrap_or("running");
    let has_error = task.error.is_some();
    let has_outputs = !task.images.is_empty();
    let status = normalize_provider_status(status_text, has_outputs, has_error);
    let outputs = task
        .images
        .into_iter()
        .enumerate()
        .map(|(index, asset)| {
            let mime_type = normalized_optional_text(asset.mime_type.as_deref())
                .or_else(|| infer_mime_type_from_url(asset.url.as_deref()))
                .or_else(|| Some("image/png".to_string()));
            GeneratedMediaOutput {
                output_index: index as i32,
                kind: GeneratedMediaKind::Image,
                provider_asset_id: normalized_optional_text(asset.id.as_deref()),
                provider_uri: normalized_optional_text(asset.uri.as_deref()).or_else(|| {
                    Some(format!(
                        "provider://{provider_code}/tasks/{}/images/{index}",
                        task.task_id
                            .as_deref()
                            .or(task.id.as_deref())
                            .unwrap_or("unknown")
                    ))
                }),
                provider_url: normalized_optional_text(asset.url.as_deref()),
                file_name: Some(format!(
                    "generated-{index}.{}",
                    file_extension_for_mime(mime_type.as_deref())
                )),
                mime_type,
                size_bytes: None,
                width: i64_to_i32(asset.width),
                height: i64_to_i32(asset.height),
                duration_seconds: asset.duration_seconds,
            }
        })
        .collect::<Vec<_>>();
    let (error_code, error_message) = task.error.map_or((None, None), |error| {
        (
            normalized_optional_text(error.code.as_deref())
                .or_else(|| normalized_optional_text(error.error_type.as_deref())),
            normalized_optional_text(error.message.as_deref()),
        )
    });
    let provider_terminal = matches!(
        status,
        ImageGenerationRuntimeStatus::Importing
            | ImageGenerationRuntimeStatus::Succeeded
            | ImageGenerationRuntimeStatus::Failed
            | ImageGenerationRuntimeStatus::Cancelled
            | ImageGenerationRuntimeStatus::Expired
    );
    Ok(NormalizedProviderGenerationResult {
        provider_code,
        provider_task_id: normalized_optional_text(task.task_id.as_deref())
            .or_else(|| normalized_optional_text(task.id.as_deref())),
        provider_status,
        provider_state,
        status,
        provider_terminal,
        ready_for_drive_import: provider_terminal && !outputs.is_empty() && !has_error,
        outputs,
        error_code,
        error_message,
    })
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
    if matches!(normalized.as_str(), "queued" | "pending") {
        return ImageGenerationRuntimeStatus::Submitted;
    }
    ImageGenerationRuntimeStatus::Rendering
}

fn infer_mime_type_from_url(value: Option<&str>) -> Option<String> {
    let value = value?.trim().to_ascii_lowercase();
    for (suffix, mime_type) in [
        (".jpg", "image/jpeg"),
        (".jpeg", "image/jpeg"),
        (".webp", "image/webp"),
        (".gif", "image/gif"),
        (".png", "image/png"),
    ] {
        if value.ends_with(suffix) {
            return Some(mime_type.to_string());
        }
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
