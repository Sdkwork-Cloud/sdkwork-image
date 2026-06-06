pub const IMAGE_INITIAL_MIGRATION: &str = "0001_image_foundation.sql";

const IMAGE_INITIAL_MIGRATION_SQL: &str =
    include_str!("../migrations/0001_image_foundation.sql");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageRepositoryBinding {
    pub domain: &'static str,
    pub repository_name: &'static str,
    pub tables: Vec<&'static str>,
    pub requires_transaction: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageStorageCapabilityManifest {
    pub name: &'static str,
    pub schema_version: &'static str,
    pub tables: Vec<&'static str>,
    pub generation_tables: Vec<&'static str>,
    pub asset_tables: Vec<&'static str>,
    pub gallery_tables: Vec<&'static str>,
    pub migrations: Vec<&'static str>,
    pub repository_bindings: Vec<ImageRepositoryBinding>,
}

pub fn image_generation_tables() -> Vec<&'static str> {
    vec!["image_preset", "image_generation_job", "image_edit_task"]
}

pub fn image_asset_tables() -> Vec<&'static str> {
    vec!["image_asset"]
}

pub fn image_gallery_tables() -> Vec<&'static str> {
    vec!["image_gallery", "image_gallery_item"]
}

pub fn image_database_tables() -> Vec<&'static str> {
    let mut tables = image_generation_tables();
    tables.extend(image_asset_tables());
    tables.extend(image_gallery_tables());
    tables
}

pub fn image_initial_migration_sql() -> &'static str {
    IMAGE_INITIAL_MIGRATION_SQL
}

pub fn image_storage_capability_manifest() -> ImageStorageCapabilityManifest {
    ImageStorageCapabilityManifest {
        name: "image-storage",
        schema_version: "2026-06-06",
        tables: image_database_tables(),
        generation_tables: image_generation_tables(),
        asset_tables: image_asset_tables(),
        gallery_tables: image_gallery_tables(),
        migrations: vec![IMAGE_INITIAL_MIGRATION],
        repository_bindings: vec![
            ImageRepositoryBinding {
                domain: "image",
                repository_name: "ImageGenerationRepository",
                tables: image_generation_tables(),
                requires_transaction: true,
            },
            ImageRepositoryBinding {
                domain: "image",
                repository_name: "ImageAssetRepository",
                tables: image_asset_tables(),
                requires_transaction: true,
            },
            ImageRepositoryBinding {
                domain: "image",
                repository_name: "ImageGalleryRepository",
                tables: image_gallery_tables(),
                requires_transaction: true,
            },
        ],
    }
}
