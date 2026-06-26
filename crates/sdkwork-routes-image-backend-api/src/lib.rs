pub mod manifest;

pub use manifest::{
    backend_routes, required_dual_token_headers, HttpMethod, ImageHttpRoute, BACKEND_API_PREFIX,
    ROUTE_CRATE_PACKAGE,
};

pub fn gateway_mount() -> axum::Router {
    axum::Router::new()
}
