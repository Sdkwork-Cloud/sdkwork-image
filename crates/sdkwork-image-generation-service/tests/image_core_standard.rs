use sdkwork_drive_workspace_service::uploader::{UploaderActor, UploaderRetention, UploaderTarget};
use sdkwork_image_generation_service::{
    build_drive_uploader_command_for_generated_output, create_image_generation_request,
    normalize_openai_image_generation_outputs, normalize_provider_task_generation_result,
    plan_drive_import_for_generated_outputs, plan_image_generation_provider_dispatch,
    validate_image_generation_request, DriveGeneratedMediaContext, GeneratedMediaKind,
    GeneratedMediaOutput, ImageGenerationActor, ImageGenerationCreateCommand,
    ImageGenerationRequest, ImageGenerationRuntimeStatus, ImageJobStatus, ImageProviderOperation,
    ImageProviderTaskMode, ImageVisibility, OpenAiGeneratedImage, ProviderGeneratedMediaAsset,
    ProviderTaskErrorSnapshot, ProviderTaskSnapshot, GENERATED_MEDIA_DEFAULT_CHUNK_SIZE_BYTES,
    IMAGE_CAPABILITY, IMAGE_DOMAIN, IMAGE_WORKSPACE,
};

#[test]
fn exposes_image_domain_identity() {
    assert_eq!(IMAGE_WORKSPACE, "sdkwork-image");
    assert_eq!(IMAGE_DOMAIN, "image");
    assert_eq!(IMAGE_CAPABILITY, "image");
}

#[test]
fn validates_image_generation_request_boundaries() {
    let request = create_image_generation_request(
        "100001",
        Some("0"),
        "Premium device beauty shot",
        "1024x1024",
        "studio",
        ImageVisibility::Tenant,
    );

    assert_eq!(request.tenant_id, "100001");
    assert_eq!(request.organization_id.as_deref(), Some("0"));
    assert_eq!(request.status, ImageJobStatus::Queued);
    assert!(validate_image_generation_request(&request).is_ok());

    let missing_prompt = ImageGenerationRequest {
        prompt: " ".to_string(),
        ..request.clone()
    };
    assert_eq!(
        validate_image_generation_request(&missing_prompt),
        Err("image generation prompt is required"),
    );

    let invalid_resolution = ImageGenerationRequest {
        resolution: "1024".to_string(),
        ..request
    };
    assert_eq!(
        validate_image_generation_request(&invalid_resolution),
        Err("image generation resolution must use WIDTHxHEIGHT"),
    );
}

#[test]
fn image_status_and_visibility_are_stable_integer_contracts() {
    assert_eq!(ImageJobStatus::Queued as i32, 1);
    assert_eq!(ImageJobStatus::Rendering as i32, 2);
    assert_eq!(ImageJobStatus::Ready as i32, 3);
    assert_eq!(ImageJobStatus::Failed as i32, 4);

    assert_eq!(ImageVisibility::Private as i32, 1);
    assert_eq!(ImageVisibility::Tenant as i32, 2);
    assert_eq!(ImageVisibility::Public as i32, 3);
}

#[test]
fn runtime_statuses_map_to_storage_job_status_and_drive_sync_status() {
    for status in [
        ImageGenerationRuntimeStatus::Queued,
        ImageGenerationRuntimeStatus::Dispatching,
        ImageGenerationRuntimeStatus::Submitted,
    ] {
        assert_eq!(status.as_job_status(), ImageJobStatus::Queued);
        assert_eq!(status.as_job_status_code(), 1);
        assert_eq!(status.as_drive_sync_status(), "pending");
    }

    for status in [
        ImageGenerationRuntimeStatus::Rendering,
        ImageGenerationRuntimeStatus::CancelRequested,
    ] {
        assert_eq!(status.as_job_status(), ImageJobStatus::Rendering);
        assert_eq!(status.as_job_status_code(), 2);
        assert_eq!(status.as_drive_sync_status(), "pending");
    }

    assert_eq!(
        ImageGenerationRuntimeStatus::Importing.as_job_status(),
        ImageJobStatus::Rendering,
    );
    assert_eq!(
        ImageGenerationRuntimeStatus::Importing.as_drive_sync_status(),
        "importing",
    );

    assert_eq!(
        ImageGenerationRuntimeStatus::Succeeded.as_job_status(),
        ImageJobStatus::Ready,
    );
    assert_eq!(
        ImageGenerationRuntimeStatus::Succeeded.as_job_status_code(),
        3,
    );
    assert_eq!(
        ImageGenerationRuntimeStatus::Succeeded.as_drive_sync_status(),
        "imported",
    );

    for status in [
        ImageGenerationRuntimeStatus::Failed,
        ImageGenerationRuntimeStatus::Cancelled,
        ImageGenerationRuntimeStatus::Expired,
    ] {
        assert_eq!(status.as_job_status(), ImageJobStatus::Failed);
        assert_eq!(status.as_job_status_code(), 4);
        assert_eq!(status.as_drive_sync_status(), "failed");
    }
}

