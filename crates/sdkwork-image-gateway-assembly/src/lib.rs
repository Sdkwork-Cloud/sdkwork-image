//! Gateway assembly for sdkwork-image.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
//! App-api generation routes require `Arc<ImageGenerationHost>` at assembly time.

mod bootstrap;
mod generated;

pub use bootstrap::{
    assemble_application_router, assemble_application_router_from_env, ApplicationAssembly,
};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
