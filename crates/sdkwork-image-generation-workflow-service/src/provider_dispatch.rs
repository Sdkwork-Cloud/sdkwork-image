use sdkwork_image_generation_service::{
    ImageGenerationCommand, ImageGenerationServicePort, ImageProviderDispatchPlan,
    ImageProviderSubmission, NormalizedProviderGenerationResult,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ImageGenerationProviderDispatchError {
    #[error("image generation provider dispatch failed: {0}")]
    Provider(String),
}

pub async fn dispatch_image_generation_provider(
    service: &dyn ImageGenerationServicePort,
    command: ImageGenerationCommand,
) -> Result<ImageProviderSubmission, ImageGenerationProviderDispatchError> {
    service
        .generate(command)
        .await
        .map_err(|error| ImageGenerationProviderDispatchError::Provider(error.to_string()))
}

pub async fn retrieve_image_generation_provider(
    service: &dyn ImageGenerationServicePort,
    dispatch_plan: &ImageProviderDispatchPlan,
    provider_task_id: &str,
) -> Result<NormalizedProviderGenerationResult, ImageGenerationProviderDispatchError> {
    service
        .retrieve(dispatch_plan, provider_task_id)
        .await
        .map_err(|error| ImageGenerationProviderDispatchError::Provider(error.to_string()))
}
