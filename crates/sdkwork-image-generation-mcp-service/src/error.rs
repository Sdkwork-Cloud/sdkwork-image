use schemars::JsonSchema;
use sdkwork_image_generation_service::ImageGenerationProviderError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpToolError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl McpToolError {
    pub(crate) fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: "invalid_request".to_string(),
            message: message.into(),
            retryable: false,
        }
    }

    pub(crate) fn task_not_found(handle: &str) -> Self {
        Self {
            code: "task_not_found".to_string(),
            message: format!("image generation task handle was not found: {handle}"),
            retryable: false,
        }
    }
}

impl From<ImageGenerationProviderError> for McpToolError {
    fn from(error: ImageGenerationProviderError) -> Self {
        let code = match &error {
            ImageGenerationProviderError::InvalidRequest(_) => "invalid_request",
            ImageGenerationProviderError::UnsupportedVendor(_) => "unsupported_vendor",
            ImageGenerationProviderError::UnsupportedModel(_) => "unsupported_model",
            ImageGenerationProviderError::UnsupportedCapability(_) => "unsupported_capability",
            ImageGenerationProviderError::UnsupportedParameter(_) => "unsupported_parameter",
            ImageGenerationProviderError::ProviderNotConfigured(_) => "provider_not_configured",
            ImageGenerationProviderError::ProviderUnavailable(_) => "provider_unavailable",
            ImageGenerationProviderError::RateLimited(_) => "rate_limited",
            ImageGenerationProviderError::Rejected(_) => "rejected",
            ImageGenerationProviderError::Timeout(_) => "timeout",
            ImageGenerationProviderError::Transport(_) => "transport",
            ImageGenerationProviderError::InvalidProviderResponse(_) => "invalid_provider_response",
            ImageGenerationProviderError::Configuration(_) => "configuration",
        };
        Self {
            code: code.to_string(),
            message: error.to_string(),
            retryable: error.retryable(),
        }
    }
}
