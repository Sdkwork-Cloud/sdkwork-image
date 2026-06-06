# SDKWork Image

`sdkwork-image` owns SDKWork image generation, image editing, image galleries, image generation history/workspace UI, image-specific Rust storage/API contracts, and image SDK generation inputs.

Shared foundation, UI primitives, and core runtime remain dependencies. Image workspace packages, image route catalogs, image database contracts, image OpenAPI authorities, and image generated SDK families live here rather than in `sdkwork-appbase`.

## Image Generation Runtime

The Rust image generation stack is split by ownership boundary:

- `packages/native-rust/image/sdkwork-image-core-rust` owns image domain constants, generation command validation, provider dispatch planning, provider task/result normalization, AI generated Drive import planning, and Drive uploader command construction with `scene`.
- `packages/native-rust/image/sdkwork-image-service-rust` owns service-level orchestration contracts for create, polling refresh, webhook refresh, Drive import planning, Drive upload preparation steps, outbox event planning, and the runtime step contract required for state consistency.
- `packages/native-rust/image/sdkwork-image-provider-claw-router-rust` owns the Claw Router provider gateway and calls `clawrouter_open_sdk` generated Rust SDK APIs. Product code must use this gateway or another approved generated-SDK adapter instead of raw provider HTTP.
- `packages/native-rust/image/sdkwork-image-storage-sqlx-rust` owns image database table, migration, and repository SQL contracts for generation jobs, multi-output records, provider tasks, webhook events, Drive sync status, and notification outbox.
- `packages/native-rust/image/sdkwork-image-http-rust` owns the image API route catalog that materializes OpenAPI and SDK families, including backend provider webhook receive routes.

Image API paths use `/generations` resource names. Legacy `/generation_jobs`, `generationJobs`, and `{jobId}` names are intentionally excluded from the generated API surface.

Generated image and future media outputs are imported into Drive `ai_generated` space for the current user or anonymous actor. Every Drive uploader command and persisted output plan carries the business `scene`.

Task and webhook providers converge through the same normalized provider result model. Polling refresh, webhook refresh, Drive import planning, Drive upload preparation, SQL repository updates, and notification outbox planning are explicit runtime steps so multi-output generations cannot diverge between provider state and Drive state.

Claw Router generated Rust SDK exposes `images.create_generation` for OpenAI-compatible image generation, including the OpenAI `n` output-count field. `sdkwork-image` binds `output_count` through that generated SDK field and normalizes multi-output provider responses before Drive import.