#[test]
fn plans_ai_generated_outputs_into_drive_ai_generated_space_for_logged_in_users() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            generation_id: "generation-001".to_string(),
            provider_code: "openai".to_string(),
            model: Some("gpt-image-1".to_string()),
            scene: "product_hero".to_string(),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        vec![GeneratedMediaOutput {
            output_index: 0,
            kind: GeneratedMediaKind::Image,
            provider_asset_id: Some("provider-image-001".to_string()),
            provider_uri: Some("provider://openai/images/provider-image-001".to_string()),
            provider_url: Some("https://provider.example.com/temporary-image.png".to_string()),
            file_name: Some("hero.png".to_string()),
            mime_type: Some("image/png".to_string()),
            size_bytes: Some(2048),
            width: Some(1024),
            height: Some(1024),
            duration_seconds: None,
        }],
    )
    .expect("drive import plan should be created");

    assert_eq!(plans.len(), 1);
    let plan = &plans[0];
    assert_eq!(plan.output_index, 0);
    assert_eq!(plan.scene, "product_hero");
    assert_eq!(plan.drive_space_type, "ai_generated");
    assert_eq!(plan.drive_owner_subject_type, "user");
    assert_eq!(plan.drive_owner_subject_id, "user-001");
    assert_eq!(plan.drive_actor_type, "user");
    assert_eq!(plan.drive_actor_id, "user-001");
    assert_eq!(plan.drive_space_id, "space-ai-generated-user-user-001");
    assert_eq!(plan.drive_upload_profile_code, "image");
    assert_eq!(plan.media_resource.kind, "image");
    assert_eq!(plan.media_resource.source, "drive");
    assert_eq!(plan.media_resource.uri, plan.drive_uri);
    assert_eq!(plan.media_resource.id, plan.drive_node_id);
    assert_eq!(plan.media_resource.ai.provenance, "generated");
    assert_eq!(plan.media_resource.ai.provider.as_deref(), Some("openai"));
    assert_eq!(plan.media_resource.ai.model.as_deref(), Some("gpt-image-1"));
    assert_eq!(
        plan.media_resource.ai.generation_task_id.as_deref(),
        Some("generation-001"),
    );
    assert_eq!(
        plan.media_resource
            .metadata
            .get("scene")
            .map(String::as_str),
        Some("product_hero"),
    );
    assert_eq!(
        plan.media_resource
            .metadata
            .get("spaceType")
            .map(String::as_str),
        Some("ai_generated"),
    );
    assert!(
        plan.media_resource.url.is_none(),
        "provider URLs must not become persisted business media identity",
    );
}

#[test]
fn builds_drive_uploader_command_with_scene_for_logged_in_generated_output() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            generation_id: "generation-001".to_string(),
            provider_code: "openai".to_string(),
            model: Some("gpt-image-1".to_string()),
            scene: "product_hero".to_string(),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        vec![GeneratedMediaOutput {
            output_index: 0,
            kind: GeneratedMediaKind::Image,
            provider_asset_id: Some("provider-image-001".to_string()),
            provider_uri: Some("provider://openai/images/provider-image-001".to_string()),
            provider_url: Some("https://provider.example.com/temporary-image.png".to_string()),
            file_name: Some("hero.png".to_string()),
            mime_type: Some("image/png".to_string()),
            size_bytes: Some(2048),
            width: Some(1024),
            height: Some(1024),
            duration_seconds: None,
        }],
    )
    .expect("drive import plan should be created");

    let command = build_drive_uploader_command_for_generated_output(
        &plans[0],
        "100001",
        Some("0"),
        "operator-001",
        1_780_000_000_000,
    )
    .expect("drive uploader command should be created");

    assert_eq!(command.scene.as_deref(), Some("product_hero"));
    assert_eq!(command.source.as_deref(), Some("ai_generated"));
    assert_eq!(command.app_id, IMAGE_WORKSPACE);
    assert_eq!(command.app_resource_type, "ai_generation_output");
    assert_eq!(command.app_resource_id, "generation-001:0");
    assert_eq!(command.upload_profile_code, "image");
    assert_eq!(command.original_file_name, "hero.png");
    assert_eq!(command.content_type, "image/png");
    assert_eq!(command.content_length, 2048);
    assert_eq!(
        command.chunk_size_bytes,
        GENERATED_MEDIA_DEFAULT_CHUNK_SIZE_BYTES,
    );
    assert!(matches!(
        command.actor,
        UploaderActor::User { ref user_id } if user_id == "user-001"
    ));
    assert!(matches!(
        command.target,
        UploaderTarget::AiGeneratedSpace {
            parent_node_id: None
        }
    ));
    assert!(matches!(command.retention, UploaderRetention::LongTerm));
}

