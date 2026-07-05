mod api_response;
mod http_route_manifest;
pub mod manifest;
mod routes;
mod subject;
mod web_bootstrap;

pub use http_route_manifest::app_route_manifest;
pub use manifest::{
    app_routes, required_dual_token_headers, HttpMethod, ImageHttpRoute, APP_API_PREFIX,
    ROUTE_CRATE_PACKAGE,
};
pub use web_bootstrap::{
    image_app_api_prefixes, image_app_api_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

use std::sync::Arc;

use axum::Router;
use sdkwork_image_generation_host::ImageGenerationHost;
use sdkwork_web_core::HttpRouteManifest;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    app_route_manifest()
}

pub fn build_app_router(host: Arc<ImageGenerationHost>) -> Router {
    routes::build_image_app_router(host)
}

pub async fn build_app_router_with_framework(host: Arc<ImageGenerationHost>) -> Router {
    wrap_router_with_web_framework_from_env(build_app_router(host)).await
}

pub async fn gateway_mount(host: Arc<ImageGenerationHost>) -> Router {
    build_app_router_with_framework(host).await
}
