# sdkwork_image_claw_router_provider_service

This README is the SDKWork module entrypoint for `sdkwork_image_claw_router_provider_service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../../sdkwork-specs/`.

This crate owns the Claw Router provider boundary and calls generated `clawrouter_open_sdk` Rust APIs. Product code must use this service boundary instead of raw provider HTTP.
