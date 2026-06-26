use sdkwork_routes_image_app_api::{
    app_routes, required_dual_token_headers, HttpMethod, ImageHttpRoute, APP_API_PREFIX,
    ROUTE_CRATE_PACKAGE,
};

#[test]
fn exposes_app_api_route_manifest_identity() {
    assert_eq!(ROUTE_CRATE_PACKAGE, "sdkwork-routes-image-app-api");
    assert_eq!(APP_API_PREFIX, "/app/v3/api");
}

#[test]
fn app_routes_expose_user_facing_image_workspace_operations() {
    let routes = app_routes();

    for route in [
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/presets",
            "image",
            "presets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/app/v3/api/image/generations",
            "image",
            "generations.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/generations",
            "image",
            "generations.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/app/v3/api/image/generations/{generationId}/refresh",
            "image",
            "generations.refresh",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/assets",
            "image",
            "assets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/galleries",
            "image",
            "galleries.list",
        ),
    ] {
        assert!(
            routes.contains(&route),
            "missing app image route: {route:?}",
        );
    }
}

#[test]
fn route_catalog_has_stable_app_operation_surface() {
    let mut operation_ids: Vec<&str> = app_routes()
        .iter()
        .map(|route| route.operation_id)
        .collect();
    operation_ids.sort();

    assert_eq!(
        operation_ids,
        vec![
            "assets.list",
            "assets.retrieve",
            "editTasks.create",
            "editTasks.retrieve",
            "galleries.items.create",
            "galleries.list",
            "galleries.retrieve",
            "generations.cancel",
            "generations.create",
            "generations.list",
            "generations.refresh",
            "generations.retrieve",
            "presets.list",
            "presets.retrieve",
        ],
    );
}

#[test]
fn app_routes_use_standard_surface_and_security_contracts() {
    for route in app_routes() {
        assert!(route.path.starts_with(APP_API_PREFIX));
        assert!(!route.path.starts_with("/backend/v3/api"));
        assert!(!route.path.starts_with("/image/v3/api"));
        assert!(route.path.contains("/image/"));
        assert_eq!(route.tag, "image");
        assert!(route.operation_id.contains('.'));
        assert!(!route.operation_id.contains('_'));
        assert!(!route.path.contains("generation_jobs"));
        assert!(!route.path.contains("{jobId}"));
    }

    assert_eq!(
        required_dual_token_headers(),
        ["Authorization", "Access-Token"],
    );
}
