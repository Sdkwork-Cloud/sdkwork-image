# SDKWork Image
repository-kind: application

`sdkwork-image` owns SDKWork image generation, image editing, image galleries, image generation history/workspace UI, image-specific Rust storage/API contracts, and image SDK generation inputs.

Shared foundation, UI primitives, and core runtime remain dependencies. Image workspace packages, image route catalogs, image database contracts, image OpenAPI authorities, and image generated SDK families live here rather than in `sdkwork-appbase`.

## Repository Structure

```text
sdkwork-image/
  apis/                           # API contracts and OpenAPI sources
  apps/
    sdkwork-image-pc/             # PC browser/desktop application
    sdkwork-image-h5/             # H5/Capacitor mobile application
    sdkwork-image-flutter-mobile/ # Flutter mobile application
  crates/                         # Rust backend crates
  sdks/                           # SDK families and generation
  jobs/                           # Job definitions and schedules
  tools/                          # Developer and operator tools
  plugins/                        # Application plugins
  examples/                       # Runnable examples
  configs/                        # Config templates and schemas
  deployments/                    # Deployment descriptors
  scripts/                        # Build and release scripts
  docs/                           # Documentation and ADRs
  tests/                          # Cross-package tests
  apps/sdkwork-image-common/packages/         # Shared cross-architecture contracts
```

## Application Surfaces

### PC Application (`apps/sdkwork-image-pc/`)

PC browser/desktop application following `APP_PC_ARCHITECTURE_SPEC.md`.

Packages:
- `sdkwork-image-pc` - Image generation, editing, and galleries
- `sdkwork-image-pc-generation` - Generation history and task provenance

### H5 Application (`apps/sdkwork-image-h5/`)

H5/Capacitor mobile application following `APP_H5_ARCHITECTURE_SPEC.md`.

### Flutter Application (`apps/sdkwork-image-flutter-mobile/`)

Flutter mobile application following `FLUTTER_APP_MOBILE_ARCHITECTURE_SPEC.md`.

## Image Generation Runtime

The Rust image generation stack is split by ownership boundary:

- `crates/sdkwork-image-generation-service` owns image domain constants, generation command validation, provider dispatch planning, provider task/result normalization, AI generated Drive import planning, and Drive uploader command construction with `scene`.
- `crates/sdkwork-image-generation-workflow-service` owns service-level orchestration contracts for create, polling refresh, webhook refresh, Drive import planning, Drive upload preparation steps, outbox event planning, and the runtime step contract required for state consistency.
- `crates/sdkwork-image-generation-runtime-service` executes ClawRouter provider dispatch, prepares Drive uploader commands through `sdkwork-assets-ingestion-drive`, and runs live Drive import when `ImageDriveImportRuntime::try_from_env()` succeeds (`HttpProviderArtifactFetcher` fetches transient provider artifact URLs over HTTPS only).
- `crates/sdkwork-image-generation-host` owns the application host injected into gateway assembly: ClawRouter gateway wiring, generation service orchestration (create/refresh execute Drive import when enabled), SQL lifecycle side effects (provider task tracking, notification outbox enqueue), optional background processor (provider polling + webhook callback delivery), and SQLx-backed generation projection storage (`ImageGenerationHost::from_runtime_env`).
- `crates/sdkwork-image-generation-repository-sqlx` owns SQL migrations, repository SQL contracts, and `SqlxGenerationProjectionRepository` for create/list/get/refresh persistence.
- `crates/sdkwork-image-claw-router-provider-service` owns the Claw Router provider gateway (`clawrouter_open_sdk` only; no raw provider HTTP).
- `crates/sdkwork-routes-image-app-api` owns user-facing `/app/v3/api/image/*` HTTP handlers (generations, presets, assets, galleries, edit_tasks) with SdkWorkApiResponse envelopes, IAM runtime subject projection, and dual-token route manifest registration.
- `crates/sdkwork-routes-image-open-api` / `crates/sdkwork-routes-image-backend-api` owns the remaining image API route catalogs that materialize OpenAPI and SDK families, including backend provider webhook receive routes.
- `crates/sdkwork-image-gateway-assembly` merges app-api, backend-api, and open-api routers; production bootstrap uses `assemble_application_router_from_env()` (ClawRouter + IMAGE database + optional DRIVE database/object store via `ImageGenerationHost::from_runtime_env()`).

