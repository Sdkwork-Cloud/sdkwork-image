//! Image generation runtime orchestration: ClawRouter provider dispatch and Drive import execution.
//!
//! Planning remains in `sdkwork-image-generation-workflow-service`; this crate executes runtime steps
//! against the generated Claw Router SDK gateway and Drive uploader.

mod drive_import;
mod drive_runtime;
mod orchestration;
mod provider_fetch;
mod provider_fetch_http;

pub use drive_import::{
    build_drive_import_execution_context, execute_drive_import_uploads,
    plan_drive_upload_preparations, CompletedDriveImportArtifact, ImageDriveUploadPreparation,
};
pub use drive_runtime::ImageDriveImportRuntime;
pub use provider_fetch_http::HttpProviderArtifactFetcher;
pub use orchestration::{
    execute_create_generation_dispatch, execute_refresh_generation_dispatch,
    ImageGenerationCreateRuntimeInput, ImageGenerationCreateRuntimeResult,
    ImageGenerationRefreshRuntimeInput, ImageGenerationRefreshRuntimeResult,
};
pub use provider_fetch::{ProviderArtifactContent, ProviderArtifactFetcher, ProviderArtifactRef};

pub use sdkwork_image_generation_workflow_service::{
    dispatch_image_provider_via_claw_router, retrieve_image_provider_via_claw_router,
    ClawRouterDispatchError,
};

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ImageRuntimeError {
    #[error("image runtime validation failed: {0}")]
    Validation(&'static str),
    #[error("image runtime planning failed: {0}")]
    Planning(&'static str),
    #[error("claw router dispatch failed: {0}")]
    ClawRouter(#[from] ClawRouterDispatchError),
    #[error("drive import failed: {0}")]
    DriveImport(String),
    #[error("provider artifact fetch failed: {0}")]
    ProviderFetch(String),
}
