pub const IMAGE_WORKSPACE: &str = "sdkwork-image";
pub const IMAGE_DOMAIN: &str = "image";
pub const IMAGE_CAPABILITY: &str = "image";

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageJobStatus {
    Queued = 1,
    Rendering = 2,
    Ready = 3,
    Failed = 4,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageVisibility {
    Private = 1,
    Tenant = 2,
    Public = 3,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageGenerationRequest {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub prompt: String,
    pub resolution: String,
    pub style: String,
    pub visibility: ImageVisibility,
    pub status: ImageJobStatus,
}

pub fn create_image_generation_request(
    tenant_id: impl Into<String>,
    organization_id: Option<&str>,
    prompt: impl Into<String>,
    resolution: impl Into<String>,
    style: impl Into<String>,
    visibility: ImageVisibility,
) -> ImageGenerationRequest {
    ImageGenerationRequest {
        tenant_id: tenant_id.into(),
        organization_id: organization_id.map(str::to_string),
        prompt: prompt.into(),
        resolution: resolution.into(),
        style: style.into(),
        visibility,
        status: ImageJobStatus::Queued,
    }
}

pub fn validate_image_generation_request(
    request: &ImageGenerationRequest,
) -> Result<(), &'static str> {
    if request.tenant_id.trim().is_empty() {
        return Err("image generation tenant is required");
    }

    if request.prompt.trim().is_empty() {
        return Err("image generation prompt is required");
    }

    if request.style.trim().is_empty() {
        return Err("image generation style is required");
    }

    let Some((width, height)) = request.resolution.split_once('x') else {
        return Err("image generation resolution must use WIDTHxHEIGHT");
    };

    if parse_positive_dimension(width).is_none() || parse_positive_dimension(height).is_none() {
        return Err("image generation resolution must use WIDTHxHEIGHT");
    }

    Ok(())
}

fn parse_positive_dimension(value: &str) -> Option<u32> {
    let dimension = value.trim().parse::<u32>().ok()?;
    (dimension > 0).then_some(dimension)
}