### Drive import runtime environment

When DRIVE database configuration is present and `IMAGE_DRIVE_IMPORT_ENABLED` is not disabled, successful image generations import provider artifacts into Drive `ai_generated` space immediately after create/refresh dispatch:

| Variable | Purpose |
| --- | --- |
| `DRIVE_*` | Drive database connection (same as `sdkwork-drive`) |
| `IMAGE_DRIVE_IMPORT_ENABLED` | `true` (default) / `false` to skip live import |
| `IMAGE_DRIVE_OBJECT_STORE_ROOT` | Local object store root (default `.data/image-drive-objects`) |
| `SDKWORK_CLAWROUTER_OPEN_API_BASE_URL` | ClawRouter provider dispatch |
| `IMAGE_*` | Image projection database |
| `IMAGE_BACKGROUND_PROCESSOR_ENABLED` | `true` (default) / `false` to disable provider poll + notification delivery loop |
| `IMAGE_BACKGROUND_POLL_INTERVAL_SECONDS` | Background loop interval (default `30`) |
| `IMAGE_PROVIDER_POLL_INTERVAL_SECONDS` | Per-task provider poll backoff (default `30`) |

Wire outputs persist `sync_status: imported`, `drive_node_id`, and `drive_uri` when import completes.

Image API paths use `/generations` resource names. Legacy `/generation_jobs`, `generationJobs`, and `{jobId}` names are intentionally excluded from the generated API surface.

Generated image and future media outputs are imported through `sdkwork-drive-workspace-service` into Drive `ai_generated` space for the current user or anonymous actor. Every Drive uploader command and persisted output plan carries the business `scene`.

Task and webhook providers converge through the same normalized provider result model. Polling refresh, webhook refresh, Drive import planning, Drive upload preparation, SQL repository updates, and notification outbox planning are explicit runtime steps so multi-output generations cannot diverge between provider state and Drive state.

Claw Router generated Rust SDK exposes `images.create_generation` for OpenAI-compatible image generation, including the OpenAI `n` output-count field. It also exposes task-based image providers through generated SDK resources such as `images_midjourney`, `images_nano_banana`, and `images_vidu`. `sdkwork-image` binds OpenAI `output_count` through the generated `n` field, maps Nano Banana and Vidu `referenceImages` into generated provider request models, rejects `referenceImages` for providers whose generated Claw Router request models do not expose a reference-image field, and normalizes multi-output or task-based provider responses before Drive import. Task-based dispatch plans and persistence snapshots record both the generated create SDK method and the generated retrieve SDK method used for polling, so runtime audit data can prove that `images_midjourney.list_v1_images_generations`, `images_nano_banana.retrieve_generations`, and `videos_vidu.list_ent_v2_tasks_creations` are the polling boundaries. The executable Claw Router gateway currently supports OpenAI-compatible image generation, Midjourney image tasks, Nano Banana image tasks, and Vidu reference-to-image tasks; other provider-native codes remain unsupported until their generated SDK image resources are present and mapped.

## SDKWork Documentation Contract

Domain: content
Capability: image-workspace
Package type: react-package
Status: standard

### Public API

Public exports are declared in `specs/component.spec.json` under `contracts.publicExports`.

### Required SDK Surface

- None declared in `specs/component.spec.json`.

### Configuration

Configuration keys and runtime entrypoints are declared in `specs/component.spec.json`.

### SaaS/Private/Local Behavior

This module follows the canonical standards linked from `specs/component.spec.json`, including deployment and runtime configuration rules where applicable.

### Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module.

### Extension Points

Extension points are limited to declared public exports, runtime entrypoints, SDK clients, events, and config keys.

### Verification

- `pnpm typecheck`

### Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)
