use clawrouter_open_sdk::{OpenAiImageGenerationRequest, SdkworkAiClient};
use sdkwork_image_core::{
    plan_image_generation_provider_dispatch, ImageGenerationCreateCommand, ImageProviderOperation,
    ImageProviderTaskMode,
};
use sdkwork_image_provider_claw_router::{
    build_midjourney_image_generation_request, build_nano_banana_image_generation_request,
    build_openai_image_generation_request, openai_image_generation_sdk_supports_output_count,
    provider_gateway_supports_create_operation, provider_gateway_supports_retrieve_operation,
    CLAW_ROUTER_IMAGE_GENERATION_METHOD, CLAW_ROUTER_OPEN_SDK_CRATE,
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
        webhook_url: Some("https://app.example.com/hooks/gemini".to_string()),
        idempotency_key: None,
    })
    .expect("provider-native dispatch plan should build");

    assert!(provider_gateway_supports_create_operation(&openai));
    assert!(provider_gateway_supports_create_operation(&midjourney));
    assert!(!provider_gateway_supports_create_operation(
        &provider_native
    ));
    assert!(!provider_gateway_supports_retrieve_operation(&openai));
    assert!(provider_gateway_supports_retrieve_operation(&midjourney));
    assert!(!provider_gateway_supports_retrieve_operation(
        &provider_native
    ));
}