#[test]
fn plans_anonymous_generated_outputs_with_app_owned_ai_generation_space() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: None,
            generation_id: "generation-anon".to_string(),
            provider_code: "midjourney".to_string(),
            model: None,
            scene: "anonymous_tryout".to_string(),
            actor: ImageGenerationActor::Anonymous {
                anonymous_id: "anon-session-001".to_string(),
            },
        },
        vec![GeneratedMediaOutput {
            output_index: 0,
            kind: GeneratedMediaKind::Video,
            provider_asset_id: None,
            provider_uri: Some("provider://midjourney/tasks/task-001/output-0".to_string()),
            provider_url: None,
            file_name: None,
            mime_type: Some("video/mp4".to_string()),
            size_bytes: None,
            width: Some(1280),
            height: Some(720),
            duration_seconds: Some(6),
        }],
    )
    .expect("anonymous drive import plan should be created");

    let plan = &plans[0];
    assert_eq!(plan.drive_space_type, "ai_generated");
    assert_eq!(plan.drive_owner_subject_type, "app");
    assert_eq!(plan.drive_owner_subject_id, "app:sdkwork-image:anonymous",);
    assert_eq!(plan.drive_actor_type, "anonymous");
    assert_eq!(plan.drive_actor_id, "anon-session-001");
    assert_eq!(
        plan.drive_space_id,
        "space-ai-generated-app-app-sdkwork-image-anonymous",
    );
    assert_eq!(plan.drive_upload_profile_code, "video");
    assert_eq!(plan.media_resource.kind, "video");
    assert_eq!(
        plan.media_resource
            .metadata
            .get("scene")
            .map(String::as_str),
        Some("anonymous_tryout"),
    );
}

#[test]
fn builds_drive_uploader_command_with_scene_for_anonymous_generated_output() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: None,
            generation_id: "generation-anon".to_string(),
            provider_code: "midjourney".to_string(),
            model: None,
            scene: "anonymous_tryout".to_string(),
            actor: ImageGenerationActor::Anonymous {
                anonymous_id: "anon-session-001".to_string(),
            },
        },
        vec![GeneratedMediaOutput {
            output_index: 0,
            kind: GeneratedMediaKind::Video,
            provider_asset_id: None,
            provider_uri: Some("provider://midjourney/tasks/task-001/output-0".to_string()),
            provider_url: None,
            file_name: Some("tryout.mp4".to_string()),
            mime_type: Some("video/mp4".to_string()),
            size_bytes: Some(4096),
            width: Some(1280),
            height: Some(720),
            duration_seconds: Some(6),
        }],
    )
    .expect("anonymous drive import plan should be created");

    let command = build_drive_uploader_command_for_generated_output(
        &plans[0],
        "100001",
        None,
        "operator-001",
        1_780_000_000_000,
    )
    .expect("anonymous drive uploader command should be created");

    assert_eq!(command.scene.as_deref(), Some("anonymous_tryout"));
    assert_eq!(command.source.as_deref(), Some("ai_generated"));
    assert_eq!(command.upload_profile_code, "video");
    assert_eq!(command.content_type, "video/mp4");
    assert!(matches!(
        command.actor,
        UploaderActor::Anonymous { ref anonymous_id } if anonymous_id == "anon-session-001"
    ));
    assert!(matches!(
        command.target,
        UploaderTarget::AiGeneratedSpace {
            parent_node_id: None
        }
    ));
}

