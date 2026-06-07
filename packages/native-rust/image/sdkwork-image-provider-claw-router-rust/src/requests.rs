use clawrouter_open_sdk::{
    MidjourneyImageGenerationRequest, NanoBananaImageGenerationRequest,
    OpenAiImageGenerationRequest, ViduReferenceToImageRequest,
};
use sdkwork_image_core::ImageProviderDispatchPlan;

pub fn build_openai_image_generation_request(
    plan: &ImageProviderDispatchPlan,
) -> OpenAiImageGenerationRequest {
    OpenAiImageGenerationRequest {
        model: plan
            .model
            .clone()
            .unwrap_or_else(|| plan.provider_code.clone()),
        n: Some(plan.output_count.into()),
        prompt: plan.prompt.clone(),
        quality: plan.quality.clone(),
        response_format: plan.response_format.clone(),
        size: plan.size.clone(),
    }
}

pub fn build_midjourney_image_generation_request(
    plan: &ImageProviderDispatchPlan,
) -> MidjourneyImageGenerationRequest {
    MidjourneyImageGenerationRequest {
        aspect_ratio: aspect_ratio_from_size(plan.size.as_deref()),
        callback_url: plan.callback_url.clone(),
        model: plan.model.clone(),
        prompt: prompt_with_optional_negative(plan),
        seed: None,
        style: plan.quality.clone(),
    }
}

pub fn build_nano_banana_image_generation_request(
    plan: &ImageProviderDispatchPlan,
) -> NanoBananaImageGenerationRequest {
    NanoBananaImageGenerationRequest {
        aspect_ratio: aspect_ratio_from_size(plan.size.as_deref()),
        callback_url: plan.callback_url.clone(),
        images: (!plan.reference_images.is_empty()).then(|| plan.reference_images.clone()),
        model: plan.model.clone(),
        prompt: prompt_with_optional_negative(plan),
        seed: None,
        size: plan.size.clone(),
    }
}

pub fn build_vidu_reference_to_image_request(
    plan: &ImageProviderDispatchPlan,
) -> ViduReferenceToImageRequest {
    ViduReferenceToImageRequest {
        aspect_ratio: aspect_ratio_from_size(plan.size.as_deref()),
        callback_url: plan.callback_url.clone(),
        images: plan.reference_images.clone(),
        model: plan
            .model
            .clone()
            .unwrap_or_else(|| plan.provider_code.clone()),
        payload: None,
        prompt: prompt_with_optional_negative(plan),
        seed: None,
        style: plan.quality.clone(),
    }
}

pub fn openai_image_generation_sdk_supports_output_count() -> bool {
    true
}

fn prompt_with_optional_negative(plan: &ImageProviderDispatchPlan) -> String {
    match plan.negative_prompt.as_deref() {
        Some(negative_prompt) if !negative_prompt.trim().is_empty() => {
            format!(
                "{}\n\nNegative prompt: {}",
                plan.prompt,
                negative_prompt.trim()
            )
        }
        _ => plan.prompt.clone(),
    }
}

fn aspect_ratio_from_size(size: Option<&str>) -> Option<String> {
    let size = size?.trim();
    let (width, height) = size.split_once('x')?;
    let width = width.trim().parse::<i64>().ok()?;
    let height = height.trim().parse::<i64>().ok()?;
    if width <= 0 || height <= 0 {
        return None;
    }
    let divisor = gcd(width, height);
    Some(format!("{}:{}", width / divisor, height / divisor))
}

fn gcd(mut left: i64, mut right: i64) -> i64 {
    while right != 0 {
        let next = left % right;
        left = right;
        right = next;
    }
    left.abs().max(1)
}
