pub mod http_route_manifest;
pub mod manifest;
pub mod web_bootstrap;

pub use http_route_manifest::open_route_manifest;
pub use manifest::{
    open_api_routes, HttpMethod, ImageHttpRoute, OPEN_API_PREFIX, ROUTE_CRATE_PACKAGE,
};
pub use web_bootstrap::{
    image_open_api_prefixes, image_open_api_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};
