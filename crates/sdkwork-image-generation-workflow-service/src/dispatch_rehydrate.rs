use sdkwork_image_generation_service::{
    ImageProviderDispatchPlan, ImageProviderOperation, ImageProviderTaskMode,
};

use crate::ImageProviderRequestSnapshot;

pub fn rehydrate_image_provider_dispatch_plan(
    snapshot: &ImageProviderRequestSnapshot,
) -> Result<ImageProviderDispatchPlan, &'static str> {
    Ok(ImageProviderDispatchPlan {
        provider_id: snapshot.provider_id.clone(),
        provider_code: snapshot.provider_code.clone(),
        provider_operation: parse_provider_operation(&snapshot.provider_operation)?,
        task_mode: parse_task_mode(&snapshot.task_mode)?,
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
        vendor_parameters: None,
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
        "provider_native.images.generate" => {
            Ok(ImageProviderOperation::ProviderNativeImageGeneration)
        }
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

fn resolve_output_count_parameter(value: &str) -> Result<&'static str, &'static str> {
    match value.trim() {
        "n" => Ok("n"),
        _ => Err("stored image output_count provider parameter is unsupported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider_request_snapshot_from_dispatch_plan_for_test;
    use sdkwork_image_generation_service::plan_image_generation_provider_dispatch;
    use sdkwork_image_generation_service::ImageGenerationCreateCommand;

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
        assert_eq!(restored.provider_id, dispatch.provider_id);
        assert_eq!(restored.task_mode, dispatch.task_mode);
    }
}
