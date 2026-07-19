use std::sync::Arc;

use rmcp::{
    service::{RunningService, ServerInitializeError},
    transport::streamable_http_server::{
        session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
    },
    RoleServer, ServiceExt,
};

use crate::ImageGenerationMcpService;

pub type ImageGenerationMcpHttpService =
    StreamableHttpService<ImageGenerationMcpService, LocalSessionManager>;

pub fn streamable_http_service(
    service: ImageGenerationMcpService,
    config: StreamableHttpServerConfig,
) -> ImageGenerationMcpHttpService {
    StreamableHttpService::new(
        move || Ok(service.clone()),
        Arc::new(LocalSessionManager::default()),
        config,
    )
}

pub async fn serve_stdio(
    service: ImageGenerationMcpService,
) -> Result<RunningService<RoleServer, ImageGenerationMcpService>, ServerInitializeError> {
    service.serve(rmcp::transport::stdio()).await
}