#[test]
fn plans_multi_output_generation_with_distinct_drive_nodes_and_scene() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            generation_id: "generation-multi".to_string(),
            provider_code: "nano-banana".to_string(),
            model: Some("banana-image-pro".to_string()),
            scene: "brand_variants".to_string(),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        vec![
            GeneratedMediaOutput {
                output_index: 0,
                kind: GeneratedMediaKind::Image,
                provider_asset_id: Some("asset-0".to_string()),
                provider_uri: None,
                provider_url: Some("https://provider.example.com/0.png".to_string()),
                file_name: Some("variant-a.png".to_string()),
                mime_type: Some("image/png".to_string()),
                size_bytes: Some(100),
                width: Some(1024),
                height: Some(1024),
                duration_seconds: None,
            },
            GeneratedMediaOutput {
                output_index: 1,
                kind: GeneratedMediaKind::Image,
                provider_asset_id: Some("asset-1".to_string()),
                provider_uri: None,
                provider_url: Some("https://provider.example.com/1.png".to_string()),
                file_name: Some("variant-b.png".to_string()),
                mime_type: Some("image/png".to_string()),
                size_bytes: Some(101),
                width: Some(1024),
                height: Some(1024),
                duration_seconds: None,
            },
        ],
    )
    .expect("multi-output drive import plans should be created");

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].output_index, 0);
    assert_eq!(plans[1].output_index, 1);
    assert_eq!(plans[0].drive_space_id, plans[1].drive_space_id);
    assert_ne!(plans[0].drive_node_id, plans[1].drive_node_id);
    assert_ne!(plans[0].drive_uri, plans[1].drive_uri);
    assert_eq!(plans[0].scene, "brand_variants");
    assert_eq!(plans[1].scene, "brand_variants");
    assert_eq!(
        plans[0]
            .media_resource
            .metadata
            .get("scene")
            .map(String::as_str),
        Some("brand_variants"),
    );
    assert_eq!(
        plans[1]
            .media_resource
            .metadata
            .get("scene")
            .map(String::as_str),
        Some("brand_variants"),
    );
}

#[test]
fn rejects_duplicate_output_indexes_before_drive_sync() {
    let result = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: None,
            generation_id: "generation-duplicate".to_string(),
            provider_code: "openai".to_string(),
            model: None,
            scene: "duplicate_guard".to_string(),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        vec![
            GeneratedMediaOutput {
                output_index: 0,
                kind: GeneratedMediaKind::Image,
                provider_asset_id: None,
                provider_uri: None,
                provider_url: None,
                file_name: None,
                mime_type: Some("image/png".to_string()),
                size_bytes: None,
                width: None,
                height: None,
                duration_seconds: None,
            },
            GeneratedMediaOutput {
                output_index: 0,
                kind: GeneratedMediaKind::Image,
                provider_asset_id: None,
                provider_uri: None,
                provider_url: None,
                file_name: None,
                mime_type: Some("image/png".to_string()),
                size_bytes: None,
                width: None,
                height: None,
                duration_seconds: None,
            },
        ],
    );

    assert_eq!(result, Err("generated media output_index must be unique"));
}

#[test]
fn plans_image_generation_provider_dispatch_through_claw_router_sdk_boundary() {
    let plan = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Premium product shot on a matte desk".to_string(),
        negative_prompt: Some("blurry, distorted".to_string()),
        scene: "product_hero".to_string(),
        provider_code: Some("openai".to_string()),
        model: Some("gpt-image-1".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: Some("natural".to_string()),
        output_count: Some(3),
        reference_images: vec![],
        webhook_url: Some("https://app.example.com/hooks/image".to_string()),
        idempotency_key: Some("generation-idempotency-001".to_string()),
    })
    .expect("provider dispatch plan should be created");

    assert_eq!(plan.provider_code, "openai");
    assert_eq!(
        plan.provider_operation,
        ImageProviderOperation::OpenAiImageGeneration,
    );
    assert_eq!(plan.task_mode, ImageProviderTaskMode::Synchronous);
    assert_eq!(plan.claw_router_api_path, "/v1/images/generations");
    assert_eq!(plan.claw_router_sdk_resource, "images");
    assert_eq!(plan.claw_router_sdk_method, "create_generation");
    assert_eq!(plan.prompt, "Premium product shot on a matte desk");
    assert_eq!(plan.model.as_deref(), Some("gpt-image-1"));
    assert_eq!(plan.size.as_deref(), Some("1024x1024"));
    assert_eq!(plan.quality.as_deref(), Some("natural"));
    assert_eq!(plan.response_format.as_deref(), Some("url"));
    assert_eq!(plan.output_count, 3);
    assert_eq!(plan.output_count_provider_parameter.as_deref(), Some("n"));
    assert!(!plan.claw_router_api_path.contains("generation_jobs"));
    assert!(!plan.claw_router_sdk_method.contains("generationJobs"));
}

