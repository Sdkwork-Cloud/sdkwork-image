use sdkwork_drive_product::uploader::{UploaderActor, UploaderRetention, UploaderTarget};
use sdkwork_image_core::{
    build_drive_uploader_command_for_generated_output, create_image_generation_request,
    plan_drive_import_for_generated_outputs, validate_image_generation_request,
    DriveGeneratedMediaContext, GeneratedMediaKind, GeneratedMediaOutput, ImageGenerationActor,
    ImageGenerationRequest, ImageJobStatus, ImageVisibility,
    GENERATED_MEDIA_DEFAULT_CHUNK_SIZE_BYTES, IMAGE_CAPABILITY, IMAGE_DOMAIN, IMAGE_WORKSPACE,
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
        "tenant-1",
        Some("org-1"),
        "Premium device beauty shot",
        "1024x1024",
        "studio",
        ImageVisibility::Tenant,
    );

    assert_eq!(request.tenant_id, "tenant-1");
    assert_eq!(request.organization_id.as_deref(), Some("org-1"));
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
fn plans_ai_generated_outputs_into_drive_ai_generated_space_for_logged_in_users() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "tenant-1".to_string(),
            organization_id: Some("org-1".to_string()),
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
            tenant_id: "tenant-1".to_string(),
            organization_id: Some("org-1".to_string()),
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
        "tenant-1",
        Some("org-1"),
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
        UploaderTarget::Space { ref space_id, parent_node_id: None }
            if space_id == "space-ai-generated-user-user-001"
    ));
    assert!(matches!(command.retention, UploaderRetention::LongTerm));
}

#[test]
fn plans_anonymous_generated_outputs_with_app_owned_ai_generation_space() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "tenant-1".to_string(),
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
    assert_eq!(
        plan.drive_owner_subject_id,
        "app:sdkwork-image:anonymous:anon-session-001",
    );
    assert_eq!(
        plan.drive_space_id,
        "space-ai-generated-app-anonymous-anon-session-001",
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
            tenant_id: "tenant-1".to_string(),
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
        "tenant-1",
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
        UploaderTarget::Space { ref space_id, parent_node_id: None }
            if space_id == "space-ai-generated-app-anonymous-anon-session-001"
    ));
}

#[test]
fn plans_multi_output_generation_with_distinct_drive_nodes_and_scene() {
    let plans = plan_drive_import_for_generated_outputs(
        DriveGeneratedMediaContext {
            tenant_id: "tenant-1".to_string(),
            organization_id: Some("org-1".to_string()),
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
            tenant_id: "tenant-1".to_string(),
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
