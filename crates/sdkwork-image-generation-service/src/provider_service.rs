use std::sync::Arc;

use async_trait::async_trait;
use sdkwork_image_generation_provider_spi::{
    ImageGenerationCommand, ImageGenerationProviderDescriptor, ImageGenerationProviderError,
    ImageGenerationProviderRegistry, ImageGenerationProviderResult, ImageProviderDispatchPlan,
    ImageProviderSubmission, ImageVendorId, NormalizedProviderGenerationResult,
};

#[async_trait]
pub trait ImageGenerationServicePort: Send + Sync {
    async fn generate(
        &self,
        command: ImageGenerationCommand,
    ) -> ImageGenerationProviderResult<ImageProviderSubmission>;

    async fn retrieve(
        &self,
        dispatch_plan: &ImageProviderDispatchPlan,
        provider_task_id: &str,
    ) -> ImageGenerationProviderResult<NormalizedProviderGenerationResult>;

    async fn cancel(
        &self,
        dispatch_plan: &ImageProviderDispatchPlan,
        provider_task_id: &str,
    ) -> ImageGenerationProviderResult<NormalizedProviderGenerationResult>;

    fn provider_descriptors(&self) -> Vec<ImageGenerationProviderDescriptor>;
}

#[derive(Clone)]
pub struct ImageGenerationService {
    providers: Arc<ImageGenerationProviderRegistry>,
}

impl ImageGenerationService {
    pub fn new(providers: ImageGenerationProviderRegistry) -> Self {
        Self {
            providers: Arc::new(providers),
        }
    }

    fn provider_for_dispatch(
        &self,
        dispatch_plan: &ImageProviderDispatchPlan,
    ) -> ImageGenerationProviderResult<
        Arc<dyn sdkwork_image_generation_provider_spi::ImageGenerationProvider>,
    > {
        if !dispatch_plan.provider_id.trim().is_empty() {
            return self.providers.select_by_id(&dispatch_plan.provider_id);
        }
        let vendor = ImageVendorId::new(&dispatch_plan.provider_code)?;
        self.providers.select_for_vendor(&vendor)
    }
}

#[async_trait]
impl ImageGenerationServicePort for ImageGenerationService {
    async fn generate(
        &self,
        command: ImageGenerationCommand,
    ) -> ImageGenerationProviderResult<ImageProviderSubmission> {
        let provider = self.providers.select_for_vendor(&command.vendor)?;
        provider.validate(&command)?;
        provider.generate(&command).await
    }

    async fn retrieve(
        &self,
        dispatch_plan: &ImageProviderDispatchPlan,
        provider_task_id: &str,
    ) -> ImageGenerationProviderResult<NormalizedProviderGenerationResult> {
        if provider_task_id.trim().is_empty() {
            return Err(ImageGenerationProviderError::InvalidRequest(
                "provider_task_id is required".to_string(),
            ));
        }
        self.provider_for_dispatch(dispatch_plan)?
            .retrieve(dispatch_plan, provider_task_id.trim())
            .await
    }

    async fn cancel(
        &self,
        dispatch_plan: &ImageProviderDispatchPlan,
        provider_task_id: &str,
    ) -> ImageGenerationProviderResult<NormalizedProviderGenerationResult> {
        if provider_task_id.trim().is_empty() {
            return Err(ImageGenerationProviderError::InvalidRequest(
                "provider_task_id is required".to_string(),
            ));
        }
        self.provider_for_dispatch(dispatch_plan)?
            .cancel(dispatch_plan, provider_task_id.trim())
            .await
    }

    fn provider_descriptors(&self) -> Vec<ImageGenerationProviderDescriptor> {
        self.providers.descriptors()
    }
}
