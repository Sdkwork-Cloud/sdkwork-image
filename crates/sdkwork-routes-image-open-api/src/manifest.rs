pub const ROUTE_CRATE_PACKAGE: &str = "sdkwork-routes-image-open-api";
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

pub fn open_api_routes() -> Vec<ImageHttpRoute> {
    vec![ImageHttpRoute::new(
        HttpMethod::Post,
        "/image/v3/api/compat/openai/images/generations",
        "imageCompat",
        "compat.openai.images.generate",
    )]
}
