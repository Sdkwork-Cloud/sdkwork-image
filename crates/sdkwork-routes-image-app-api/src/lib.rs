pub mod manifest;

pub use manifest::{
    app_routes, required_dual_token_headers, HttpMethod, ImageHttpRoute, APP_API_PREFIX,
    ROUTE_CRATE_PACKAGE,
};

pub fn gateway_mount() -> axum::Router {
    axum::Router::new()
}
