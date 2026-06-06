pub const APP_API_PREFIX: &str = "/app/v3/api";
pub const BACKEND_API_PREFIX: &str = "/backend/v3/api";
pub const OPEN_API_PREFIX: &str = "/image/v3/api";

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

pub fn app_routes() -> Vec<ImageHttpRoute> {
    vec![
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/presets",
            "image",
            "presets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/presets/{presetId}",
            "image",
            "presets.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/app/v3/api/image/generation_jobs",
            "image",
            "generationJobs.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/generation_jobs",
            "image",
            "generationJobs.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/generation_jobs/{jobId}",
            "image",
            "generationJobs.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/app/v3/api/image/edit_tasks",
            "image",
            "editTasks.create",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/edit_tasks/{taskId}",
            "image",
            "editTasks.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/assets",
            "image",
            "assets.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/assets/{assetId}",
            "image",
            "assets.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/galleries",
            "image",
            "galleries.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/app/v3/api/image/galleries/{galleryId}",
            "image",
            "galleries.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/app/v3/api/image/galleries/{galleryId}/items",
            "image",
            "galleries.items.create",
        ),
    ]
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
            "/backend/v3/api/image/generation_jobs",
            "image",
            "generationJobs.list",
        ),
        ImageHttpRoute::new(
            HttpMethod::Get,
            "/backend/v3/api/image/generation_jobs/{jobId}",
            "image",
            "generationJobs.retrieve",
        ),
        ImageHttpRoute::new(
            HttpMethod::Post,
            "/backend/v3/api/image/generation_jobs/{jobId}/cancel",
            "image",
            "generationJobs.cancel",
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

pub fn open_api_routes() -> Vec<ImageHttpRoute> {
    vec![ImageHttpRoute::new(
        HttpMethod::Post,
        "/image/v3/api/compat/openai/images/generations",
        "imageCompat",
        "compat.openai.images.generate",
    )]
}

pub fn required_dual_token_headers() -> [&'static str; 2] {
    ["Authorization", "Access-Token"]
}
