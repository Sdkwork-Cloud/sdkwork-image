use sdkwork_image_http::{
    app_routes, backend_routes, open_api_routes, required_dual_token_headers, HttpMethod,
    ImageHttpRoute, APP_API_PREFIX, BACKEND_API_PREFIX, OPEN_API_PREFIX,
};

#[test]
fn exposes_standard_app_and_backend_prefixes() {
    assert_eq!(APP_API_PREFIX, "/app/v3/api");
    assert_eq!(BACKEND_API_PREFIX, "/backend/v3/api");
    assert_eq!(OPEN_API_PREFIX, "/image/v3/api");
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
            "missing app image route: {route:?}"
        );
    }
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
            "missing backend image route: {route:?}"
        );
    }
}

#[test]
fn route_catalog_has_stable_operation_surface() {
    let mut app_operation_ids: Vec<&str> = app_routes()
        .iter()
        .map(|route| route.operation_id)
        .collect();
    app_operation_ids.sort();

    assert_eq!(
        app_operation_ids,
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

    let mut backend_operation_ids: Vec<&str> = backend_routes()
        .iter()
        .map(|route| route.operation_id)
        .collect();
    backend_operation_ids.sort();

    assert_eq!(
        backend_operation_ids,
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
fn routes_use_standard_ownership_and_security_contracts() {
    for route in app_routes()
        .into_iter()
        .chain(backend_routes())
        .chain(open_api_routes())
    {
        assert!(
            route.path.starts_with(APP_API_PREFIX)
                || route.path.starts_with(BACKEND_API_PREFIX)
                || route.path.starts_with(OPEN_API_PREFIX)
        );
        assert!(route.path.contains("/image/"));
        assert!(route.operation_id.contains('.'));
        assert!(!route.operation_id.contains('_'));
        assert!(
            !route.operation_id.contains("generationJobs"),
            "external image APIs must use generations.* operationIds",
        );
        assert!(!route.path.contains("studio"));
        assert!(!route.path.contains("appbase"));
        assert!(
            !route.path.contains("generation_jobs"),
            "external image APIs must use /generations paths",
        );
        assert!(
            !route.path.contains("{jobId}"),
            "external image APIs must expose generationId path parameters",
        );
    }

    for route in app_routes().into_iter().chain(backend_routes()) {
        assert_eq!(route.tag, "image");
    }

    assert_eq!(
        required_dual_token_headers(),
        ["Authorization", "Access-Token"]
    );
}
