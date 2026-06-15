# sdkwork_image_generation_workflow_service

This README is the SDKWork module entrypoint for `sdkwork_image_generation_workflow_service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../../sdkwork-specs/`.

This crate owns image generation service-level orchestration contracts for create, polling refresh, webhook refresh, Drive import planning, Drive upload preparation steps, outbox event planning, and the runtime step contract required for state consistency.
