pub const ROUTE_CRATE_PACKAGE: &str = "sdkwork-router-image-backend-api";
pub const BACKEND_API_PREFIX: &str = "/backend/v3/api";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HttpMethod {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageHttpRoute {
    pub method: HttpMethod,
    pub path: &'static str,
    pub tag: &'static str,
    pub operation_id: &'static str,
}

impl ImageHttpRoute {
    pub const fn new(
        method: HttpMethod,
        path: &'static str,
        tag: &'static str,
        operation_id: &'static str,
    ) -> Self {
        Self {
            method,
            path,
            tag,
            operation_id,
        }
    }
}

pub fn backend_routes() -> Vec<ImageHttpRoute> {
    vec![
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/presets",
            "image",
            "presets.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/presets",
            "image",
            "presets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/presets/{presetId}",
            "image",
            "presets.retrieve",
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
            HttpMethod::Get,
            "/backend/v3/api/image/generations/{generationId}",
            "image",
            "generations.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/generations/{generationId}/refresh",
            "image",
            "generations.refresh",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/generations/{generationId}/retry",
            "image",
            "generations.retry",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/generations/{generationId}/cancel",
            "image",
            "generations.cancel",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/provider_webhooks/{providerCode}",
            "image",
            "providerWebhooks.receive",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/edit_tasks",
            "image",
            "editTasks.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/edit_tasks/{taskId}",
            "image",
            "editTasks.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/assets",
            "image",
            "assets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/assets/{assetId}",
            "image",
            "assets.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Patch,
            "/backend/v3/api/image/assets/{assetId}",
            "image",
            "assets.update",
        ),
        ImageHttpRoute::new(
            HttpMethod::Delete,
            "/backend/v3/api/image/assets/{assetId}",
            "image",
            "assets.delete",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/galleries",
            "image",
            "galleries.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/galleries",
            "image",
            "galleries.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/galleries/{galleryId}",
            "image",
            "galleries.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Patch,
            "/backend/v3/api/image/galleries/{galleryId}",
            "image",
            "galleries.update",
        ),
        ImageHttpRoute::new(
            HttpMethod::Delete,
            "/backend/v3/api/image/galleries/{galleryId}",
            "image",
            "galleries.delete",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/galleries/{galleryId}/items",
            "image",
            "galleries.items.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/galleries/{galleryId}/items",
            "image",
            "galleries.items.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Delete,
            "/backend/v3/api/image/galleries/{galleryId}/items/{itemId}",
            "image",
            "galleries.items.delete",
        ),
    ]
}

pub fn required_dual_token_headers() -> [&'static str; 2] {
    ["Authorization", "Access-Token"]
}