#[test]
fn plans_async_provider_dispatch_for_task_based_image_providers() {
    let plan = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Brand campaign key visual".to_string(),
        negative_prompt: None,
        scene: "brand_campaign".to_string(),
        provider_code: Some("nano-banana".to_string()),
        model: Some("banana-image-pro".to_string()),
        resolution: Some("1536x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec![],
        webhook_url: Some("https://app.example.com/hooks/nano-banana".to_string()),
        idempotency_key: None,
    })
    .expect("task provider dispatch plan should be created");

    assert_eq!(plan.provider_code, "nano-banana");
    assert_eq!(
        plan.provider_operation,
        ImageProviderOperation::NanoBananaImageGeneration,
    );
    assert_eq!(plan.task_mode, ImageProviderTaskMode::Task);
    assert_eq!(
        plan.claw_router_api_path,
        "/nano-banana/v1/images/generations"
    );
    assert_eq!(plan.claw_router_sdk_resource, "images_nano_banana");
    assert_eq!(plan.claw_router_sdk_method, "create_generations");
    assert_eq!(
        plan.callback_url.as_deref(),
        Some("https://app.example.com/hooks/nano-banana")
    );
}

#[test]
fn plans_vidu_reference_image_dispatch_through_generated_claw_router_sdk() {
    let plan = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Turn this product reference into a campaign visual".to_string(),
        negative_prompt: Some("blurred text".to_string()),
        scene: "campaign_reference".to_string(),
        provider_code: Some("vidu".to_string()),
        model: Some("vidu-image-pro".to_string()),
        resolution: Some("1536x1024".to_string()),
        style: Some("cinematic".to_string()),
        output_count: Some(1),
        reference_images: vec![
            "drive://spaces/space-1/nodes/source-product".to_string(),
            "https://cdn.example.com/reference.png".to_string(),
        ],
        webhook_url: Some("https://app.example.com/hooks/vidu".to_string()),
        idempotency_key: Some("vidu-idem-001".to_string()),
    })
    .expect("vidu reference image dispatch plan should be created");

    assert_eq!(plan.provider_code, "vidu");
    assert_eq!(
        plan.provider_operation,
        ImageProviderOperation::ViduReferenceToImageGeneration,
    );
    assert_eq!(plan.task_mode, ImageProviderTaskMode::Task);
    assert_eq!(plan.claw_router_api_path, "/vidu/ent/v2/reference2image");
    assert_eq!(plan.claw_router_sdk_resource, "images_vidu");
    assert_eq!(plan.claw_router_sdk_method, "create_ent_v2_reference2image");
    assert_eq!(plan.reference_images.len(), 2);
    assert_eq!(plan.output_count_provider_parameter, None);
}

