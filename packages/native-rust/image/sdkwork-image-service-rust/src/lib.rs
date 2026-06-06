use sdkwork_image_core::{
    plan_drive_import_for_generated_outputs, plan_image_generation_provider_dispatch,
    DriveGeneratedMediaContext, DriveGeneratedMediaImportPlan, GeneratedMediaOutput,
    ImageGenerationActor, ImageGenerationCreateCommand, ImageGenerationRuntimeStatus,
    ImageProviderDispatchPlan, NormalizedProviderGenerationResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationScope {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub actor: ImageGenerationActor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationRecord {
    pub generation_id: String,
    pub status: ImageGenerationRuntimeStatus,
    pub scene: String,
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub output_count: i32,
    pub drive_space_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationProviderSubmission {
    pub generation_id: String,
    pub dispatch_plan: ImageProviderDispatchPlan,
    pub normalized_result: Option<NormalizedProviderGenerationResult>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationServicePlan {
    pub record: ImageGenerationRecord,
    pub dispatch: ImageGenerationProviderSubmission,
    pub drive_import_plans: Vec<DriveGeneratedMediaImportPlan>,
    pub outbox_events: Vec<ImageGenerationOutboxEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationOutboxEvent {
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationOutputPersistenceRow {
    pub output_index: i32,
    pub media_kind: String,
    pub scene: String,
    pub provider_code: String,
    pub provider_asset_id: Option<String>,
    pub provider_uri: Option<String>,
    pub provider_url: Option<String>,
    pub drive_space_type: String,
    pub drive_space_id: String,
    pub drive_parent_node_id: Option<String>,
    pub drive_node_id: String,
    pub drive_uri: String,
    pub resource_snapshot_id: String,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub sync_status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationPersistencePlan {
    pub generation_id: String,
    pub runtime_status: ImageGenerationRuntimeStatus,
    pub job_status_code: i32,
    pub drive_sync_status: String,
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub output_rows: Vec<ImageGenerationOutputPersistenceRow>,
    pub repository_methods: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationWebhookEnvelope {
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub external_event_id: Option<String>,
    pub event_type: String,
    pub payload_hash: String,
    pub normalized_result: NormalizedProviderGenerationResult,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationRefreshPlan {
    pub generation_id: String,
    pub status: ImageGenerationRuntimeStatus,
    pub provider_code: String,
    pub provider_task_id: Option<String>,
    pub provider_status: Option<String>,
    pub drive_import_plans: Vec<DriveGeneratedMediaImportPlan>,
    pub outbox_events: Vec<ImageGenerationOutboxEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageGenerationRuntimeStep {
    CreateGenerationRecord,
    DispatchProviderGeneration {
        provider_code: String,
        sdk_resource: String,
        sdk_method: String,
    },
    PersistProviderSubmission,
    ScheduleProviderPolling,
    AwaitProviderWebhook,
    PersistDriveImportPlan {
        output_count: i32,
    },
    PrepareDriveUpload {
        output_count: i32,
    },
    PersistOutboxEvent {
        event_type: String,
    },
}

pub fn plan_generation_create_service_flow(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    command: ImageGenerationCreateCommand,
    provider_result: Option<NormalizedProviderGenerationResult>,
) -> Result<ImageGenerationServicePlan, &'static str> {
    let generation_id =
        require_trimmed_owned(generation_id.into(), "image generation id is required")?;
    let dispatch_plan = plan_image_generation_provider_dispatch(&command)?;
    let outputs = provider_result
        .as_ref()
        .filter(|result| result.ready_for_drive_import)
        .map(|result| result.outputs.clone())
        .unwrap_or_default();
    let drive_import_plans = if outputs.is_empty() {
        Vec::new()
    } else {
        plan_drive_imports(
            &scope,
            &generation_id,
            &dispatch_plan,
            &command.scene,
            outputs,
        )?
    };
    let status = provider_result
        .as_ref()
        .map(|result| result.status)
        .unwrap_or(ImageGenerationRuntimeStatus::Dispatching);
    let provider_task_id = provider_result
        .as_ref()
        .and_then(|result| result.provider_task_id.clone());
    let provider_status = provider_result
        .as_ref()
        .and_then(|result| result.provider_status.clone());
    let drive_space_id = drive_import_plans
        .first()
        .map(|plan| plan.drive_space_id.clone());

    Ok(ImageGenerationServicePlan {
        record: ImageGenerationRecord {
            generation_id: generation_id.clone(),
            status,
            scene: command.scene,
            provider_code: dispatch_plan.provider_code.clone(),
            provider_task_id,
            provider_status,
            output_count: dispatch_plan.output_count,
            drive_space_id,
        },
        dispatch: ImageGenerationProviderSubmission {
            generation_id: generation_id.clone(),
            dispatch_plan,
            normalized_result: provider_result,
        },
        drive_import_plans,
        outbox_events: vec![ImageGenerationOutboxEvent {
            aggregate_type: "image_generation".to_string(),
            aggregate_id: generation_id,
            event_type: "image.generation.created".to_string(),
        }],
    })
}

pub fn plan_generation_create_runtime_steps(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    command: ImageGenerationCreateCommand,
) -> Result<Vec<ImageGenerationRuntimeStep>, &'static str> {
    let plan = plan_generation_create_service_flow(scope, generation_id, command, None)?;
    let dispatch_plan = &plan.dispatch.dispatch_plan;
    let mut steps = vec![
        ImageGenerationRuntimeStep::CreateGenerationRecord,
        ImageGenerationRuntimeStep::DispatchProviderGeneration {
            provider_code: dispatch_plan.provider_code.clone(),
            sdk_resource: dispatch_plan.claw_router_sdk_resource.to_string(),
            sdk_method: dispatch_plan.claw_router_sdk_method.to_string(),
        },
        ImageGenerationRuntimeStep::PersistProviderSubmission,
    ];

    if dispatch_plan.callback_url.is_some() {
        steps.push(ImageGenerationRuntimeStep::AwaitProviderWebhook);
    }
    if dispatch_plan.task_mode != sdkwork_image_core::ImageProviderTaskMode::Synchronous {
        steps.push(ImageGenerationRuntimeStep::ScheduleProviderPolling);
    }

    steps.push(ImageGenerationRuntimeStep::PersistOutboxEvent {
        event_type: "image.generation.created".to_string(),
    });

    Ok(steps)
}

pub fn plan_generation_create_persistence_plan(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    command: ImageGenerationCreateCommand,
    provider_result: Option<NormalizedProviderGenerationResult>,
) -> Result<ImageGenerationPersistencePlan, &'static str> {
    let service_plan =
        plan_generation_create_service_flow(scope, generation_id, command, provider_result)?;
    Ok(persistence_plan_from_service_plan(
        &service_plan.record,
        &service_plan.dispatch.normalized_result,
        &service_plan.drive_import_plans,
        true,
    ))
}

pub fn plan_generation_refresh_from_provider_result(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    result: NormalizedProviderGenerationResult,
) -> Result<ImageGenerationRefreshPlan, &'static str> {
    let generation_id =
        require_trimmed_owned(generation_id.into(), "image generation id is required")?;
    let scene = require_trimmed_owned(scene.into(), "image generation scene is required")?;
    let provider_code = require_trimmed_owned(
        result.provider_code.clone(),
        "image generation provider_code is required",
    )?;
    let drive_import_plans = if result.ready_for_drive_import && !result.outputs.is_empty() {
        plan_drive_import_for_generated_outputs(
            DriveGeneratedMediaContext {
                tenant_id: scope.tenant_id,
                organization_id: scope.organization_id,
                generation_id: generation_id.clone(),
                provider_code: provider_code.clone(),
                model,
                scene,
                actor: scope.actor,
            },
            result.outputs.clone(),
        )?
    } else {
        Vec::new()
    };
    let event_type = if result.status == ImageGenerationRuntimeStatus::Failed {
        "image.generation.failed"
    } else if result.ready_for_drive_import {
        "image.generation.outputs_ready"
    } else {
        "image.generation.refreshed"
    };

    Ok(ImageGenerationRefreshPlan {
        generation_id: generation_id.clone(),
        status: result.status,
        provider_code,
        provider_task_id: result.provider_task_id,
        provider_status: result.provider_status,
        drive_import_plans,
        outbox_events: vec![ImageGenerationOutboxEvent {
            aggregate_type: "image_generation".to_string(),
            aggregate_id: generation_id,
            event_type: event_type.to_string(),
        }],
    })
}

pub fn plan_generation_refresh_runtime_steps(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    result: NormalizedProviderGenerationResult,
) -> Result<Vec<ImageGenerationRuntimeStep>, &'static str> {
    let plan =
        plan_generation_refresh_from_provider_result(scope, generation_id, scene, model, result)?;
    let mut steps = vec![ImageGenerationRuntimeStep::PersistProviderSubmission];
    let output_count = i32::try_from(plan.drive_import_plans.len())
        .map_err(|_| "image generation output_count exceeds supported range")?;
    if output_count > 0 {
        steps.push(ImageGenerationRuntimeStep::PersistDriveImportPlan { output_count });
        steps.push(ImageGenerationRuntimeStep::PrepareDriveUpload { output_count });
    }
    for event in plan.outbox_events {
        steps.push(ImageGenerationRuntimeStep::PersistOutboxEvent {
            event_type: event.event_type,
        });
    }
    Ok(steps)
}

pub fn plan_generation_refresh_persistence_plan(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    result: NormalizedProviderGenerationResult,
) -> Result<ImageGenerationPersistencePlan, &'static str> {
    let plan =
        plan_generation_refresh_from_provider_result(scope, generation_id, scene, model, result)?;
    Ok(persistence_plan_from_refresh_plan(&plan))
}

pub fn plan_generation_refresh_from_webhook(
    scope: ImageGenerationScope,
    generation_id: impl Into<String>,
    scene: impl Into<String>,
    model: Option<String>,
    webhook: ImageGenerationWebhookEnvelope,
) -> Result<ImageGenerationRefreshPlan, &'static str> {
    if webhook.payload_hash.trim().is_empty() {
        return Err("image generation webhook payload_hash is required");
    }
    if webhook.event_type.trim().is_empty() {
        return Err("image generation webhook event_type is required");
    }
    if webhook.provider_code.trim() != webhook.normalized_result.provider_code.trim() {
        return Err("image generation webhook provider_code does not match normalized result");
    }
    if webhook.provider_task_id != webhook.normalized_result.provider_task_id {
        return Err("image generation webhook provider_task_id does not match normalized result");
    }

    plan_generation_refresh_from_provider_result(
        scope,
        generation_id,
        scene,
        model,
        webhook.normalized_result,
    )
}

fn persistence_plan_from_service_plan(
    record: &ImageGenerationRecord,
    normalized_result: &Option<NormalizedProviderGenerationResult>,
    drive_import_plans: &[DriveGeneratedMediaImportPlan],
    include_create_generation: bool,
) -> ImageGenerationPersistencePlan {
    let mut repository_methods = Vec::new();
    if include_create_generation {
        repository_methods.push("create_generation".to_string());
    }
    repository_methods.push("mark_provider_submitted".to_string());
    if record.provider_task_id.is_some() {
        repository_methods.push("upsert_provider_task".to_string());
    }
    if !drive_import_plans.is_empty() {
        repository_methods.push("upsert_generation_outputs".to_string());
    }
    if record.status == ImageGenerationRuntimeStatus::Failed {
        repository_methods.push("mark_generation_failed".to_string());
    }
    repository_methods.push("enqueue_notification".to_string());

    ImageGenerationPersistencePlan {
        generation_id: record.generation_id.clone(),
        runtime_status: record.status,
        job_status_code: record.status.as_job_status_code(),
        drive_sync_status: record.status.as_drive_sync_status().to_string(),
        provider_code: record.provider_code.clone(),
        provider_task_id: record.provider_task_id.clone(),
        provider_status: record.provider_status.clone().or_else(|| {
            normalized_result
                .as_ref()
                .and_then(|result| result.provider_status.clone())
        }),
        output_rows: drive_import_plans
            .iter()
            .map(output_persistence_row_from_drive_plan)
            .collect(),
        repository_methods,
    }
}

fn persistence_plan_from_refresh_plan(
    plan: &ImageGenerationRefreshPlan,
) -> ImageGenerationPersistencePlan {
    let mut repository_methods = vec!["mark_provider_submitted".to_string()];
    if !plan.drive_import_plans.is_empty() {
        repository_methods.push("upsert_generation_outputs".to_string());
    }
    if plan.status == ImageGenerationRuntimeStatus::Failed {
        repository_methods.push("mark_generation_failed".to_string());
    }
    for _ in &plan.outbox_events {
        repository_methods.push("enqueue_notification".to_string());
    }

    ImageGenerationPersistencePlan {
        generation_id: plan.generation_id.clone(),
        runtime_status: plan.status,
        job_status_code: plan.status.as_job_status_code(),
        drive_sync_status: plan.status.as_drive_sync_status().to_string(),
        provider_code: plan.provider_code.clone(),
        provider_task_id: plan.provider_task_id.clone(),
        provider_status: plan.provider_status.clone(),
        output_rows: plan
            .drive_import_plans
            .iter()
            .map(output_persistence_row_from_drive_plan)
            .collect(),
        repository_methods,
    }
}

fn output_persistence_row_from_drive_plan(
    plan: &DriveGeneratedMediaImportPlan,
) -> ImageGenerationOutputPersistenceRow {
    ImageGenerationOutputPersistenceRow {
        output_index: plan.output_index,
        media_kind: plan.media_resource.kind.clone(),
        scene: plan.scene.clone(),
        provider_code: plan.provider_code.clone(),
        provider_asset_id: plan.provider_asset_id.clone(),
        provider_uri: plan.provider_uri.clone(),
        provider_url: plan.provider_url.clone(),
        drive_space_type: plan.drive_space_type.clone(),
        drive_space_id: plan.drive_space_id.clone(),
        drive_parent_node_id: plan.drive_parent_node_id.clone(),
        drive_node_id: plan.drive_node_id.clone(),
        drive_uri: plan.drive_uri.clone(),
        resource_snapshot_id: plan.media_resource.id.clone(),
        file_name: plan.media_resource.file_name.clone(),
        mime_type: plan.media_resource.mime_type.clone(),
        size_bytes: plan.media_resource.size_bytes.clone(),
        width: plan.media_resource.width,
        height: plan.media_resource.height,
        duration_seconds: plan.media_resource.duration_seconds,
        sync_status: "pending".to_string(),
    }
}

pub fn image_generation_repository_contract_methods() -> Vec<&'static str> {
    vec![
        "create_generation",
        "mark_provider_submitted",
        "upsert_provider_task",
        "record_provider_webhook_event",
        "upsert_generation_outputs",
        "mark_drive_importing",
        "mark_drive_imported",
        "mark_generation_succeeded",
        "mark_generation_failed",
        "enqueue_notification",
        "find_due_provider_tasks",
        "find_pending_drive_imports",
    ]
}

fn plan_drive_imports(
    scope: &ImageGenerationScope,
    generation_id: &str,
    dispatch_plan: &ImageProviderDispatchPlan,
    scene: &str,
    outputs: Vec<GeneratedMediaOutput>,
) -> Result<Vec<DriveGeneratedMediaImportPlan>, &'static str> {
    plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: scope.tenant_id.clone(),
            organization_id: scope.organization_id.clone(),
            generation_id: generation_id.to_string(),
            provider_code: dispatch_plan.provider_code.clone(),
            model: dispatch_plan.model.clone(),
            scene: scene.to_string(),
            actor: scope.actor.clone(),
        },
        outputs,
    )
}

fn require_trimmed_owned(value: String, error: &'static str) -> Result<String, &'static str> {
    let value = value.trim().to_string();
    if value.is_empty() {
        Err(error)
    } else {
        Ok(value)
    }
}
