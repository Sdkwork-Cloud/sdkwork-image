use clawrouter_open_sdk::{
    OpenAiImageGenerationRequest, SdkworkAiClient, ViduCreation, ViduImageGenerationTask,
};
use sdkwork_image_claw_router_provider_service::{
    build_midjourney_image_generation_request, build_nano_banana_image_generation_request,
    build_openai_image_generation_request, build_vidu_reference_to_image_request,
    normalize_vidu_image_generation_task_result, openai_image_generation_sdk_supports_output_count,
    provider_gateway_supports_create_operation, provider_gateway_supports_retrieve_operation,
    CLAW_ROUTER_IMAGE_GENERATION_METHOD, CLAW_ROUTER_OPEN_SDK_CRATE,
};
use sdkwork_image_generation_service::{
    plan_image_generation_provider_dispatch, ImageGenerationCreateCommand, ImageProviderOperation,
    ImageProviderTaskMode,
};

#[test]
fn exposes_generated_claw_router_sdk_as_image_provider_boundary() {
    let client = SdkworkAiClient::new_with_base_url("http://127.0.0.1:18080")
        .expect("generated claw router SDK client should construct");
    let _images = client.images();

    assert_eq!(CLAW_ROUTER_OPEN_SDK_CRATE, "clawrouter_open_sdk");
    assert_eq!(
        CLAW_ROUTER_IMAGE_GENERATION_METHOD,
        "images.create_generation",
    );
}

#[test]
fn provider_lib_rs_is_public_module_boundary() {
    let lib_rs = include_str!("../src/lib.rs");

    assert!(
        lib_rs.lines().count() <= 80,
        "src/lib.rs must stay a small module assembly file",
    );
    assert!(
        !lib_rs.contains("pub struct ClawRouterImageProviderGateway"),
        "provider client implementation belongs in a focused module",
    );
    assert!(
        lib_rs.contains("pub use gateway::"),
        "public gateway exports must be re-exported from a focused module",
    );
}

#[test]
fn gateway_source_uses_only_generated_claw_router_sdk_methods_for_supported_operations() {
    let gateway_rs = include_str!("../src/gateway.rs");
    let requests_rs = include_str!("../src/requests.rs");
    let source = format!("{gateway_rs}\n{requests_rs}");
    let compact_source = source.split_whitespace().collect::<String>();

    for expected_call in [
        ".images().create_generation(",
        ".images_midjourney().create_v1_images_generation(",
        ".images_midjourney().list_v1_images_generations(",
        ".images_nano_banana().create_generations(",
        ".images_nano_banana().retrieve_generations(",
        ".images_vidu().create_ent_v2_reference2image(",
        ".videos_vidu().list_ent_v2_tasks_creations(",
    ] {
        assert!(
            compact_source.contains(expected_call),
            "missing generated Claw Router SDK call: {expected_call}",
        );
    }

    for forbidden in [
        "reqwest",
        "ureq",
        "hyper",
        "fetch(",
        "axios.",
        "Authorization",
        "Access-Token",
        "X-API-Key",
    ] {
        assert!(
            !source.contains(forbidden),
            "provider gateway must not use raw HTTP or manual credential boundary: {forbidden}",
        );
    }
}

