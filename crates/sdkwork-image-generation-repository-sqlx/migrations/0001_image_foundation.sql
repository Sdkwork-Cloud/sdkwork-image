CREATE TABLE IF NOT EXISTS image_preset (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    preset_key VARCHAR(128) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    prompt_template TEXT,
    negative_prompt_template TEXT,
    default_resolution VARCHAR(64) NOT NULL,
    default_style VARCHAR(128) NOT NULL,
    default_visibility INTEGER NOT NULL DEFAULT 1,
    tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    published_at TIMESTAMPTZ,
    deprecated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_preset_key ON image_preset (tenant_id, organization_id, preset_key);
CREATE INDEX IF NOT EXISTS idx_image_preset_scope_status ON image_preset (tenant_id, organization_id, status, updated_at, id);

CREATE TABLE IF NOT EXISTS image_generation_job (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    user_id BIGINT,
    request_id VARCHAR(128),
    trace_id VARCHAR(128),
    preset_id BIGINT,
    prompt TEXT NOT NULL,
    negative_prompt TEXT,
    resolution VARCHAR(64) NOT NULL,
    style VARCHAR(128) NOT NULL,
    model_id VARCHAR(128),
    provider_id VARCHAR(128),
    job_status INTEGER NOT NULL DEFAULT 1,
    visibility INTEGER NOT NULL DEFAULT 1,
    input_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    output_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    error_code VARCHAR(128),
    error_message TEXT,
    queued_at TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_image_generation_job_scope_status ON image_generation_job (tenant_id, organization_id, job_status, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_job_user ON image_generation_job (tenant_id, organization_id, user_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_job_preset ON image_generation_job (tenant_id, organization_id, preset_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_job_request ON image_generation_job (tenant_id, organization_id, request_id);

CREATE TABLE IF NOT EXISTS image_edit_task (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    user_id BIGINT,
    request_id VARCHAR(128),
    trace_id VARCHAR(128),
    source_asset_id BIGINT NOT NULL,
    generation_job_id BIGINT,
    edit_type VARCHAR(64) NOT NULL,
    prompt TEXT NOT NULL,
    negative_prompt TEXT,
    mask_media_resource_id VARCHAR(128),
    mask_object_blob_id BIGINT,
    mask_resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    job_status INTEGER NOT NULL DEFAULT 1,
    input_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    output_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    error_code VARCHAR(128),
    error_message TEXT,
    started_at TIMESTAMPTZ,
    finished_at TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_image_edit_task_source_asset ON image_edit_task (tenant_id, organization_id, source_asset_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_edit_task_scope_status ON image_edit_task (tenant_id, organization_id, job_status, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_edit_task_user ON image_edit_task (tenant_id, organization_id, user_id, updated_at, id);

CREATE TABLE IF NOT EXISTS image_asset (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    user_id BIGINT,
    generation_job_id BIGINT,
    edit_task_id BIGINT,
    gallery_id BIGINT,
    title VARCHAR(255),
    prompt TEXT,
    negative_prompt TEXT,
    style VARCHAR(128),
    resolution VARCHAR(64),
    mime_type VARCHAR(128),
    width INTEGER,
    height INTEGER,
    file_size BIGINT,
    asset_media_resource_id VARCHAR(128),
    asset_object_blob_id BIGINT,
    asset_resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    thumbnail_media_resource_id VARCHAR(128),
    thumbnail_object_blob_id BIGINT,
    thumbnail_resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    visibility INTEGER NOT NULL DEFAULT 1,
    provenance VARCHAR(64) NOT NULL DEFAULT 'generated',
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    published_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_image_asset_job ON image_asset (tenant_id, organization_id, generation_job_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_asset_edit_task ON image_asset (tenant_id, organization_id, edit_task_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_asset_gallery ON image_asset (tenant_id, organization_id, gallery_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_asset_scope_status ON image_asset (tenant_id, organization_id, visibility, status, updated_at, id);

CREATE TABLE IF NOT EXISTS image_gallery (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    owner_user_id BIGINT,
    gallery_key VARCHAR(128) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    visibility INTEGER NOT NULL DEFAULT 1,
    cover_asset_id BIGINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    published_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_gallery_key ON image_gallery (tenant_id, organization_id, gallery_key);
CREATE INDEX IF NOT EXISTS idx_image_gallery_scope_status ON image_gallery (tenant_id, organization_id, visibility, status, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_gallery_owner ON image_gallery (tenant_id, organization_id, owner_user_id, updated_at, id);

CREATE TABLE IF NOT EXISTS image_gallery_item (
    id BIGSERIAL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL UNIQUE,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    gallery_id BIGINT NOT NULL,
    asset_id BIGINT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    caption VARCHAR(500),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_gallery_item_asset ON image_gallery_item (tenant_id, organization_id, gallery_id, asset_id);
CREATE INDEX IF NOT EXISTS idx_image_gallery_item_gallery ON image_gallery_item (tenant_id, organization_id, gallery_id, sort_order, id);
