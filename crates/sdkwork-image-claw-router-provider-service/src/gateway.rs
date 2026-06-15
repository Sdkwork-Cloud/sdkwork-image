use clawrouter_open_sdk::{SdkworkAiClient, SdkworkError};
use sdkwork_image_generation_service::{
    normalize_openai_image_generation_outputs, ImageGenerationRuntimeStatus,
    ImageProviderDispatchPlan, ImageProviderOperation, NormalizedProviderGenerationResult,
    OpenAiGeneratedImage, ProviderTaskSnapshot,
};

use crate::normalization::{
    normalize_vidu_image_generation_task_result, normalize_vidu_task_creations_result,
    provider_assets_from_generated_media, provider_error_from_claw_router,
};
use crate::requests::{
    build_midjourney_image_generation_request, build_nano_banana_image_generation_request,
    build_openai_image_generation_request, build_vidu_reference_to_image_request,
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
    ) -> Result<Vec<sdkwork_image_generation_service::GeneratedMediaOutput>, SdkworkError> {
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
            .map_err(sdk_error_from_normalization)
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
                sdkwork_image_generation_service::normalize_provider_task_generation_result(
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
                .map_err(sdk_error_from_normalization)
            }
            ImageProviderOperation::NanoBananaImageGeneration => {
                let request = build_nano_banana_image_generation_request(plan);
                let task = self
                    .client
                    .images_nano_banana()
                    .create_generations(&request)
                    .await?;
                sdkwork_image_generation_service::normalize_provider_task_generation_result(
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
                .map_err(sdk_error_from_normalization)
            }
            ImageProviderOperation::ViduReferenceToImageGeneration => {
                let request = build_vidu_reference_to_image_request(plan);
                let task = self
                    .client
                    .images_vidu()
                    .create_ent_v2_reference2image(&request)
                    .await?;
                normalize_vidu_image_generation_task_result(&plan.provider_code, task)
                    .map_err(sdk_error_from_normalization)
            }
            _ => {
                if plan.provider_operation == ImageProviderOperation::OpenAiImageGeneration {
                    let outputs = self.create_openai_image_generation(plan).await?;
                    Ok(NormalizedProviderGenerationResult {
                        provider_code: plan.provider_code.clone(),
                        provider_task_id: None,
                        provider_status: Some("succeeded".to_string()),
                        provider_state: None,
                        status: ImageGenerationRuntimeStatus::Importing,
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
                sdkwork_image_generation_service::normalize_provider_task_generation_result(
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
                .map_err(sdk_error_from_normalization)
            }
            ImageProviderOperation::NanoBananaImageGeneration => {
                let task = self
                    .client
                    .images_nano_banana()
                    .retrieve_generations(provider_task_id)
                    .await?;
                sdkwork_image_generation_service::normalize_provider_task_generation_result(
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
                .map_err(sdk_error_from_normalization)
            }
            ImageProviderOperation::ViduReferenceToImageGeneration => {
                let task = self
                    .client
                    .videos_vidu()
                    .list_ent_v2_tasks_creations(provider_task_id)
                    .await?;
                normalize_vidu_task_creations_result(&plan.provider_code, task)
                    .map_err(sdk_error_from_normalization)
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
            | ImageProviderOperation::ViduReferenceToImageGeneration
    )
}

pub fn provider_gateway_supports_retrieve_operation(plan: &ImageProviderDispatchPlan) -> bool {
    matches!(
        plan.provider_operation,
        ImageProviderOperation::MidjourneyImageGeneration
            | ImageProviderOperation::NanoBananaImageGeneration
            | ImageProviderOperation::ViduReferenceToImageGeneration
    )
}

fn sdk_error_from_normalization(error: &'static str) -> SdkworkError {
    SdkworkError::Serialization(serde_json::Error::io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        error,
    )))
}