#[test]
fn maps_image_dispatch_plan_to_generated_openai_image_request() {
    let plan = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Product hero".to_string(),
        negative_prompt: None,
        scene: "product_hero".to_string(),
        provider_code: Some("openai".to_string()),
        model: Some("gpt-image-1".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: Some("high".to_string()),
        output_count: Some(2),
        reference_images: vec![],
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("dispatch plan should build");

    let request = build_openai_image_generation_request(&plan);
    assert_eq!(request.model, "gpt-image-1");
    assert_eq!(request.prompt, "Product hero");
    assert_eq!(request.size.as_deref(), Some("1024x1024"));
    assert_eq!(request.quality.as_deref(), Some("high"));
    assert_eq!(request.response_format.as_deref(), Some("url"));

    let serialized = serde_json::to_value(OpenAiImageGenerationRequest {
        model: request.model,
        n: request.n,
        prompt: request.prompt,
        quality: request.quality,
        response_format: request.response_format,
        size: request.size,
    })
    .expect("generated SDK request should serialize");
    assert_eq!(request.n, Some(2));
    assert_eq!(
        serialized.get("n").and_then(|value| value.as_i64()),
        Some(2)
    );
    assert_eq!(openai_image_generation_sdk_supports_output_count(), true);
}

#[test]
fn maps_task_provider_plans_to_generated_provider_native_requests() {
    let midjourney = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Editorial campaign".to_string(),
        negative_prompt: Some("low quality".to_string()),
        scene: "campaign".to_string(),
        provider_code: Some("midjourney".to_string()),
        model: Some("mj-v7".to_string()),
        resolution: Some("1536x1024".to_string()),
        style: Some("raw".to_string()),
        output_count: Some(1),
        reference_images: vec![],
        webhook_url: Some("https://app.example.com/hooks/mj".to_string()),
        idempotency_key: None,
    })
    .expect("midjourney dispatch plan should build");

    assert_eq!(
        midjourney.provider_operation,
        ImageProviderOperation::MidjourneyImageGeneration,
    );
    assert_eq!(midjourney.task_mode, ImageProviderTaskMode::Task);
    let midjourney_request = build_midjourney_image_generation_request(&midjourney);
    assert_eq!(midjourney_request.model.as_deref(), Some("mj-v7"));
    assert_eq!(midjourney_request.aspect_ratio.as_deref(), Some("3:2"));
    assert_eq!(
        midjourney_request.callback_url.as_deref(),
        Some("https://app.example.com/hooks/mj"),
    );
    assert!(midjourney_request
        .prompt
        .contains("Negative prompt: low quality"));

    let nano_banana = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Brand key visual".to_string(),
        negative_prompt: None,
        scene: "campaign".to_string(),
        provider_code: Some("nano-banana".to_string()),
        model: Some("banana-image-pro".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec![],
        webhook_url: Some("https://app.example.com/hooks/nano".to_string()),
        idempotency_key: None,
    })
    .expect("nano banana dispatch plan should build");

    assert_eq!(
        nano_banana.provider_operation,
        ImageProviderOperation::NanoBananaImageGeneration,
    );
    let nano_request = build_nano_banana_image_generation_request(&nano_banana);
    assert_eq!(nano_request.model.as_deref(), Some("banana-image-pro"));
    assert_eq!(nano_request.aspect_ratio.as_deref(), Some("1:1"));
    assert_eq!(nano_request.size.as_deref(), Some("1024x1024"));
    assert_eq!(
        nano_request.callback_url.as_deref(),
        Some("https://app.example.com/hooks/nano"),
    );
}

