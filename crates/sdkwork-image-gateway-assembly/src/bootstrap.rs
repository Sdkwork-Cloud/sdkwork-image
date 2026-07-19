//! Gateway bootstrap for sdkwork-image.
//! Generation app-api routes require an injected `ImageGenerationHost`.

use axum::Router;
use sdkwork_image_generation_host::ImageGenerationHost;
use std::sync::Arc;

pub struct ApplicationAssembly {
    pub router: Router,
    pub background_processor: Option<tokio::task::JoinHandle<()>>,
}

pub async fn assemble_application_router_from_env() -> Result<ApplicationAssembly, String> {
    let host = ImageGenerationHost::from_runtime_env().await?;
    Ok(assemble_application_router(host).await)
}

pub async fn assemble_application_router(
    generation_host: Arc<ImageGenerationHost>,
) -> ApplicationAssembly {
    let background_processor = generation_host.spawn_background_processor_if_enabled();
    let mut router = Router::new();
    router =
        router.merge(sdkwork_routes_image_app_api::gateway_mount(generation_host.clone()).await);
    router = router.merge(sdkwork_routes_image_backend_api::gateway_mount());
    router = router.merge(sdkwork_routes_image_open_api::gateway_mount());
    ApplicationAssembly {
        router,
        background_processor,
    }
}
