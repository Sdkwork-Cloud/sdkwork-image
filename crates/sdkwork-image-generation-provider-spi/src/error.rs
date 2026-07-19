#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ImageGenerationProviderError {
    #[error("image generation request is invalid: {0}")]
    InvalidRequest(String),
    #[error("image generation vendor is unsupported: {0}")]
    UnsupportedVendor(String),
    #[error("image generation model is unsupported: {0}")]
    UnsupportedModel(String),
    #[error("image generation capability is unsupported: {0}")]
    UnsupportedCapability(String),
    #[error("image generation parameter is unsupported: {0}")]
    UnsupportedParameter(String),
    #[error("image generation provider is not configured: {0}")]
    ProviderNotConfigured(String),
    #[error("image generation provider is unavailable: {0}")]
    ProviderUnavailable(String),
    #[error("image generation provider rate limited the request: {0}")]
    RateLimited(String),
    #[error("image generation provider rejected the request: {0}")]
    Rejected(String),
    #[error("image generation provider timed out: {0}")]
    Timeout(String),
    #[error("image generation provider transport failed: {0}")]
    Transport(String),
    #[error("image generation provider returned an invalid response: {0}")]
    InvalidProviderResponse(String),
    #[error("image generation provider configuration is invalid: {0}")]
    Configuration(String),
}

impl ImageGenerationProviderError {
    pub fn retryable(&self) -> bool {
        matches!(
            self,
            Self::ProviderUnavailable(_)
                | Self::RateLimited(_)
                | Self::Timeout(_)
                | Self::Transport(_)
        )
    }
}

pub type ImageGenerationProviderResult<T> = Result<T, ImageGenerationProviderError>;