#[test]
fn maps_nano_banana_reference_images_to_generated_sdk_request() {
    let nano_banana = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Blend reference products into a campaign visual".to_string(),
        negative_prompt: None,
        scene: "campaign_reference".to_string(),
        provider_code: Some("nano-banana".to_string()),
        model: Some("banana-image-pro".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec![
            " https://cdn.example.com/source-a.png ".to_string(),
            "https://cdn.example.com/source-a.png".to_string(),
            "drive://spaces/space-1/nodes/source-b".to_string(),
        ],
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("nano banana reference dispatch plan should build");

    let request = build_nano_banana_image_generation_request(&nano_banana);
    assert_eq!(
        request.images,
        Some(vec![
            "https://cdn.example.com/source-a.png".to_string(),
            "drive://spaces/space-1/nodes/source-b".to_string(),
        ]),
    );
}

#[test]
fn maps_vidu_reference_image_plan_to_generated_sdk_request_and_normalized_task() {
    let vidu = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Reference product into a campaign image".to_string(),
        negative_prompt: Some("distorted label".to_string()),
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
        idempotency_key: None,
    })
    .expect("vidu dispatch plan should build");

    assert_eq!(
        vidu.provider_operation,
        ImageProviderOperation::ViduReferenceToImageGeneration,
    );
    assert_eq!(vidu.task_mode, ImageProviderTaskMode::Task);
    let request = build_vidu_reference_to_image_request(&vidu);
    assert_eq!(request.model, "vidu-image-pro");
    assert_eq!(request.aspect_ratio.as_deref(), Some("3:2"));
    assert_eq!(request.style.as_deref(), Some("cinematic"));
    assert_eq!(
        request.callback_url.as_deref(),
        Some("https://app.example.com/hooks/vidu"),
    );
    assert_eq!(request.images.len(), 2);
    assert!(request.prompt.contains("Negative prompt: distorted label"));

    let normalized = normalize_vidu_image_generation_task_result(
        "vidu",
        ViduImageGenerationTask {
            task_id: Some("vidu-task-001".to_string()),
            state: Some("finished".to_string()),
            model: Some("vidu-image-pro".to_string()),
            creations: Some(vec![ViduCreation {
                id: Some("vidu-creation-001".to_string()),
                image_url: Some("https://provider.example.com/vidu-001.png".to_string()),
                uri: Some("provider://vidu/tasks/vidu-task-001/images/0".to_string()),
                width: Some(1536),
                height: Some(1024),
                ..Default::default()
            }]),
            ..Default::default()
        },
    )
    .expect("vidu task should normalize");

    assert_eq!(normalized.provider_code, "vidu");
    assert_eq!(
        normalized.provider_task_id.as_deref(),
        Some("vidu-task-001")
    );
    assert!(normalized.ready_for_drive_import);
    assert_eq!(normalized.outputs.len(), 1);
    assert_eq!(
        normalized.outputs[0].provider_asset_id.as_deref(),
        Some("vidu-creation-001"),
    );
    assert_eq!(
        normalized.outputs[0].provider_url.as_deref(),
        Some("https://provider.example.com/vidu-001.png"),
    );
}

#[test]
fn declares_provider_gateway_operation_support_without_raw_http_fallbacks() {
    let openai = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "OpenAI sync render".to_string(),
        negative_prompt: None,
        scene: "gateway_support".to_string(),
        provider_code: Some("openai".to_string()),
        model: Some("gpt-image-1".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec![],
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("openai dispatch plan should build");
    let midjourney = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Task render".to_string(),
        negative_prompt: None,
        scene: "gateway_support".to_string(),
        provider_code: Some("midjourney".to_string()),
        model: Some("mj-v7".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec![],
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("midjourney dispatch plan should build");
    let provider_native = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Future provider render".to_string(),
        negative_prompt: None,
        scene: "gateway_support".to_string(),
        provider_code: Some("gemini".to_string()),
        model: Some("gemini-image-pro".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec![],
        webhook_url: Some("https://app.example.com/hooks/gemini".to_string()),
        idempotency_key: None,
    });

    assert!(provider_gateway_supports_create_operation(&openai));
    assert!(provider_gateway_supports_create_operation(&midjourney));
    let vidu = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
        prompt: "Vidu support".to_string(),
        negative_prompt: None,
        scene: "gateway_support".to_string(),
        provider_code: Some("vidu".to_string()),
        model: Some("vidu-image-pro".to_string()),
        resolution: Some("1024x1024".to_string()),
        style: None,
        output_count: Some(1),
        reference_images: vec!["https://cdn.example.com/reference.png".to_string()],
        webhook_url: None,
        idempotency_key: None,
    })
    .expect("vidu dispatch plan should build");
    assert!(provider_gateway_supports_create_operation(&vidu));
    assert_eq!(
        provider_native,
        Err("image generation provider is not exposed by the generated Claw Router SDK"),
    );
    assert!(!provider_gateway_supports_retrieve_operation(&openai));
    assert!(provider_gateway_supports_retrieve_operation(&midjourney));
    assert!(provider_gateway_supports_retrieve_operation(&vidu));
    assert_eq!(
        midjourney.claw_router_retrieve_sdk_resource,
        Some("images_midjourney"),
    );
    assert_eq!(
        midjourney.claw_router_retrieve_sdk_method,
        Some("list_v1_images_generations"),
    );
    assert_eq!(vidu.claw_router_retrieve_sdk_resource, Some("videos_vidu"));
    assert_eq!(
        vidu.claw_router_retrieve_sdk_method,
        Some("list_ent_v2_tasks_creations"),
    );
}
