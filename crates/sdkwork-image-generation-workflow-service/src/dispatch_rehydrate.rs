use sdkwork_image_generation_service::{
    ImageProviderDispatchPlan, ImageProviderOperation, ImageProviderTaskMode,
};

use crate::ImageProviderRequestSnapshot;

pub fn rehydrate_image_provider_dispatch_plan(
    snapshot: &ImageProviderRequestSnapshot,
) -> Result<ImageProviderDispatchPlan, &'static str> {
    Ok(ImageProviderDispatchPlan {
        provider_code: snapshot.provider_code.clone(),
        provider_operation: parse_provider_operation(&snapshot.provider_operation)?,
        task_mode: parse_task_mode(&snapshot.task_mode)?,
        claw_router_api_path: resolve_api_path(&snapshot.api_path)?,
        claw_router_sdk_resource: resolve_sdk_token(&snapshot.sdk_resource)?,
        claw_router_sdk_method: resolve_sdk_token(&snapshot.sdk_method)?,
        claw_router_retrieve_api_path: snapshot
            .retrieve_api_path
            .as_deref()
            .map(resolve_api_path)
            .transpose()?,
        claw_router_retrieve_sdk_resource: snapshot
            .retrieve_sdk_resource
            .as_deref()
            .map(resolve_sdk_token)
            .transpose()?,
        claw_router_retrieve_sdk_method: snapshot
            .retrieve_sdk_method
            .as_deref()
            .map(resolve_sdk_token)
            .transpose()?,
        prompt: snapshot.prompt.clone(),
        negative_prompt: snapshot.negative_prompt.clone(),
        model: snapshot.model.clone(),
        size: snapshot.size.clone(),
        quality: snapshot.quality.clone(),
        response_format: Some("url".to_string()),
        output_count: snapshot.output_count,
        output_count_provider_parameter: snapshot
            .output_count_provider_parameter
            .as_deref()
            .map(resolve_output_count_parameter)
            .transpose()?,
        reference_images: snapshot.reference_images.clone(),
        callback_url: snapshot.callback_url.clone(),
        idempotency_key: snapshot.idempotency_key.clone(),
    })
}

fn parse_provider_operation(value: &str) -> Result<ImageProviderOperation, &'static str> {
    match value.trim() {
        "openai.images.generate" => Ok(ImageProviderOperation::OpenAiImageGeneration),
        "midjourney.images.generate" => Ok(ImageProviderOperation::MidjourneyImageGeneration),
        "nano_banana.images.generate" => Ok(ImageProviderOperation::NanoBananaImageGeneration),
        "vidu.images.reference_to_image" => {
            Ok(ImageProviderOperation::ViduReferenceToImageGeneration)
        }
        "provider_native.images.generate" => Ok(ImageProviderOperation::ProviderNativeImageGeneration),
        _ => Err("stored image provider_operation is unsupported"),
    }
}

fn parse_task_mode(value: &str) -> Result<ImageProviderTaskMode, &'static str> {
    match value.trim() {
        "sync" => Ok(ImageProviderTaskMode::Synchronous),
        "task" => Ok(ImageProviderTaskMode::Task),
        "webhook" => Ok(ImageProviderTaskMode::Webhook),
        _ => Err("stored image provider task_mode is unsupported"),
    }
}

fn resolve_api_path(value: &str) -> Result<&'static str, &'static str> {
    match value.trim() {
        "/v1/images/generations" => Ok("/v1/images/generations"),
        "/midjourney/v1/images/generations" => Ok("/midjourney/v1/images/generations"),
        "/midjourney/v1/images/generations/{task_id}" => {
            Ok("/midjourney/v1/images/generations/{task_id}")
        }
        "/nano-banana/v1/images/generations" => Ok("/nano-banana/v1/images/generations"),
        "/nano-banana/v1/images/generations/{task_id}" => {
            Ok("/nano-banana/v1/images/generations/{task_id}")
        }
        "/vidu/ent/v2/reference2image" => Ok("/vidu/ent/v2/reference2image"),
        "/vidu/ent/v2/tasks/{task_id}/creations" => Ok("/vidu/ent/v2/tasks/{task_id}/creations"),
        _ => Err("stored image provider api_path is unsupported"),
    }
}

fn resolve_sdk_token(value: &str) -> Result<&'static str, &'static str> {
    match value.trim() {
        "images" => Ok("images"),
        "images_midjourney" => Ok("images_midjourney"),
        "images_nano_banana" => Ok("images_nano_banana"),
        "images_vidu" => Ok("images_vidu"),
        "videos_vidu" => Ok("videos_vidu"),
        "create_generation" => Ok("create_generation"),
        "create_v1_images_generation" => Ok("create_v1_images_generation"),
        "create_generations" => Ok("create_generations"),
        "create_ent_v2_reference2image" => Ok("create_ent_v2_reference2image"),
        "list_v1_images_generations" => Ok("list_v1_images_generations"),
        "retrieve_generations" => Ok("retrieve_generations"),
        "list_ent_v2_tasks_creations" => Ok("list_ent_v2_tasks_creations"),
        _ => Err("stored image provider sdk token is unsupported"),
    }
}

fn resolve_output_count_parameter(value: &str) -> Result<&'static str, &'static str> {
    match value.trim() {
        "n" => Ok("n"),
        _ => Err("stored image output_count provider parameter is unsupported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdkwork_image_generation_service::ImageGenerationCreateCommand;
    use sdkwork_image_generation_service::plan_image_generation_provider_dispatch;
    use crate::provider_request_snapshot_from_dispatch_plan_for_test;

    #[test]
    fn rehydrates_dispatch_plan_from_provider_request_snapshot() {
        let dispatch = plan_image_generation_provider_dispatch(&ImageGenerationCreateCommand {
            prompt: "hero".to_string(),
            negative_prompt: None,
            scene: "playground_image".to_string(),
            provider_code: Some("openai".to_string()),
            model: None,
            resolution: None,
            style: None,
            output_count: Some(1),
            reference_images: Vec::new(),
            webhook_url: None,
            idempotency_key: None,
        })
        .expect("dispatch");
        let snapshot = provider_request_snapshot_from_dispatch_plan_for_test(&dispatch);
        let restored = rehydrate_image_provider_dispatch_plan(&snapshot).expect("restore");
        assert_eq!(restored.provider_code, dispatch.provider_code);
        assert_eq!(restored.provider_operation, dispatch.provider_operation);
        assert_eq!(restored.claw_router_api_path, dispatch.claw_router_api_path);
        assert_eq!(
            restored.claw_router_sdk_method,
            dispatch.claw_router_sdk_method
        );
    }
}
