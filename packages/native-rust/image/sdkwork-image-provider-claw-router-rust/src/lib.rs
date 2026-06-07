mod gateway;
mod normalization;
mod requests;

pub use gateway::{
    provider_gateway_supports_create_operation, provider_gateway_supports_retrieve_operation,
    ClawRouterImageProviderGateway, CLAW_ROUTER_IMAGE_GENERATION_METHOD,
    CLAW_ROUTER_OPEN_SDK_CRATE,
};
pub use normalization::{
    normalize_vidu_image_generation_task_result, normalize_vidu_task_creations_result,
};
pub use requests::{
    build_midjourney_image_generation_request, build_nano_banana_image_generation_request,
    build_openai_image_generation_request, build_vidu_reference_to_image_request,
    openai_image_generation_sdk_supports_output_count,
};
