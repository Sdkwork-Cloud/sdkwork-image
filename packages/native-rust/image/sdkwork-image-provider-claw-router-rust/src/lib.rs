use clawrouter_open_sdk::{
    MidjourneyImageGenerationRequest, NanoBananaImageGenerationRequest,
    OpenAiImageGenerationRequest, SdkworkAiClient, SdkworkError,
};
use sdkwork_image_core::{
    normalize_openai_image_generation_outputs, normalize_provider_task_generation_result,
    ImageProviderDispatchPlan, ImageProviderOperation, NormalizedProviderGenerationResult,
    OpenAiGeneratedImage, ProviderGeneratedMediaAsset, ProviderTaskErrorSnapshot,
    ProviderTaskSnapshot,
};

pub const CLAW_ROUTER_OPEN_SDK_CRATE: &str = "clawrouter_open_sdk";
pub const CLAW_ROUTER_IMAGE_GENERATION_METHOD: &str = "images.create_generation";

#[derive(Clone)]
pub struct ClawRouterImageProviderGateway {
    client: SdkworkAiClient,
}

impl ClawRouterImageProviderGateway {
    pub fn new(client: SdkworkAiClient) -> Self {
        Self { client }
    }

    pub async fn create_openai_image_generation(
        &self,
        plan: &ImageProviderDispatchPlan,
    ) -> Result<Vec<sdkwork_image_core::GeneratedMediaOutput>, SdkworkError> {
        let request = build_openai_image_generation_request(plan);
        let response = self.client.images().create_generation(&request).await?;
        let images = response
            .data
            .into_iter()
            .map(|image| OpenAiGeneratedImage {
                url: image.url,
                b64_json: image.b64_json,
                mime_type: image.mime_type,
                revised_prompt: image.revised_prompt,
            })
            .collect::<Vec<_>>();
        normalize_openai_image_generation_outputs(&plan.provider_code, images)
            .map_err(|error| sdk_error_from_normalization(error))
    }

    pub async fn create_async_image_generation(
        &self,
        plan: &ImageProviderDispatchPlan,
    ) -> Result<NormalizedProviderGenerationResult, SdkworkError> {
        match plan.provider_operation {
            ImageProviderOperation::MidjourneyImageGeneration => {
                let request = build_midjourney_image_generation_request(plan);
                let task = self
                    .client
                    .images_midjourney()
                    .create_v1_images_generation(&request)
                    .await?;
                normalize_provider_task_generation_result(
                    &plan.provider_code,
                    ProviderTaskSnapshot {
                        task_id: task.task_id,
                        id: task.id,
                        status: task.status,
                        state: task.state,
                        model: task.model,
                        images: provider_assets_from_generated_media(
                            task.images.unwrap_or_default(),
                        ),
                        error: task.error.map(provider_error_from_claw_router),
                    },
                )
                .map_err(|error| sdk_error_from_normalization(error))
            }
            ImageProviderOperation::NanoBananaImageGeneration => {
                let request = build_nano_banana_image_generation_request(plan);
                let task = self
                    .client
                    .images_nano_banana()
                    .create_generations(&request)
                    .await?;
                normalize_provider_task_generation_result(
                    &plan.provider_code,
                    ProviderTaskSnapshot {
                        task_id: task.task_id,
                        id: task.id,
                        status: task.status,
                        state: task.state,
                        model: task.model,
                        images: provider_assets_from_generated_media(
                            task.images.unwrap_or_default(),
                        ),
                        error: task.error.map(provider_error_from_claw_router),
                    },
                )
                .map_err(|error| sdk_error_from_normalization(error))
            }
            _ => {
                if plan.provider_operation == ImageProviderOperation::OpenAiImageGeneration {
                    let outputs = self.create_openai_image_generation(plan).await?;
                    Ok(NormalizedProviderGenerationResult {
                        provider_code: plan.provider_code.clone(),
                        provider_task_id: None,
                        provider_status: Some("succeeded".to_string()),
                        provider_state: None,
                        status: sdkwork_image_core::ImageGenerationRuntimeStatus::Importing,
                        provider_terminal: true,
                        ready_for_drive_import: true,
                        outputs,
                        error_code: None,
                        error_message: None,
                    })
                } else {
                    Err(sdk_error_from_normalization(
                        "image provider operation is not exposed by the generated Claw Router SDK gateway",
                    ))
                }
            }
        }
    }

