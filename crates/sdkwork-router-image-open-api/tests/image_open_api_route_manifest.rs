use sdkwork_router_image_open_api::{
    open_api_routes, HttpMethod, ImageHttpRoute, OPEN_API_PREFIX, ROUTE_CRATE_PACKAGE,
};

#[test]
fn exposes_open_api_route_manifest_identity() {
    assert_eq!(ROUTE_CRATE_PACKAGE, "sdkwork-router-image-open-api");
    assert_eq!(OPEN_API_PREFIX, "/image/v3/api");
}

#[test]
fn open_api_routes_expose_image_owned_openai_compatibility_operations() {
    let routes = open_api_routes();

    assert_eq!(
        routes,
        vec![ImageHttpRoute::new(
            HttpMethod::Post,
            "/image/v3/api/compat/openai/images/generations",
            "imageCompat",
            "compat.openai.images.generate",
        )],
    );
}

#[test]
fn open_api_routes_use_only_image_open_api_prefix() {
    for route in open_api_routes() {
        assert!(route.path.starts_with(OPEN_API_PREFIX));
        assert!(!route.path.starts_with("/app/v3/api"));
        assert!(!route.path.starts_with("/backend/v3/api"));
        assert!(route.operation_id.contains('.'));
        assert!(!route.operation_id.contains('_'));
        assert!(!route.path.contains("generation_jobs"));
        assert!(!route.path.contains("{jobId}"));
    }
}
