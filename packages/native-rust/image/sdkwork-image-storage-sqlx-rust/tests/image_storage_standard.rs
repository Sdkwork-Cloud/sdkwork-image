use sdkwork_image_storage_sqlx::{
    image_asset_tables, image_database_tables, image_gallery_tables, image_generation_tables,
    image_initial_migration_sql, image_runtime_migration_sql, image_storage_capability_manifest,
    IMAGE_INITIAL_MIGRATION, IMAGE_RUNTIME_MIGRATION,
};

#[test]
fn exposes_image_storage_table_catalog() {
    let tables = image_database_tables();

    for table in [
        "image_preset",
        "image_generation_job",
        "image_generation_output",
        "image_provider_binding",
        "image_provider_task",
        "image_provider_webhook_event",
        "image_notification_outbox",
        "image_edit_task",
        "image_asset",
        "image_gallery",
        "image_gallery_item",
    ] {
        assert!(tables.contains(&table), "missing image table: {table}");
    }

    for table in tables {
        assert!(
            table.starts_with("image_"),
            "image storage must only expose image-prefixed tables: {table}",
        );
        assert!(
            !table.starts_with("studio_") && !table.starts_with("plus_"),
            "image storage must not expose appbase/studio legacy tables: {table}",
        );
    }
}

#[test]
fn splits_generation_asset_and_gallery_tables() {
    assert_eq!(
        image_generation_tables(),
        vec![
            "image_preset",
            "image_generation_job",
            "image_generation_output",
            "image_provider_binding",
            "image_provider_task",
            "image_provider_webhook_event",
            "image_notification_outbox",
            "image_edit_task",
        ],
    );
    assert_eq!(image_asset_tables(), vec!["image_asset"]);
    assert_eq!(
        image_gallery_tables(),
        vec!["image_gallery", "image_gallery_item"]
    );
}

#[test]
fn runtime_migration_declares_provider_drive_and_multi_output_sync_tables() {
    let sql = image_runtime_migration_sql();

    for expected in [
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS scene VARCHAR(128)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_code VARCHAR(128)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_operation VARCHAR(128)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_task_id VARCHAR(256)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_status VARCHAR(128)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_state JSONB NOT NULL DEFAULT '{}'::jsonb",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS idempotency_key VARCHAR(128)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS callback_url TEXT",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS next_poll_at TIMESTAMPTZ",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS last_polled_at TIMESTAMPTZ",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS submitted_at TIMESTAMPTZ",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS import_started_at TIMESTAMPTZ",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS output_asset_count INTEGER NOT NULL DEFAULT 0",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS drive_space_id VARCHAR(128)",
        "ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS drive_sync_status VARCHAR(64) NOT NULL DEFAULT 'pending'",
        "CREATE TABLE IF NOT EXISTS image_generation_output",
        "media_kind VARCHAR(32) NOT NULL",
        "scene VARCHAR(128) NOT NULL",
        "provider_code VARCHAR(128) NOT NULL",
        "provider_asset_id VARCHAR(256)",
        "provider_uri TEXT",
        "provider_url TEXT",
        "drive_space_id VARCHAR(128)",
        "drive_node_id VARCHAR(128)",
        "drive_uri TEXT",
        "object_blob_id VARCHAR(128)",
        "resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb",
        "sync_status VARCHAR(64) NOT NULL DEFAULT 'pending'",
        "CREATE TABLE IF NOT EXISTS image_provider_binding",
        "CREATE TABLE IF NOT EXISTS image_provider_task",
        "CREATE TABLE IF NOT EXISTS image_provider_webhook_event",
        "CREATE TABLE IF NOT EXISTS image_notification_outbox",
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_image_generation_output_index",
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_image_generation_output_provider_asset",
        "CREATE INDEX IF NOT EXISTS idx_image_generation_output_drive_sync",
        "CREATE INDEX IF NOT EXISTS idx_image_generation_output_scene",
        "CREATE INDEX IF NOT EXISTS idx_image_provider_task_poll",
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_image_provider_webhook_event",
    ] {
        assert!(
            sql.contains(expected),
            "image runtime migration must contain `{expected}`",
        );
    }

    assert!(
        !sql.contains("asset_url")
            && !sql.contains("thumbnail_url")
            && !sql.contains("image_url")
            && !sql.contains("video_url"),
        "image runtime storage must not persist naked business media URL columns",
    );
}

