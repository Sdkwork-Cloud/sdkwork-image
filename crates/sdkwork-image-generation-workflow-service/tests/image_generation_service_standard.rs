use sdkwork_image_generation_service::{
    normalize_provider_task_generation_result, ImageGenerationActor, ImageGenerationCreateCommand,
    ImageGenerationRuntimeStatus, ProviderGeneratedMediaAsset, ProviderTaskSnapshot,
};
use sdkwork_image_generation_workflow_service::{
    image_generation_repository_contract_methods, plan_generation_create_persistence_plan,
    plan_generation_create_runtime_steps, plan_generation_create_service_flow,
    plan_generation_refresh_from_provider_result, plan_generation_refresh_from_webhook,
    plan_generation_refresh_persistence_plan, plan_generation_refresh_runtime_steps,
    ImageGenerationRuntimeStep, ImageGenerationScope, ImageGenerationWebhookEnvelope,
};

#[test]
fn plans_create_flow_with_provider_result_drive_import_and_outbox() {
    let result = normalize_provider_task_generation_result(
        "nano-banana",
        ProviderTaskSnapshot {
            task_id: Some("task-001".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: Some("completed".to_string()),
            model: Some("banana-image-pro".to_string()),
            images: vec![ProviderGeneratedMediaAsset {
                id: Some("asset-001".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-001.png".to_string()),
                mime_type: Some("image/png".to_string()),
                width: Some(1024),
                height: Some(1024),
                duration_seconds: None,
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_generation_create_service_flow(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "generation-001",
        ImageGenerationCreateCommand {
            prompt: "Product hero".to_string(),
            negative_prompt: None,
            scene: "product_hero".to_string(),
            provider_code: Some("nano-banana".to_string()),
            model: Some("banana-image-pro".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(1),
            reference_images: vec![],
            webhook_url: Some("https://app.example.com/hooks/image".to_string()),
            idempotency_key: Some("idem-001".to_string()),
        },
        Some(result),
    )
    .expect("service flow should plan");

    assert_eq!(plan.record.generation_id, "generation-001");
    assert_eq!(plan.record.status, ImageGenerationRuntimeStatus::Importing);
    assert_eq!(plan.record.scene, "product_hero");
    assert_eq!(plan.record.provider_code, "nano-banana");
    assert_eq!(plan.record.provider_task_id.as_deref(), Some("task-001"));
    assert_eq!(
        plan.dispatch.dispatch_plan.claw_router_sdk_resource,
        "images_nano_banana",
    );
    assert_eq!(plan.drive_import_plans.len(), 1);
    assert_eq!(plan.drive_import_plans[0].scene, "product_hero");
    assert_eq!(plan.drive_import_plans[0].drive_space_type, "ai_generated");
    assert_eq!(
        plan.drive_import_plans[0].drive_owner_subject_id,
        "user-001",
    );
    assert_eq!(plan.outbox_events[0].event_type, "image.generation.created");
}

#[test]
fn plans_refresh_from_polling_result_with_drive_import_outputs_ready_event() {
    let result = normalize_provider_task_generation_result(
        "midjourney",
        ProviderTaskSnapshot {
            task_id: Some("task-002".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("mj-v7".to_string()),
            images: vec![ProviderGeneratedMediaAsset {
                id: Some("asset-002".to_string()),
                uri: Some("provider://midjourney/tasks/task-002/images/0".to_string()),
                url: Some("https://provider.example.com/asset-002.png".to_string()),
                mime_type: Some("image/png".to_string()),
                width: Some(1536),
                height: Some(1024),
                duration_seconds: None,
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_generation_refresh_from_provider_result(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: ImageGenerationActor::Anonymous {
                anonymous_id: "anon-001".to_string(),
            },
        },
        "generation-002",
        "anonymous_tryout",
        Some("mj-v7".to_string()),
        result,
    )
    .expect("refresh plan should build");

    assert_eq!(plan.status, ImageGenerationRuntimeStatus::Importing);
    assert_eq!(plan.provider_task_id.as_deref(), Some("task-002"));
    assert_eq!(plan.drive_import_plans.len(), 1);
    assert_eq!(
        plan.drive_import_plans[0].drive_owner_subject_id,
        "app:sdkwork-image:anonymous",
    );
    assert_eq!(plan.drive_import_plans[0].drive_actor_type, "anonymous");
    assert_eq!(plan.drive_import_plans[0].drive_actor_id, "anon-001");
    assert_eq!(
        plan.outbox_events[0].event_type,
        "image.generation.outputs_ready",
    );
}

#[test]
fn validates_webhook_consistency_before_planning_refresh() {
    let result = normalize_provider_task_generation_result(
        "nano-banana",
        ProviderTaskSnapshot {
            task_id: Some("task-003".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: None,
            model: None,
            images: vec![ProviderGeneratedMediaAsset {
                id: Some("asset-003".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-003.png".to_string()),
                mime_type: Some("image/png".to_string()),
                width: None,
                height: None,
                duration_seconds: None,
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_generation_refresh_from_webhook(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "generation-003",
        "webhook_scene",
        None,
        ImageGenerationWebhookEnvelope {
            provider_code: "nano-banana".to_string(),
            provider_task_id: Some("task-003".to_string()),
            external_event_id: Some("evt-003".to_string()),
            event_type: "generation.succeeded".to_string(),
            payload_hash: "hash-003".to_string(),
            normalized_result: result,
        },
    )
    .expect("webhook refresh plan should build");

    assert_eq!(plan.generation_id, "generation-003");
    assert_eq!(plan.drive_import_plans.len(), 1);
    assert_eq!(
        plan.outbox_events[0].event_type,
        "image.generation.outputs_ready",
    );
}

#[test]
fn declares_repository_methods_needed_for_complete_runtime_consistency() {
    let methods = image_generation_repository_contract_methods();

    for expected in [
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
    ] {
        assert!(
            methods.contains(&expected),
            "missing repository method {expected}"
        );
    }
}

#[test]
fn plans_create_persistence_bindings_with_integer_job_status_and_drive_sync_state() {
    let result = normalize_provider_task_generation_result(
        "nano-banana",
        ProviderTaskSnapshot {
            task_id: Some("task-persist-001".to_string()),
            id: None,
            status: Some("succeeded".to_string()),
            state: Some("completed".to_string()),
            model: Some("banana-image-pro".to_string()),
            images: vec![ProviderGeneratedMediaAsset {
                id: Some("asset-persist-001".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-persist-001.png".to_string()),
                mime_type: Some("image/png".to_string()),
                width: Some(1024),
                height: Some(1024),
                duration_seconds: None,
            }],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let plan = plan_generation_create_persistence_plan(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "generation-persist-001",
        ImageGenerationCreateCommand {
            prompt: "Product hero".to_string(),
            negative_prompt: None,
            scene: "product_hero".to_string(),
            provider_code: Some("nano-banana".to_string()),
            model: Some("banana-image-pro".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(1),
            reference_images: vec![],
            webhook_url: Some("https://app.example.com/hooks/image".to_string()),
            idempotency_key: Some("idem-persist-001".to_string()),
        },
        Some(result),
    )
    .expect("persistence plan should build");

    assert_eq!(plan.generation_id, "generation-persist-001");
    assert_eq!(plan.runtime_status, ImageGenerationRuntimeStatus::Importing);
    assert_eq!(plan.job_status_code, 2);
    assert_eq!(plan.drive_sync_status, "importing");
    assert_eq!(plan.repository_methods[0], "create_generation");
    assert!(plan
        .repository_methods
        .contains(&"upsert_generation_outputs".to_string()));
    assert!(plan
        .repository_methods
        .contains(&"mark_provider_submitted".to_string()));
    assert_eq!(plan.output_rows.len(), 1);
    assert_eq!(plan.output_rows[0].sync_status, "pending");
    assert_eq!(plan.output_rows[0].scene, "product_hero");
    assert_eq!(plan.output_rows[0].drive_space_type, "ai_generated");
    assert_eq!(plan.output_rows[0].media_kind, "image");
}

#[test]
fn plans_create_persistence_snapshots_with_reference_images_for_replay_and_audit() {
    let plan = plan_generation_create_persistence_plan(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "generation-persist-reference-001",
        ImageGenerationCreateCommand {
            prompt: "Use the reference products for a campaign visual".to_string(),
            negative_prompt: Some("warped packaging".to_string()),
            scene: "campaign_reference".to_string(),
            provider_code: Some("nano-banana".to_string()),
            model: Some("banana-image-pro".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: Some("editorial".to_string()),
            output_count: Some(1),
            reference_images: vec![
                " https://cdn.example.com/source-a.png ".to_string(),
                "https://cdn.example.com/source-a.png".to_string(),
                "drive://spaces/space-1/nodes/source-b".to_string(),
            ],
            webhook_url: Some("https://app.example.com/hooks/nano".to_string()),
            idempotency_key: Some("idem-reference-001".to_string()),
        },
        None,
    )
    .expect("reference-image persistence plan should build");

    let input_snapshot = plan
        .input_snapshot
        .as_ref()
        .expect("create persistence must retain the normalized input snapshot");
    assert_eq!(input_snapshot.provider_code, "nano-banana");
    assert_eq!(input_snapshot.scene, "campaign_reference");
    assert_eq!(
        input_snapshot.provider_operation,
        "nano_banana.images.generate",
    );
    assert_eq!(input_snapshot.output_count, 1);
    assert_eq!(
        input_snapshot.reference_images,
        vec![
            "https://cdn.example.com/source-a.png".to_string(),
            "drive://spaces/space-1/nodes/source-b".to_string(),
        ],
    );

    let provider_request_snapshot = plan
        .provider_request_snapshot
        .as_ref()
        .expect("create persistence must retain the generated provider request snapshot");
    assert_eq!(provider_request_snapshot.sdk_resource, "images_nano_banana");
    assert_eq!(provider_request_snapshot.sdk_method, "create_generations");
    assert_eq!(
        provider_request_snapshot.retrieve_sdk_resource.as_deref(),
        Some("images_nano_banana"),
    );
    assert_eq!(
        provider_request_snapshot.retrieve_sdk_method.as_deref(),
        Some("retrieve_generations"),
    );
    assert_eq!(
        provider_request_snapshot.retrieve_api_path.as_deref(),
        Some("/nano-banana/v1/images/generations/{task_id}"),
    );
    assert_eq!(
        provider_request_snapshot.reference_images,
        input_snapshot.reference_images,
    );
}

#[test]
fn plans_refresh_persistence_bindings_for_failed_and_ready_outputs() {
    let failure = normalize_provider_task_generation_result(
        "midjourney",
        ProviderTaskSnapshot {
            task_id: Some("task-persist-failed".to_string()),
            id: None,
            status: Some("failed".to_string()),
            state: None,
            model: None,
            images: vec![],
            error: None,
        },
    )
    .expect("failure result should normalize");

    let failed_plan = plan_generation_refresh_persistence_plan(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: ImageGenerationActor::Anonymous {
                anonymous_id: "anon-persist".to_string(),
            },
        },
        "generation-persist-failed",
        "anonymous_tryout",
        None,
        failure,
    )
    .expect("failed persistence plan should build");

    assert_eq!(failed_plan.job_status_code, 4);
    assert_eq!(failed_plan.drive_sync_status, "failed");
    assert_eq!(failed_plan.provider_code, "midjourney");
    assert!(failed_plan
        .repository_methods
        .contains(&"mark_generation_failed".to_string()));
    assert!(failed_plan.output_rows.is_empty());

    let ready = normalize_provider_task_generation_result(
        "midjourney",
        ProviderTaskSnapshot {
            task_id: Some("task-persist-ready".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("mj-v7".to_string()),
            images: vec![ProviderGeneratedMediaAsset {
                id: Some("asset-persist-ready".to_string()),
                uri: None,
                url: Some("https://provider.example.com/asset-persist-ready.png".to_string()),
                mime_type: Some("image/png".to_string()),
                width: None,
                height: None,
                duration_seconds: None,
            }],
            error: None,
        },
    )
    .expect("ready result should normalize");

    let ready_plan = plan_generation_refresh_persistence_plan(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: ImageGenerationActor::Anonymous {
                anonymous_id: "anon-persist".to_string(),
            },
        },
        "generation-persist-ready",
        "anonymous_tryout",
        Some("mj-v7".to_string()),
        ready,
    )
    .expect("ready persistence plan should build");

    assert_eq!(ready_plan.job_status_code, 2);
    assert_eq!(ready_plan.drive_sync_status, "importing");
    assert_eq!(ready_plan.provider_code, "midjourney");
    assert!(ready_plan
        .repository_methods
        .contains(&"upsert_generation_outputs".to_string()));
    assert_eq!(ready_plan.output_rows.len(), 1);
    assert_eq!(ready_plan.output_rows[0].sync_status, "pending");
    assert_eq!(ready_plan.output_rows[0].drive_space_type, "ai_generated");
}

#[test]
fn plans_executable_runtime_steps_for_generation_create_flow() {
    let steps = plan_generation_create_runtime_steps(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "generation-runtime-001",
        ImageGenerationCreateCommand {
            prompt: "Product hero".to_string(),
            negative_prompt: None,
            scene: "product_hero".to_string(),
            provider_code: Some("nano-banana".to_string()),
            model: Some("banana-image-pro".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(2),
            reference_images: vec![],
            webhook_url: Some("https://app.example.com/hooks/image".to_string()),
            idempotency_key: Some("idem-runtime-001".to_string()),
        },
    )
    .expect("runtime steps should plan");

    assert_eq!(
        steps,
        vec![
            ImageGenerationRuntimeStep::CreateGenerationRecord,
            ImageGenerationRuntimeStep::DispatchProviderGeneration {
                provider_code: "nano-banana".to_string(),
                sdk_resource: "images_nano_banana".to_string(),
                sdk_method: "create_generations".to_string(),
            },
            ImageGenerationRuntimeStep::PersistProviderSubmission,
            ImageGenerationRuntimeStep::AwaitProviderWebhook,
            ImageGenerationRuntimeStep::ScheduleProviderPolling {
                provider_code: "nano-banana".to_string(),
                api_path: "/nano-banana/v1/images/generations/{task_id}".to_string(),
                sdk_resource: "images_nano_banana".to_string(),
                sdk_method: "retrieve_generations".to_string(),
            },
            ImageGenerationRuntimeStep::PersistOutboxEvent {
                event_type: "image.generation.created".to_string(),
            },
        ],
    );
}

#[test]
fn rejects_executable_runtime_steps_without_generated_create_sdk_method() {
    let result = plan_generation_create_runtime_steps(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        "generation-runtime-provider-native",
        ImageGenerationCreateCommand {
            prompt: "Future provider render".to_string(),
            negative_prompt: None,
            scene: "provider_native_unmapped".to_string(),
            provider_code: Some("gemini".to_string()),
            model: Some("gemini-image-pro".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(1),
            reference_images: vec![],
            webhook_url: None,
            idempotency_key: None,
        },
    );

    assert_eq!(
        result,
        Err("image generation provider is not exposed by the generated Claw Router SDK"),
    );
}

#[test]
fn plans_executable_runtime_steps_for_refresh_outputs_and_drive_sync() {
    let result = normalize_provider_task_generation_result(
        "midjourney",
        ProviderTaskSnapshot {
            task_id: Some("task-runtime-002".to_string()),
            id: None,
            status: Some("completed".to_string()),
            state: None,
            model: Some("mj-v7".to_string()),
            images: vec![
                ProviderGeneratedMediaAsset {
                    id: Some("asset-runtime-0".to_string()),
                    uri: None,
                    url: Some("https://provider.example.com/runtime-0.png".to_string()),
                    mime_type: Some("image/png".to_string()),
                    width: Some(1024),
                    height: Some(1024),
                    duration_seconds: None,
                },
                ProviderGeneratedMediaAsset {
                    id: Some("asset-runtime-1".to_string()),
                    uri: None,
                    url: Some("https://provider.example.com/runtime-1.png".to_string()),
                    mime_type: Some("image/png".to_string()),
                    width: Some(1024),
                    height: Some(1024),
                    duration_seconds: None,
                },
            ],
            error: None,
        },
    )
    .expect("provider result should normalize");

    let steps = plan_generation_refresh_runtime_steps(
        ImageGenerationScope {
            tenant_id: "100001".to_string(),
            organization_id: None,
            actor: ImageGenerationActor::Anonymous {
                anonymous_id: "anon-runtime-002".to_string(),
            },
        },
        "generation-runtime-002",
        "anonymous_tryout",
        Some("mj-v7".to_string()),
        result,
    )
    .expect("runtime refresh steps should plan");

    assert_eq!(
        steps,
        vec![
            ImageGenerationRuntimeStep::PersistProviderSubmission,
            ImageGenerationRuntimeStep::PersistDriveImportPlan { output_count: 2 },
            ImageGenerationRuntimeStep::PrepareDriveUpload { output_count: 2 },
            ImageGenerationRuntimeStep::PersistOutboxEvent {
                event_type: "image.generation.outputs_ready".to_string(),
            },
        ],
    );
}

#[test]
fn finalize_persistence_marks_generation_succeeded_after_import() {
    use sdkwork_image_generation_workflow_service::{
        finalize_persistence_after_drive_import, ImageGenerationOutputPersistenceRow,
        ImageGenerationPersistencePlan, OutputDriveImportState,
    };

    let mut persistence = ImageGenerationPersistencePlan {
        generation_id: "gen-1".to_string(),
        runtime_status: ImageGenerationRuntimeStatus::Importing,
        job_status_code: ImageGenerationRuntimeStatus::Importing.as_job_status_code(),
        drive_sync_status: "importing".to_string(),
        provider_code: "openai".to_string(),
        provider_task_id: None,
        provider_status: None,
        input_snapshot: None,
        provider_request_snapshot: None,
        output_rows: vec![ImageGenerationOutputPersistenceRow {
            output_index: 0,
            media_kind: "image".to_string(),
            scene: "playground_image".to_string(),
            provider_code: "openai".to_string(),
            provider_asset_id: None,
            provider_uri: None,
            provider_url: Some("https://example.com/a.png".to_string()),
            drive_space_type: "ai_generated".to_string(),
            drive_space_id: "space-1".to_string(),
            drive_parent_node_id: None,
            drive_node_id: "node-1".to_string(),
            drive_uri: "drive://spaces/space-1/nodes/node-1".to_string(),
            resource_snapshot_id: "res-1".to_string(),
            file_name: Some("a.png".to_string()),
            mime_type: Some("image/png".to_string()),
            size_bytes: None,
            width: None,
            height: None,
            duration_seconds: None,
            sync_status: "pending".to_string(),
        }],
        repository_methods: vec!["upsert_generation_outputs".to_string()],
        outbox_events: Vec::new(),
    };
    finalize_persistence_after_drive_import(
        &mut persistence,
        "imported",
        &[OutputDriveImportState {
            output_index: 0,
            sync_status: "imported".to_string(),
            drive_space_id: Some("space-1".to_string()),
            drive_node_id: Some("node-1".to_string()),
            drive_uri: Some("drive://spaces/space-1/nodes/node-1".to_string()),
        }],
    );
    assert_eq!(persistence.drive_sync_status, "imported");
    assert_eq!(persistence.output_rows[0].sync_status, "imported");
    assert_eq!(
        persistence.runtime_status,
        ImageGenerationRuntimeStatus::Succeeded
    );
    assert!(persistence
        .repository_methods
        .iter()
        .any(|method| method == "mark_generation_succeeded"));
}
