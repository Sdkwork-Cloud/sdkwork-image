//! Stable provider port for SDKWork image generation.

mod error;
mod model;
mod normalization;
mod provider;
mod registry;

pub use error::{ImageGenerationProviderError, ImageGenerationProviderResult};
pub use model::{
    plan_image_generation_provider_dispatch, plan_unified_image_generation_provider_dispatch,
    GeneratedMediaKind, GeneratedMediaOutput, ImageGenerationCommand, ImageGenerationCreateCommand,
    ImageGenerationGeometry, ImageGenerationModelSelection, ImageGenerationReference,
    ImageGenerationRuntimeStatus, ImageGenerationVendorParameters, ImageJobStatus,
    ImageProviderDispatchPlan, ImageProviderOperation, ImageProviderTaskMode, ImageVendorId,
    NormalizedProviderGenerationResult, OpenAiGeneratedImage, ProviderGeneratedMediaAsset,
    ProviderTaskErrorSnapshot, ProviderTaskSnapshot,
};
pub use normalization::{
    normalize_openai_image_generation_outputs, normalize_provider_task_generation_result,
};
pub use provider::{
    ImageGenerationProvider, ImageGenerationProviderCapability, ImageGenerationProviderDescriptor,
    ImageGenerationProviderHealth, ImageProviderSubmission,
};
pub use registry::{ImageGenerationProviderRegistry, ImageGenerationProviderRegistryBuilder};