#[test]
fn normalizes_reference_images_before_enforcing_effective_item_limit() {
    let mut reference_images = vec![
        " https://cdn.example.com/reference-a.png ".to_string(),
        "https://cdn.example.com/reference-a.png".to_string(),
        "drive://spaces/space-1/nodes/reference-b".to_string(),
    ];
    reference_images.extend((0..14).map(|_| "   ".to_string()));

    let plan = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Use normalized reference images".to_string(),
        negative_prompt: None,
        scene: "reference_normalization".to_string(),
        provider_code: Some("nano-banana".to_string()),
        model: Some("banana-image-pro".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images,
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("raw reference image input can contain empty and duplicate values");

    assert_eq!(
        plan.reference_images,
        vec![
            "https://cdn.example.com/reference-a.png".to_string(),
            "drive://spaces/space-1/nodes/reference-b".to_string(),
        ],
    );
}

#[test]
fn plans_task_provider_retrieval_through_generated_claw_router_sdk() {
    let cases = [
        (
            "midjourney",
            "mj-v7",
            "images_midjourney",
            "list_v1_images_generations",
            "/midjourney/v1/images/generations/{task_id}",
            vec![],
        ),
        (
            "nano-banana",
            "banana-image-pro",
            "images_nano_banana",
            "retrieve_generations",
            "/nano-banana/v1/images/generations/{task_id}",
            vec![],
        ),
        (
            "vidu",
            "vidu-image-pro",
            "videos_vidu",
            "list_ent_v2_tasks_creations",
            "/vidu/ent/v2/tasks/{task_id}/creations",
            vec!["https://cdn.example.com/reference.png".to_string()],
        ),
    ];

    for (
        provider_code,
        model,
        retrieve_sdk_resource,
        retrieve_sdk_method,
        retrieve_api_path,
        reference_images,
    ) in cases
    {
        let plan = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
            prompt: "Poll generated image task".to_string(),
            negative_prompt: None,
            scene: "polling_contract".to_string(),
            provider_code: Some(provider_code.to_string()),
            model: Some(model.to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(1),
            reference_images,
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("task provider dispatch plan should build");

        assert_eq!(plan.task_mode, ImageProviderTaskMode::Task);
        assert_eq!(
            plan.claw_router_retrieve_sdk_resource,
            Some(retrieve_sdk_resource),
            "{provider_code} retrieve resource must be generated SDK-backed",
        );
        assert_eq!(
            plan.claw_router_retrieve_sdk_method,
            Some(retrieve_sdk_method),
            "{provider_code} retrieve method must be generated SDK-backed",
        );
        assert_eq!(
            plan.claw_router_retrieve_api_path,
            Some(retrieve_api_path),
            "{provider_code} retrieve path must be auditable",
        );
    }
}

#[test]
fn rejects_providers_without_generated_claw_router_image_create_method() {
    for provider_code in ["gemini", "kling", "jimeng", "volcengine", "custom-provider"] {
        let result = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
            prompt: "Render through a provider that has no generated image SDK resource"
                .to_string(),
            negative_prompt: None,
            scene: "generated_sdk_guard".to_string(),
            provider_code: Some(provider_code.to_string()),
            model: Some("image-model".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(1),
            reference_images: vec![],
            webhook_url: Some("https://app.example.com/hooks/provider".to_string()),
            idempotency_key: None,
        });

        assert_eq!(
            result,
            Err("image generation provider is not exposed by the generated Claw Router SDK"),
            "{provider_code} must not fall back to an unrelated generated SDK method",
        );
    }
}

#[test]
fn rejects_reference_images_for_providers_without_generated_reference_image_field() {
    for provider_code in ["openai", "midjourney"] {
        let result = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
            prompt: "Use this reference image".to_string(),
            negative_prompt: None,
            scene: "reference_guard".to_string(),
            provider_code: Some(provider_code.to_string()),
            model: Some("image-model".to_string()),
            resolution: Some("1024x1024".to_string()),
            style: None,
            output_count: Some(1),
            reference_images: vec!["https://cdn.example.com/reference.png".to_string()],
            webhook_url: None,
            idempotency_key: None,
        });

        assert_eq!(
            result,
            Err("image generation provider does not support reference_images"),
            "{provider_code} must not silently drop reference_images",
        );
    }
}

