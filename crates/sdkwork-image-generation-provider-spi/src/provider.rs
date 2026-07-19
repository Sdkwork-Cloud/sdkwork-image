use async_trait::async_trait;

use crate::{
    ImageGenerationCommand, ImageGenerationProviderResult, ImageProviderDispatchPlan,
    ImageVendorId, NormalizedProviderGenerationResult,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageGenerationProviderCapability {
    TextToImage,
    ReferenceToImage,
    MultipleOutputs,
    NegativePrompt,
    Seed,
    Polling,
    Webhook,
    Cancellation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationProviderDescriptor {
    pub id: String,
    pub vendors: Vec<ImageVendorId>,
    pub capabilities: Vec<ImageGenerationProviderCapability>,
}

impl ImageGenerationProviderDescriptor {
    pub fn supports_vendor(&self, vendor: &ImageVendorId) -> bool {
        self.vendors.iter().any(|candidate| candidate == vendor)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationProviderHealth {
    pub available: bool,
    pub detail: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageProviderSubmission {
    pub dispatch_plan: ImageProviderDispatchPlan,
    pub result: NormalizedProviderGenerationResult,
}

#[async_trait]
pub trait ImageGenerationProvider: Send + Sync {
    fn descriptor(&self) -> &ImageGenerationProviderDescriptor;

    fn validate(&self, command: &ImageGenerationCommand) -> ImageGenerationProviderResult<()>;

    async fn generate(
        &self,
        command: &ImageGenerationCommand,
    ) -> ImageGenerationProviderResult<ImageProviderSubmission>;

    async fn retrieve(
        &self,
        dispatch_plan: &ImageProviderDispatchPlan,
        provider_task_id: &str,
    ) -> ImageGenerationProviderResult<NormalizedProviderGenerationResult>;

    async fn cancel(
        &self,
        _dispatch_plan: &ImageProviderDispatchPlan,
        _provider_task_id: &str,
    ) -> ImageGenerationProviderResult<NormalizedProviderGenerationResult> {
        Err(crate::ImageGenerationProviderError::UnsupportedCapability(
            "cancellation".to_string(),
        ))
    }

    async fn health(&self) -> ImageGenerationProviderResult<ImageGenerationProviderHealth> {
        Ok(ImageGenerationProviderHealth {
            available: true,
            detail: None,
        })
    }
}