    pub async fn retrieve_async_image_generation(
        &self,
        plan: &ImageProviderDispatchPlan,
        provider_task_id: &str,
    ) -> Result<NormalizedProviderGenerationResult, SdkworkError> {
        match plan.provider_operation {
            ImageProviderOperation::MidjourneyImageGeneration => {
                let task = self
                    .client
                    .images_midjourney()
                    .list_v1_images_generations(provider_task_id)
                    .await?;
                normalize_provider_task_generation_result(
                    &plan.provider_code,
                    ProviderTaskSnapshot {
                        task_id: task.task_id,
                        id: task.id,
                        status: task.status,
                        state: task.state,
                        model: task.model,
                        images: provider_assets_from_generated_media(
                            task.images.unwrap_or_default(),
                        ),
                        error: task.error.map(provider_error_from_claw_router),
                    },
                )
                .map_err(|error| sdk_error_from_normalization(error))
            }
            ImageProviderOperation::NanoBananaImageGeneration => {
                let task = self
                    .client
                    .images_nano_banana()
                    .retrieve_generations(provider_task_id)
                    .await?;
                normalize_provider_task_generation_result(
                    &plan.provider_code,
                    ProviderTaskSnapshot {
                        task_id: task.task_id,
                        id: task.id,
                        status: task.status,
                        state: task.state,
                        model: task.model,
                        images: provider_assets_from_generated_media(
                            task.images.unwrap_or_default(),
                        ),
                        error: task.error.map(provider_error_from_claw_router),
                    },
                )
                .map_err(|error| sdk_error_from_normalization(error))
            }
            _ => Err(sdk_error_from_normalization(
                "image provider operation does not support task retrieval",
            )),
        }
    }
}

pub fn provider_gateway_supports_create_operation(plan: &ImageProviderDispatchPlan) -> bool {
    matches!(
        plan.provider_operation,
        ImageProviderOperation::OpenAiImageGeneration
            | ImageProviderOperation::MidjourneyImageGeneration
            | ImageProviderOperation::NanoBananaImageGeneration
    )
}

pub fn provider_gateway_supports_retrieve_operation(plan: &ImageProviderDispatchPlan) -> bool {
    matches!(
        plan.provider_operation,
        ImageProviderOperation::MidjourneyImageGeneration
            | ImageProviderOperation::NanoBananaImageGeneration
    )
}

pub fn build_openai_image_generation_request(
    plan: &ImageProviderDispatchPlan,
) -> OpenAiImageGenerationRequest {
    OpenAiImageGenerationRequest {
        model: plan
            .model
            .clone()
            .unwrap_or_else(|| plan.provider_code.clone()),
        n: Some(plan.output_count.into()),
        prompt: plan.prompt.clone(),
        quality: plan.quality.clone(),
        response_format: plan.response_format.clone(),
        size: plan.size.clone(),
    }
}

pub fn build_midjourney_image_generation_request(
    plan: &ImageProviderDispatchPlan,
) -> MidjourneyImageGenerationRequest {
    MidjourneyImageGenerationRequest {
        aspect_ratio: aspect_ratio_from_size(plan.size.as_deref()),
        callback_url: plan.callback_url.clone(),
        model: plan.model.clone(),
        prompt: prompt_with_optional_negative(plan),
        seed: None,
        style: plan.quality.clone(),
    }
}

pub fn build_nano_banana_image_generation_request(
    plan: &ImageProviderDispatchPlan,
) -> NanoBananaImageGenerationRequest {
    NanoBananaImageGenerationRequest {
        aspect_ratio: aspect_ratio_from_size(plan.size.as_deref()),
        callback_url: plan.callback_url.clone(),
        images: None,
        model: plan.model.clone(),
        prompt: prompt_with_optional_negative(plan),
        seed: None,
        size: plan.size.clone(),
    }
}

pub fn openai_image_generation_sdk_supports_output_count() -> bool {
    true
}

fn provider_assets_from_generated_media(
    assets: Vec<clawrouter_open_sdk::ProviderGeneratedMedia>,
) -> Vec<ProviderGeneratedMediaAsset> {
    assets
        .into_iter()
        .map(|asset| ProviderGeneratedMediaAsset {
            id: asset.id,
            uri: asset.uri,
            url: asset.url,
            mime_type: asset.mime_type,
            width: asset.width,
            height: asset.height,
            duration_seconds: asset.duration.map(|value| value.round() as i32),
        })
        .collect()
}

fn provider_error_from_claw_router(
    error: clawrouter_open_sdk::ProviderTaskError,
) -> ProviderTaskErrorSnapshot {
    ProviderTaskErrorSnapshot {
        code: error.code,
        message: error.message,
        error_type: error.r#type,
    }
}

fn prompt_with_optional_negative(plan: &ImageProviderDispatchPlan) -> String {
    match plan.negative_prompt.as_deref() {
        Some(negative_prompt) if !negative_prompt.trim().is_empty() => {
            format!(
                "{}\n\nNegative prompt: {}",
                plan.prompt,
                negative_prompt.trim()
            )
        }
        _ => plan.prompt.clone(),
    }
}

fn aspect_ratio_from_size(size: Option<&str>) -> Option<String> {
    let size = size?.trim();
    let (width, height) = size.split_once('x')?;
    let width = width.trim().parse::<i64>().ok()?;
    let height = height.trim().parse::<i64>().ok()?;
    if width <= 0 || height <= 0 {
        return None;
    }
    let divisor = gcd(width, height);
    Some(format!("{}:{}", width / divisor, height / divisor))
}

fn gcd(mut left: i64, mut right: i64) -> i64 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left.abs().max(1)
}

fn sdk_error_from_normalization(error: &'static str) -> SdkworkError {
    SdkworkError::Serialization(serde_json::Error::io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        error,
    )))
}
