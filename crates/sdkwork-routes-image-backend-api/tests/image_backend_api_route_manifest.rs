use sdkwork_routes_image_backend_api::{
    backend_routes, required_dual_token_headers, HttpMethod, ImageHttpRoute, BACKEND_API_PREFIX,
    ROUTE_CRATE_PACKAGE,
};

#[test]
fn exposes_backend_api_route_manifest_identity() {
    assert_eq!(ROUTE_CRATE_PACKAGE, "sdkwork-routes-image-backend-api");
    assert_eq!(BACKEND_API_PREFIX, "/backend/v3/api");
}

#[test]
fn backend_routes_expose_image_management_operations() {
    let routes = backend_routes();

    for route in [
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/presets",
            "image",
            "presets.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Patch,
            "/backend/v3/api/image/presets/{presetId}",
            "image",
            "presets.update",
        ),
        ImageHttpRoute::new(
            HttpMethod::Delete,
            "/backend/v3/api/image/presets/{presetId}",
            "image",
            "presets.delete",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/generations",
            "image",
            "generations.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/generations/{generationId}/retry",
            "image",
            "generations.retry",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/provider_webhooks/{providerCode}",
            "image",
            "providerWebhooks.receive",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/assets",
            "image",
            "assets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Patch,
            "/backend/v3/api/image/galleries/{galleryId}",
            "image",
            "galleries.update",
        ),
    ] {
        assert!(
            routes.contains(&route),
            "missing backend image route: {route:?}",
        );
    }
}

#[test]
fn route_catalog_has_stable_backend_operation_surface() {
    let mut operation_ids: Vec<&str> = backend_routes()
        .iter()
        .map(|route| route.operation_id)
        .collect();
    operation_ids.sort();

    assert_eq!(
        operation_ids,
        vec![
            "assets.delete",
            "assets.list",
            "assets.retrieve",
            "assets.update",
            "editTasks.list",
            "editTasks.retrieve",
            "galleries.create",
            "galleries.delete",
            "galleries.items.create",
            "galleries.items.delete",
            "galleries.items.list",
            "galleries.list",
            "galleries.retrieve",
            "galleries.update",
            "generations.cancel",
            "generations.list",
            "generations.refresh",
            "generations.retrieve",
            "generations.retry",
            "presets.create",
            "presets.delete",
            "presets.list",
            "presets.retrieve",
            "presets.update",
            "providerWebhooks.receive",
        ],
    );
}

#[test]
fn backend_routes_use_standard_surface_and_security_contracts() {
    for route in backend_routes() {
        assert!(route.path.starts_with(BACKEND_API_PREFIX));
        assert!(!route.path.starts_with("/app/v3/api"));
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