#[test]
fn initial_migration_declares_standard_image_tables() {
    let sql = image_initial_migration_sql();

    for expected in [
        "CREATE TABLE IF NOT EXISTS image_preset",
        "CREATE TABLE IF NOT EXISTS image_generation_job",
        "CREATE TABLE IF NOT EXISTS image_edit_task",
        "CREATE TABLE IF NOT EXISTS image_asset",
        "CREATE TABLE IF NOT EXISTS image_gallery",
        "CREATE TABLE IF NOT EXISTS image_gallery_item",
        "prompt TEXT NOT NULL",
        "negative_prompt TEXT",
        "resolution VARCHAR(64) NOT NULL",
        "style VARCHAR(128) NOT NULL",
        "job_status INTEGER NOT NULL DEFAULT 1",
        "asset_media_resource_id VARCHAR(128)",
        "asset_object_blob_id BIGINT",
        "asset_resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb",
        "thumbnail_media_resource_id VARCHAR(128)",
        "thumbnail_resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb",
        "visibility INTEGER NOT NULL DEFAULT 1",
        "sort_order INTEGER NOT NULL DEFAULT 0",
    ] {
        assert!(
            sql.contains(expected),
            "image migration must contain `{expected}`",
        );
    }

    assert!(
        !sql.contains("asset_url")
            && !sql.contains("thumbnail_url")
            && !sql.contains("cover_image"),
        "image storage must use stable media references instead of naked URL columns",
    );
}

#[test]
fn image_tables_have_standard_context_columns_and_hot_path_indexes() {
    let initial_sql = image_initial_migration_sql();
    let runtime_sql = image_runtime_migration_sql();

    for table in [
        "image_preset",
        "image_generation_job",
        "image_edit_task",
        "image_asset",
        "image_gallery",
        "image_gallery_item",
    ] {
        assert_standard_context_columns(initial_sql, table);
    }

    for table in [
        "image_generation_output",
        "image_provider_binding",
        "image_provider_task",
        "image_provider_webhook_event",
        "image_notification_outbox",
    ] {
        assert_standard_context_columns(runtime_sql, table);
    }

    for expected in [
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_image_preset_key",
        "CREATE INDEX IF NOT EXISTS idx_image_generation_job_scope_status",
        "CREATE INDEX IF NOT EXISTS idx_image_generation_job_user",
        "CREATE INDEX IF NOT EXISTS idx_image_edit_task_source_asset",
        "CREATE INDEX IF NOT EXISTS idx_image_asset_job",
        "CREATE INDEX IF NOT EXISTS idx_image_asset_gallery",
        "CREATE INDEX IF NOT EXISTS idx_image_gallery_scope_status",
        "CREATE UNIQUE INDEX IF NOT EXISTS uk_image_gallery_item_asset",
    ] {
        assert!(
            initial_sql.contains(expected) || runtime_sql.contains(expected),
            "image migration must contain `{expected}`",
        );
    }
}

#[test]
fn manifest_declares_image_storage_contract() {
    let manifest = image_storage_capability_manifest();

    assert_eq!(manifest.name, "image-storage");
    assert_eq!(manifest.schema_version, "2026-06-06");
    assert_eq!(
        manifest.migrations,
        vec![IMAGE_INITIAL_MIGRATION, IMAGE_RUNTIME_MIGRATION],
    );
    assert_eq!(manifest.tables, image_database_tables());
    assert_eq!(manifest.generation_tables, image_generation_tables());
    assert_eq!(manifest.asset_tables, image_asset_tables());
    assert_eq!(manifest.gallery_tables, image_gallery_tables());
    assert!(manifest
        .repository_bindings
        .iter()
        .any(|binding| binding.repository_name == "ImageGenerationRepository"));
    assert!(manifest
        .repository_bindings
        .iter()
        .any(|binding| binding.repository_name == "ImageGalleryRepository"));
}

fn table_definition<'a>(sql: &'a str, table_name: &str) -> Option<&'a str> {
    let marker = format!("CREATE TABLE IF NOT EXISTS {table_name}");
    let start = sql.find(&marker)?;
    let after_start = &sql[start..];
    let end = after_start.find("\n);")?;
    Some(&after_start[..end])
}

fn assert_standard_context_columns(sql: &str, table: &str) {
    let definition = table_definition(sql, table).expect("table definition");
    for column in [
        "id BIGSERIAL PRIMARY KEY",
        "uuid VARCHAR(64) NOT NULL UNIQUE",
        "tenant_id BIGINT NOT NULL DEFAULT 0",
        "organization_id BIGINT NOT NULL DEFAULT 0",
        "data_scope INTEGER NOT NULL DEFAULT 0",
        "status INTEGER NOT NULL DEFAULT 1",
        "created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP",
        "updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP",
        "version BIGINT NOT NULL DEFAULT 0",
    ] {
        assert!(
            definition.contains(column),
            "{table} must contain standard column `{column}`",
        );
    }
}
