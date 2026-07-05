use async_trait::async_trait;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderArtifactRef {
    pub output_index: i32,
    pub provider_url: Option<String>,
    pub provider_uri: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderArtifactContent {
    pub output_index: i32,
    pub body: Vec<u8>,
    pub content_type: String,
    pub original_file_name: String,
}

#[async_trait]
pub trait ProviderArtifactFetcher: Send + Sync {
    async fn fetch_provider_artifact(
        &self,
        artifact: &ProviderArtifactRef,
    ) -> Result<ProviderArtifactContent, String>;
}