#[test]
fn normalizes_openai_image_generation_response_into_multi_output_drive_inputs() {
    let outputs = normalize_openai_image_generation_outputs(
        "openai",
        vec![
            OpenAiGeneratedImage {
                url: Some("https://provider.example.com/generated/hero-a.png".to_string()),
                b64_json: None,
                mime_type: Some("image/png".to_string()),
                revised_prompt: Some("A refined hero shot".to_string()),
            },
            OpenAiGeneratedImage {
                url: None,
                b64_json: Some("ZmFrZS1pbWFnZS1ieXRlcw==".to_string()),
                mime_type: Some("image/png".to_string()),
                revised_prompt: None,
            },
        ],
    )
    .expect("openai image outputs should normalize");

    assert_eq!(outputs.len(), 2);
    assert_eq!(outputs[0].output_index, 0);
    assert_eq!(outputs[0].kind, GeneratedMediaKind::Image);
    assert_eq!(
        outputs[0].provider_url.as_deref(),
        Some("https://provider.example.com/generated/hero-a.png"),
    );
    assert_eq!(
        outputs[0].provider_uri.as_deref(),
        Some("provider://openai/images/0"),
    );
    assert_eq!(outputs[0].mime_type.as_deref(), Some("image/png"));
    assert_eq!(outputs[1].output_index, 1);
    assert_eq!(
        outputs[1].provider_uri.as_deref(),
        Some("data:image/png;base64,ZmFrZS1pbWFnZS1ieXRlcw=="),
    );
    assert!(outputs[1].provider_url.is_none());

    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "100001".to_string(),
            organization_id: Some("0".to_string()),
            generation_id: "generation-openai-sync".to_string(),
            provider_code: "openai".to_string(),
            model: Some("gpt-image-1".to_string()),
            scene: "product_hero".to_string(),
            actor: ImageGenerationActor::User {
                user_id: "user-001".to_string(),
            },
        },
        outputs,
    )
    .expect("drive import plans should be created from normalized provider outputs");

    assert_eq!(plans.len(), 2);
    assert_eq!(plans[0].drive_space_type, "ai_generated");
    assert_eq!(plans[1].drive_space_type, "ai_generated");
    assert_eq!(plans[0].scene, "product_hero");
    assert_eq!(plans[1].scene, "product_hero");
}

#[test]
fn normalizes_async_provider_task_result_for_webhook_or_polling_consistency() {
    let normalized = normalize_provider_task_generation_result(
        "nano-banana",
        ProviderTaskSnapshot {
            task_id: Some("task-nano-001".to_string()),
            id: Some("provider-result-001".to_string()),
            status: Some("SUCCEEDED".to_string()),
            state: Some("completed".to_string()),
            model: Some("banana-image-pro".to_string()),
            images: vec![
                ProviderGeneratedMediaAsset {
                    id: Some("asset-0".to_string()),
                    uri: Some("provider://nano-banana/tasks/task-nano-001/images/0".to_string()),
                    url: Some("https://provider.example.com/nano/0.png".to_string()),
                    mime_type: Some("image/png".to_string()),
                    width: Some(1536),
                    height: Some(1024),
                    duration_seconds: None,
                },
                ProviderGeneratedMediaAsset {
                    id: Some("asset-1".to_string()),
                    uri: Some("provider://nano-banana/tasks/task-nano-001/images/1".to_string()),
                    url: Some("https://provider.example.com/nano/1.png".to_string()),
                    mime_type: Some("image/png".to_string()),
                    width: Some(1536),
                    height: Some(1024),
                    duration_seconds: None,
                },
            ],
            error: None,
        },
    )
    .expect("provider task should normalize");

    assert_eq!(normalized.provider_code, "nano-banana");
    assert_eq!(
        normalized.provider_task_id.as_deref(),
        Some("task-nano-001")
    );
    assert_eq!(normalized.provider_status.as_deref(), Some("SUCCEEDED"));
    assert_eq!(normalized.status, ImageGenerationRuntimeStatus::Importing);
    assert!(normalized.provider_terminal);
    assert!(normalized.ready_for_drive_import);
    assert_eq!(normalized.outputs.len(), 2);
    assert_eq!(
        normalized.outputs[0].provider_asset_id.as_deref(),
        Some("asset-0")
    );
    assert_eq!(
        normalized.outputs[1].provider_asset_id.as_deref(),
        Some("asset-1")
    );
}

#[test]
fn normalizes_provider_failure_without_drive_import_outputs() {
    let normalized = normalize_provider_task_generation_result(
        "midjourney",
        ProviderTaskSnapshot {
            task_id: Some("task-midjourney-001".to_string()),
            id: None,
            status: Some("failed".to_string()),
            state: None,
            model: Some("mj-v7".to_string()),
            images: vec![],
            error: Some(ProviderTaskErrorSnapshot {
                code: Some("provider_failed".to_string()),
                message: Some("Provider rejected prompt".to_string()),
                error_type: Some("moderation".to_string()),
            }),
        },
    )
    .expect("provider failure should normalize");

    assert_eq!(normalized.status, ImageGenerationRuntimeStatus::Failed);
    assert!(normalized.provider_terminal);
    assert!(!normalized.ready_for_drive_import);
    assert!(normalized.outputs.is_empty());
    assert_eq!(normalized.error_code.as_deref(), Some("provider_failed"));
    assert_eq!(
        normalized.error_message.as_deref(),
        Some("Provider rejected prompt")
    );
}
