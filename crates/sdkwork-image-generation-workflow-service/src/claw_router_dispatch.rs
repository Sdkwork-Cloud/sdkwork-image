use sdkwork_image_claw_router_provider_service::{
    provider_gateway_supports_create_operation, provider_gateway_supports_retrieve_operation,
    ClawRouterImageProviderGateway,
};
use sdkwork_image_generation_service::{
    ImageProviderDispatchPlan, NormalizedProviderGenerationResult,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ClawRouterDispatchError {
    #[error("image provider operation is not supported by the ClawRouter gateway")]
    UnsupportedOperation,
    #[error("claw router gateway failed: {0}")]
    Gateway(String),
}

pub async fn dispatch_image_provider_via_claw_router(
    gateway: &ClawRouterImageProviderGateway,
    dispatch_plan: &ImageProviderDispatchPlan,
) -> Result<NormalizedProviderGenerationResult, ClawRouterDispatchError> {
    if !provider_gateway_supports_create_operation(dispatch_plan) {
        return Err(ClawRouterDispatchError::UnsupportedOperation);
    }
    gateway
        .create_async_image_generation(dispatch_plan)
        .await
        .map_err(|error| ClawRouterDispatchError::Gateway(error.to_string()))
}

pub async fn retrieve_image_provider_via_claw_router(
    gateway: &ClawRouterImageProviderGateway,
    dispatch_plan: &ImageProviderDispatchPlan,
    provider_task_id: &str,
) -> Result<NormalizedProviderGenerationResult, ClawRouterDispatchError> {
    if !provider_gateway_supports_retrieve_operation(dispatch_plan) {
        return Err(ClawRouterDispatchError::UnsupportedOperation);
    }
    gateway
        .retrieve_async_image_generation(dispatch_plan, provider_task_id)
        .await
        .map_err(|error| ClawRouterDispatchError::Gateway(error.to_string()))
}
