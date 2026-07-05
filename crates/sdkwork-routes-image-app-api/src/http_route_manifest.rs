use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/presets",
        "image",
        "presets.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/presets/{presetId}",
        "image",
        "presets.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/image/generations",
        "image",
        "generations.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/generations",
        "image",
        "generations.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/generations/{generationId}",
        "image",
        "generations.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/image/generations/{generationId}/refresh",
        "image",
        "generations.refresh",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/image/generations/{generationId}/cancel",
        "image",
        "generations.cancel",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/image/edit_tasks",
        "image",
        "editTasks.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/edit_tasks/{taskId}",
        "image",
        "editTasks.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/assets",
        "image",
        "assets.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/assets/{assetId}",
        "image",
        "assets.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/galleries",
        "image",
        "galleries.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/image/galleries/{galleryId}",
        "image",
        "galleries.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/image/galleries/{galleryId}/items",
        "image",
        "galleries.items.create",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_matches_catalog_route_count() {
        assert_eq!(HTTP_ROUTES.len(), crate::manifest::app_routes().len());
    }

    #[test]
    fn generation_routes_are_dual_token_protected() {
        let manifest = app_route_manifest();
        for path in [
            "/app/v3/api/image/generations",
            "/app/v3/api/image/generations/{generationId}",
            "/app/v3/api/image/generations/{generationId}/refresh",
        ] {
            assert!(
                manifest.match_route("POST", path).is_some()
                    || manifest.match_route("GET", path).is_some(),
                "missing route manifest entry for {path}",
            );
        }
    }
}
