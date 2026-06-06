ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS scene VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_code VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_operation VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_task_id VARCHAR(256);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_status VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS provider_state JSONB NOT NULL DEFAULT '{}'::jsonb;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS idempotency_key VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS callback_url TEXT;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS next_poll_at TIMESTAMPTZ;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS last_polled_at TIMESTAMPTZ;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS submitted_at TIMESTAMPTZ;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS import_started_at TIMESTAMPTZ;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS output_asset_count INTEGER NOT NULL DEFAULT 0;
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS drive_space_id VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS drive_parent_node_id VARCHAR(128);
ALTER TABLE image_generation_job ADD COLUMN IF NOT EXISTS drive_sync_status VARCHAR(64) NOT NULL DEFAULT 'pending';

CREATE INDEX IF NOT EXISTS idx_image_generation_job_provider_task
    ON image_generation_job (tenant_id, organization_id, provider_code, provider_task_id);
CREATE INDEX IF NOT EXISTS idx_image_generation_job_poll
    ON image_generation_job (tenant_id, organization_id, job_status, next_poll_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_job_drive_sync
    ON image_generation_job (tenant_id, organization_id, drive_sync_status, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_job_scene
    ON image_generation_job (tenant_id, organization_id, scene, updated_at, id);

CREATE TABLE IF NOT EXISTS image_generation_output (
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
    generation_job_id BIGINT NOT NULL,
    generation_uuid VARCHAR(64),
    output_index INTEGER NOT NULL,
    media_kind VARCHAR(32) NOT NULL,
    scene VARCHAR(128) NOT NULL,
    provider_code VARCHAR(128) NOT NULL,
    provider_operation VARCHAR(128),
    provider_task_id VARCHAR(256),
    provider_asset_id VARCHAR(256),
    provider_uri TEXT,
    provider_url TEXT,
    provider_payload_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    drive_space_type VARCHAR(64) NOT NULL DEFAULT 'ai_generated',
    drive_space_id VARCHAR(128),
    drive_parent_node_id VARCHAR(128),
    drive_node_id VARCHAR(128),
    drive_uri TEXT,
    object_blob_id VARCHAR(128),
    resource_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    resource_hash VARCHAR(128),
    file_name VARCHAR(512),
    mime_type VARCHAR(256),
    size_bytes BIGINT,
    width INTEGER,
    height INTEGER,
    duration_seconds NUMERIC(18, 6),
    checksum_sha256_hex VARCHAR(128),
    sync_status VARCHAR(64) NOT NULL DEFAULT 'pending',
    import_attempts INTEGER NOT NULL DEFAULT 0,
    next_retry_at TIMESTAMPTZ,
    imported_at TIMESTAMPTZ,
    error_code VARCHAR(128),
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_generation_output_index
    ON image_generation_output (tenant_id, organization_id, generation_job_id, output_index);
CREATE UNIQUE INDEX IF NOT EXISTS uk_image_generation_output_provider_asset
    ON image_generation_output (provider_code, provider_asset_id)
    WHERE provider_asset_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_image_generation_output_job
    ON image_generation_output (tenant_id, organization_id, generation_job_id, output_index);
CREATE INDEX IF NOT EXISTS idx_image_generation_output_drive_sync
    ON image_generation_output (tenant_id, organization_id, sync_status, next_retry_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_output_scene
    ON image_generation_output (tenant_id, organization_id, scene, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_generation_output_drive_node
    ON image_generation_output (tenant_id, organization_id, drive_space_id, drive_node_id);

CREATE TABLE IF NOT EXISTS image_provider_binding (
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
    provider_code VARCHAR(128) NOT NULL,
    provider_display_name VARCHAR(255) NOT NULL,
    capability_code VARCHAR(128) NOT NULL,
    route_profile VARCHAR(128) NOT NULL,
    claw_router_provider_code VARCHAR(128),
    task_mode VARCHAR(64) NOT NULL DEFAULT 'sync',
    webhook_enabled BOOLEAN NOT NULL DEFAULT FALSE,
    polling_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    priority INTEGER NOT NULL DEFAULT 100,
    config_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_provider_binding_capability
    ON image_provider_binding (tenant_id, organization_id, provider_code, capability_code);
CREATE INDEX IF NOT EXISTS idx_image_provider_binding_status
    ON image_provider_binding (tenant_id, organization_id, capability_code, status, priority, id);

CREATE TABLE IF NOT EXISTS image_provider_task (
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
    generation_job_id BIGINT NOT NULL,
    provider_code VARCHAR(128) NOT NULL,
    provider_operation VARCHAR(128) NOT NULL,
    provider_task_id VARCHAR(256),
    provider_request_id VARCHAR(256),
    provider_status VARCHAR(128),
    dispatch_status VARCHAR(64) NOT NULL DEFAULT 'pending',
    callback_url TEXT,
    request_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    response_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    last_event_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    poll_attempts INTEGER NOT NULL DEFAULT 0,
    next_poll_at TIMESTAMPTZ,
    last_polled_at TIMESTAMPTZ,
    submitted_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    error_code VARCHAR(128),
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_provider_task_provider
    ON image_provider_task (provider_code, provider_task_id)
    WHERE provider_task_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_image_provider_task_job
    ON image_provider_task (tenant_id, organization_id, generation_job_id, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_image_provider_task_poll
    ON image_provider_task (tenant_id, organization_id, dispatch_status, next_poll_at, id);

CREATE TABLE IF NOT EXISTS image_provider_webhook_event (
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
    provider_code VARCHAR(128) NOT NULL,
    provider_task_id VARCHAR(256),
    external_event_id VARCHAR(256),
    event_type VARCHAR(128) NOT NULL,
    payload_hash VARCHAR(128) NOT NULL,
    signature_valid BOOLEAN,
    payload_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    process_status VARCHAR(64) NOT NULL DEFAULT 'pending',
    processed_at TIMESTAMPTZ,
    error_code VARCHAR(128),
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_image_provider_webhook_event
    ON image_provider_webhook_event (provider_code, payload_hash);
CREATE INDEX IF NOT EXISTS idx_image_provider_webhook_event_task
    ON image_provider_webhook_event (tenant_id, organization_id, provider_code, provider_task_id, created_at, id);
CREATE INDEX IF NOT EXISTS idx_image_provider_webhook_event_process
    ON image_provider_webhook_event (tenant_id, organization_id, process_status, created_at, id);

CREATE TABLE IF NOT EXISTS image_notification_outbox (
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
    aggregate_type VARCHAR(128) NOT NULL,
    aggregate_id VARCHAR(128) NOT NULL,
    event_type VARCHAR(128) NOT NULL,
    payload_snapshot JSONB NOT NULL DEFAULT '{}'::jsonb,
    delivery_status VARCHAR(64) NOT NULL DEFAULT 'pending',
    delivery_attempts INTEGER NOT NULL DEFAULT 0,
    next_delivery_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    error_code VARCHAR(128),
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_image_notification_outbox_delivery
    ON image_notification_outbox (tenant_id, organization_id, delivery_status, next_delivery_at, id);
CREATE INDEX IF NOT EXISTS idx_image_notification_outbox_aggregate
    ON image_notification_outbox (aggregate_type, aggregate_id, created_at, id);
