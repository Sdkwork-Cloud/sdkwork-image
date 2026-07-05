# sdkwork-routes-image-app-api

Image app-api route crate for `/app/v3/api/image/*`.

## Responsibilities

- Route catalog (`manifest.rs`) aligned with OpenAPI authority
- Dual-token HTTP route manifest for web-framework auth/rate-limit resolution
- Generation handlers: create, list, retrieve, refresh, cancel (`routes.rs`)
- Catalog handlers: presets, assets, galleries, edit_tasks (`routes.rs`)
- SdkWorkApiResponse / ProblemDetail mapping via `sdkwork-utils-rust` (`api_response.rs`)
- IAM runtime subject projection from `IamAppContext` (`subject.rs`)

## Bootstrap

Gateway assembly injects `Arc<ImageGenerationHost>` and optionally starts the background processor:

```rust
// Production: ClawRouter + IMAGE database + optional DRIVE import + background processor
let assembly = assemble_application_router_from_env().await?;
// assembly.background_processor holds the tokio task when IMAGE_BACKGROUND_PROCESSOR_ENABLED (default true)

// Tests / custom wiring
let host = ImageGenerationHost::for_test(gateway);
let assembly = assemble_application_router(host).await;
```

Handlers are mounted through `gateway_mount(host).await`, which wraps the router with IAM/web-framework layers from environment.

Machine-readable contract: `specs/component.spec.json`. Standards: `../../../../sdkwork-specs/`.
