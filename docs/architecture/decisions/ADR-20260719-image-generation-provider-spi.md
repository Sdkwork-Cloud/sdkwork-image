# ADR-20260719: Image Generation Provider SPI

Status: accepted
Date: 2026-07-19
Owner: SDKWork Image maintainers

## Context

Image generation used a generated unified Rust SDK for multiple vendors, but generated SDK route,
resource, method, and DTO details were present in domain dispatch plans, workflow snapshots, runtime
functions, and host services. The SDK dependency is valid; exposing its transport vocabulary across
business layers is not.

## Decision

Introduce a transport-neutral `sdkwork-image-generation-provider-spi`, make
`sdkwork-image-generation-service` the registry-backed consumer entrypoint, and place all generated
SDK routing and DTO conversion in `sdkwork-image-generation-provider-adapter`. The host constructs
the generated SDK client and injects the adapter through the SPI.

`vendor` identifies the requested model owner or compatibility family. `provider_id` identifies the
registered execution implementation. The default provider implementation uses the unified generated
SDK, but its dependency identity is not exposed as a vendor.

Common parameters are strongly typed. Vendor-only parameters use a versioned schema plus JSON value;
the adapter validates the schema and deserializes it into a vendor-specific private type before
constructing generated SDK DTOs.

## Consequences

- Other Rust modules depend on `sdkwork-image-generation-service` or its service port.
- Additional provider implementations can be registered without changing workflow or routes.
- Generated SDK resources and methods are adapter test evidence, not persistent domain state.
- Pre-release consumers use the canonical service, SPI, and adapter packages directly; no provider
  compatibility package is retained.
- Cancellation or other unsupported capabilities fail explicitly; there is no silent vendor/model
  fallback or raw HTTP fallback.
