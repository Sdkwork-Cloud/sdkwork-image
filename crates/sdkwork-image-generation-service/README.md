# sdkwork_image_generation_service

This README is the SDKWork module entrypoint for `sdkwork_image_generation_service`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../../sdkwork-specs/`.

This crate owns the injected `ImageGenerationServicePort` and its default registry-backed service,
re-exports the stable provider SPI contracts, and retains image/Drive domain planning. Consumers use
the crate root and do not depend on generated SDK DTOs or a concrete provider adapter.
