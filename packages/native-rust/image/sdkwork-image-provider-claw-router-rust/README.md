# sdkwork_image_provider_claw_router

Domain: content
Capability: image-provider-claw-router
Package type: rust-crate
Status: standard

This README is the SDKWork module entrypoint for `sdkwork_image_provider_claw_router`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../../../sdkwork-specs/`.

## Public API

- `.`

## Required SDK Surface

- Generated Rust SDK crate: `clawrouter-open-sdk`.
- Generated client resources used by this adapter: `images.create_generation`, `images_midjourney.create_v1_images_generation`, `images_midjourney.list_v1_images_generations`, `images_nano_banana.create_generations`, `images_nano_banana.retrieve_generations`, `images_vidu.create_ent_v2_reference2image`, and `videos_vidu.list_ent_v2_tasks_creations`.
- Task-based generation plans persist the generated create method and the generated retrieve method separately so polling can be audited without raw HTTP or provider-local route strings.
- Supported executable provider operations: OpenAI-compatible image generation, Midjourney image generation tasks, Nano Banana image generation tasks including optional reference images, and Vidu reference-to-image generation tasks.
- `referenceImages` is executable only for generated Claw Router request models that expose a reference-image field. The core dispatch layer rejects reference images for OpenAI-compatible and Midjourney plans so this adapter does not silently drop caller input.
- Provider-native codes without an explicit generated Claw Router image create resource are rejected before dispatch planning until the generated SDK resource exists and is mapped here.
- Product code must use this adapter or another approved generated-SDK adapter instead of raw Claw Router HTTP.

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in `specs/component.spec.json`. Shared modules must receive configuration through typed bootstrap or service boundaries rather than reading host-local environment state directly.

## SaaS/Private/Local Behavior

This component follows the deployment and runtime rules referenced by its `canonicalSpecs` entries. SaaS, private, and local behavior must stay compatible with the relevant SDKWork specs before implementation changes are made.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
