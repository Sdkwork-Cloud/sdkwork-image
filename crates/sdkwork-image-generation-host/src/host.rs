use std::sync::Arc;

use clawrouter_open_sdk::{SdkworkAiClient, SdkworkConfig};
use sdkwork_image_claw_router_provider_service::ClawRouterImageProviderGateway;
use sdkwork_image_generation_repository_sqlx::{
    connect_and_bootstrap_image_database_from_env, GenerationProjectionRepository,
    ImageGenerationBackgroundRepository, InMemoryGenerationProjectionRepository,
    InMemoryImageCatalogRepository, SqlxGenerationProjectionRepository,
    SqlxImageCatalogRepository, SqlxImageGenerationBackgroundRepository,
};

use crate::background::{
    image_background_processor_enabled_from_env, spawn_image_generation_background_processor,
};
use crate::catalog_service::ImageCatalogService;
use crate::service::ImageGenerationService;

pub struct ImageGenerationHost {
    service: Arc<ImageGenerationService>,
    catalog: Arc<ImageCatalogService>,
    gateway: ClawRouterImageProviderGateway,
    background: Option<Arc<dyn ImageGenerationBackgroundRepository>>,
}

impl ImageGenerationHost {
    pub fn new(
        gateway: ClawRouterImageProviderGateway,
        store: Arc<dyn GenerationProjectionRepository>,
        catalog_store: Arc<dyn sdkwork_image_generation_repository_sqlx::ImageCatalogRepository>,
        drive_import: Option<Arc<sdkwork_image_generation_runtime_service::ImageDriveImportRuntime>>,
        background: Option<Arc<dyn ImageGenerationBackgroundRepository>>,
    ) -> Arc<Self> {
        Arc::new(Self {
            service: Arc::new(ImageGenerationService::new(
                gateway.clone(),
                store,
                drive_import,
            )),
            catalog: Arc::new(ImageCatalogService::new(catalog_store)),
            gateway,
            background,
        })
    }

    pub fn from_gateway_with_store(
        gateway: ClawRouterImageProviderGateway,
        store: Arc<dyn GenerationProjectionRepository>,
        catalog_store: Arc<dyn sdkwork_image_generation_repository_sqlx::ImageCatalogRepository>,
        drive_import: Option<Arc<sdkwork_image_generation_runtime_service::ImageDriveImportRuntime>>,
        background: Option<Arc<dyn ImageGenerationBackgroundRepository>>,
    ) -> Arc<Self> {
        Self::new(gateway, store, catalog_store, drive_import, background)
    }

    pub fn for_test(gateway: ClawRouterImageProviderGateway) -> Arc<Self> {
        Self::new(
            gateway,
            InMemoryGenerationProjectionRepository::new(),
            InMemoryImageCatalogRepository::new(),
            None,
            None,
        )
    }

    pub async fn from_runtime_env() -> Result<Arc<Self>, String> {
        let gateway = claw_router_gateway_from_env()?;
        let database = connect_and_bootstrap_image_database_from_env().await?;
        let pool = database.pool().clone();
        let store = SqlxGenerationProjectionRepository::new(pool.clone());
        let catalog_store = SqlxImageCatalogRepository::new(pool.clone());
        let background: Arc<dyn ImageGenerationBackgroundRepository> =
            SqlxImageGenerationBackgroundRepository::new(pool);
        let drive_import = sdkwork_image_generation_runtime_service::ImageDriveImportRuntime::try_from_env()
            .await?;
        Ok(Self::new(
            gateway,
            store,
            catalog_store,
            drive_import,
            Some(background),
        ))
    }

    pub fn service(&self) -> Arc<ImageGenerationService> {
        self.service.clone()
    }

    pub fn catalog(&self) -> Arc<ImageCatalogService> {
        self.catalog.clone()
    }

    pub fn gateway(&self) -> &ClawRouterImageProviderGateway {
        &self.gateway
    }

    pub fn spawn_background_processor_if_enabled(&self) -> Option<tokio::task::JoinHandle<()>> {
        if !image_background_processor_enabled_from_env() {
            return None;
        }
        let background = self.background.clone()?;
        Some(spawn_image_generation_background_processor(
            self.service.clone(),
            background,
        ))
    }
}

fn claw_router_gateway_from_env() -> Result<ClawRouterImageProviderGateway, String> {
    let base_url = std::env::var("SDKWORK_CLAWROUTER_OPEN_API_BASE_URL")
        .or_else(|_| std::env::var("CLAWROUTER_OPEN_API_BASE_URL"))
        .map_err(|_| {
            "SDKWORK_CLAWROUTER_OPEN_API_BASE_URL (or CLAWROUTER_OPEN_API_BASE_URL) is required"
                .to_string()
        })?;
    let client = SdkworkAiClient::new(SdkworkConfig::new(base_url.trim()))
        .map_err(|error| format!("claw router client init failed: {error}"))?;
    if let Ok(api_key) = std::env::var("SDKWORK_CLAWROUTER_OPEN_API_KEY") {
        if !api_key.trim().is_empty() {
            client.set_api_key(api_key.trim());
        }
    }
    Ok(ClawRouterImageProviderGateway::new(client))
}

impl std::fmt::Debug for ImageGenerationHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImageGenerationHost").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_wraps_gateway_and_service() {
        let client =
            SdkworkAiClient::new(SdkworkConfig::new("http://127.0.0.1:0")).expect("client");
        let host = ImageGenerationHost::for_test(ClawRouterImageProviderGateway::new(client));
        assert!(Arc::strong_count(&host.service()) >= 1);
    }
}
