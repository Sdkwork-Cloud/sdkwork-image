use sdkwork_image_generation_service::{
    ImageGenerationCommand, ImageGenerationCreateCommand, ImageGenerationServicePort,
    ImageProviderDispatchPlan, NormalizedProviderGenerationResult,
};
use sdkwork_image_generation_workflow_service::{
    dispatch_image_generation_provider, plan_generation_create_service_flow_with_dispatch,
    plan_generation_refresh_from_provider_result, retrieve_image_generation_provider,
    ImageGenerationScope, ImageGenerationServicePlan,
};

use sdkwork_assets_ingestion::DriveImportPlan;

use crate::drive_import::{plan_drive_upload_preparations, ImageDriveUploadPreparation};
use crate::ImageRuntimeError;

#[derive(Clone, Debug)]
pub struct ImageGenerationCreateRuntimeInput {
    pub scope: ImageGenerationScope,
    pub generation_id: String,
    pub command: ImageGenerationCreateCommand,
    pub operator_id: String,
    pub now_epoch_ms: i64,
}

#[derive(Clone, Debug)]
pub struct ImageGenerationCreateRuntimeResult {
    pub dispatch_plan: ImageProviderDispatchPlan,
    pub provider_result: NormalizedProviderGenerationResult,
    pub service_plan: ImageGenerationServicePlan,
    pub import_plan: DriveImportPlan,
    pub upload_preparations: Vec<ImageDriveUploadPreparation>,
}

#[derive(Clone, Debug)]
pub struct ImageGenerationRefreshRuntimeInput {
    pub scope: ImageGenerationScope,
    pub generation_id: String,
    pub scene: String,
    pub model: Option<String>,
    pub dispatch_plan: ImageProviderDispatchPlan,
    pub provider_task_id: String,
    pub operator_id: String,
    pub now_epoch_ms: i64,
}

#[derive(Clone, Debug)]
pub struct ImageGenerationRefreshRuntimeResult {
    pub provider_result: NormalizedProviderGenerationResult,
    pub import_plan: DriveImportPlan,
    pub upload_preparations: Vec<ImageDriveUploadPreparation>,
}

pub async fn execute_create_generation_dispatch(
    provider_service: &dyn ImageGenerationServicePort,
    input: ImageGenerationCreateRuntimeInput,
) -> Result<ImageGenerationCreateRuntimeResult, ImageRuntimeError> {
    let provider_command = ImageGenerationCommand::try_from(&input.command)
        .map_err(|_| ImageRuntimeError::Validation("invalid image generation provider command"))?;
    let submission = dispatch_image_generation_provider(provider_service, provider_command).await?;
    let dispatch_plan = submission.dispatch_plan;
    let provider_result = submission.result;
    let service_plan = plan_generation_create_service_flow_with_dispatch(
        input.scope.clone(),
        input.generation_id.clone(),
        input.command.clone(),
        dispatch_plan.clone(),
        Some(provider_result.clone()),
    )
    .map_err(|message| ImageRuntimeError::Planning(message))?;
    let outputs = provider_result.outputs.clone();
    let (import_plan, upload_preparations) = plan_drive_upload_preparations(
        input.scope.tenant_id.clone(),
        input.scope.organization_id.clone(),
        &input.scope.actor,
        &service_plan.drive_import_plans,
        &outputs,
        dispatch_plan.provider_operation.as_str(),
        input.operator_id,
        input.now_epoch_ms,
    )?;
    Ok(ImageGenerationCreateRuntimeResult {
        dispatch_plan,
        provider_result,
        service_plan,
        import_plan,
        upload_preparations,
    })
}

pub async fn execute_refresh_generation_dispatch(
    provider_service: &dyn ImageGenerationServicePort,
    input: ImageGenerationRefreshRuntimeInput,
) -> Result<ImageGenerationRefreshRuntimeResult, ImageRuntimeError> {
    let provider_result = retrieve_image_generation_provider(
        provider_service,
        &input.dispatch_plan,
        &input.provider_task_id,
    )
    .await?;
    let refresh_plan = plan_generation_refresh_from_provider_result(
        input.scope.clone(),
        input.generation_id.clone(),
        input.scene.clone(),
        input.model.clone(),
        provider_result.clone(),
    )
    .map_err(|message| ImageRuntimeError::Planning(message))?;
    let outputs = provider_result.outputs.clone();
    let (import_plan, upload_preparations) = plan_drive_upload_preparations(
        input.scope.tenant_id.clone(),
        input.scope.organization_id.clone(),
        &input.scope.actor,
        &refresh_plan.drive_import_plans,
        &outputs,
        input.dispatch_plan.provider_operation.as_str(),
        input.operator_id,
        input.now_epoch_ms,
    )?;
    Ok(ImageGenerationRefreshRuntimeResult {
        provider_result,
        import_plan,
        upload_preparations,
    })
}
