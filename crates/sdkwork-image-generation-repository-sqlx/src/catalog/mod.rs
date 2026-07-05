mod memory;
mod sqlx_store;
mod wire;

pub use memory::InMemoryImageCatalogRepository;
pub use sqlx_store::SqlxImageCatalogRepository;
pub use wire::{
    ImageAssetRecord, ImageEditTaskRecord, ImageGalleryItemRecord, ImageGalleryRecord,
    ImagePresetRecord,
};

use async_trait::async_trait;

use crate::RepositoryError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageCatalogScope {
    pub tenant_id: String,
    pub organization_id: Option<String>,
    pub user_id: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageEditTaskCreateCommand {
    pub source_asset_id: String,
    pub edit_type: String,
    pub prompt: String,
    pub negative_prompt: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageGalleryItemCreateCommand {
    pub asset_id: String,
    pub caption: Option<String>,
    pub sort_order: Option<i32>,
}

#[async_trait]
pub trait ImageCatalogRepository: Send + Sync {
    async fn list_presets(
        &self,
        scope: &ImageCatalogScope,
        limit: i64,
        offset: i64,
        q: Option<&str>,
    ) -> Result<Vec<ImagePresetRecord>, RepositoryError>;

    async fn get_preset(
        &self,
        scope: &ImageCatalogScope,
        preset_id: &str,
    ) -> Result<Option<ImagePresetRecord>, RepositoryError>;

    async fn list_assets(
        &self,
        scope: &ImageCatalogScope,
        limit: i64,
        offset: i64,
        q: Option<&str>,
    ) -> Result<Vec<ImageAssetRecord>, RepositoryError>;

    async fn get_asset(
        &self,
        scope: &ImageCatalogScope,
        asset_id: &str,
    ) -> Result<Option<ImageAssetRecord>, RepositoryError>;

    async fn list_galleries(
        &self,
        scope: &ImageCatalogScope,
        limit: i64,
        offset: i64,
        q: Option<&str>,
    ) -> Result<Vec<ImageGalleryRecord>, RepositoryError>;

    async fn get_gallery(
        &self,
        scope: &ImageCatalogScope,
        gallery_id: &str,
    ) -> Result<Option<ImageGalleryRecord>, RepositoryError>;

    async fn create_gallery_item(
        &self,
        scope: &ImageCatalogScope,
        gallery_id: &str,
        command: ImageGalleryItemCreateCommand,
    ) -> Result<ImageGalleryItemRecord, RepositoryError>;

    async fn create_edit_task(
        &self,
        scope: &ImageCatalogScope,
        command: ImageEditTaskCreateCommand,
    ) -> Result<ImageEditTaskRecord, RepositoryError>;

    async fn get_edit_task(
        &self,
        scope: &ImageCatalogScope,
        task_id: &str,
    ) -> Result<Option<ImageEditTaskRecord>, RepositoryError>;
}
